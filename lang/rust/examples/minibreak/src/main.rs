// Very difficult "breakout" clone
// Sample contributed by Andi McClure, available under Creative Commons Zero (public domain)
// If you substantially reuse this code a credit would be appreciated but is not required

#![no_std]
#![no_main]
#![allow(unused_parens)]

use core::panic::PanicInfo;
use core::slice::from_raw_parts_mut;

extern crate alloc;

use embedded_alloc::Heap;
use litex_hal as hal;
use litex_pac as pac;
use litex_openfpga::*;
use riscv_rt::entry;

// Definition is required for uart_printer.rs to work
hal::uart! {
    UART: pac::UART,
}

#[repr(u16)]
#[allow(dead_code)]
enum PocketControls {
    DpadUp     = 1<<0,
    DpadDown   = 1<<1,
    DpadLeft   = 1<<2,
    DpadRight  = 1<<3,
    FaceA      = 1<<4,
    FaceB      = 1<<5,
    FaceX      = 1<<6,
    FaceY      = 1<<7,
    TrigL1     = 1<<8,
    TrigR1     = 1<<9,
    TrigL2     = 1<<10,
    TrigR2     = 1<<11,
    TrigL3     = 1<<12,
    TrigR3     = 1<<13,
    FaceSelect = 1<<14,
    FaceStart  = 1<<15,
}

// const TEST_ADDR: *mut u32 = (0xF0001800 + 0x0028) as *mut u32;

// Fix for missing main functions
#[no_mangle]
fn fminf(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[no_mangle]
fn fmaxf(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

use core::mem::MaybeUninit;

const HEAP_SIZE: usize = 200 * 1024;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic:");
    println!("{info}");

    loop {}
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

const DISPLAY_WIDTH: usize = 266;
const DISPLAY_HEIGHT: usize = 240;

const READ_LENGTH: usize = 0x10000;

fn render_init(framebuffer_address: *mut u16) {
    let framebuffer = unsafe { from_raw_parts_mut(framebuffer_address, DISPLAY_WIDTH * DISPLAY_HEIGHT) };

    const PIXEL_MAX:usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
    for idx in 0..PIXEL_MAX {
        framebuffer[idx] = 0xFFFF;
    }
}

fn pixel(framebuffer_address: *mut u16, x: usize, y: usize) -> &'static mut u16 {
    let framebuffer = unsafe { from_raw_parts_mut(framebuffer_address as *mut u16, READ_LENGTH) };

    &mut framebuffer[y * DISPLAY_WIDTH + x]
}

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    println!("-- Minibreak --");

    // Framebuffer pointer
    let fb:*mut u16 = peripherals.VIDEO_FRAMEBUFFER.dma_base.read().bits() as *mut u16;

    render_init(fb);

    // "APP"
    {
        use glam::IVec2;
        use alloc::vec::Vec;

        // Config
        let config_chaos = 0; // 0-2 inclusive
        const CONFIG_IMMORTAL:bool = false;
        const LFO_MAX:u16 = 48000;

        // Basic
        let mut wave:u16 = 0;
        let mut lfo:u16 = LFO_MAX/4;
        const FREQ_DELTA:u16 = 150;
        const AUDIO_TARGET:i32 = 48000/60 + 200; // Try to fill audio buffer to this point
//        let mut first_frame = true;

        let mut paused = false;
        let mut dead = false;
        let mut won = false;
        let mut cont1_key_last = 0;
        let mut bleep_high = false;
        let mut bleeping = 0;
        let mut blooping = 0;
        let mut final_vader_facing = 0;

        cfg_if::cfg_if! {
            if #[cfg(feature = "speed-debug")] {
                const SPEED_DEBUG_RATE:u32 = 1; // Every frame
                let mut frame_already_overdue:bool = false;
                let mut video_frame_counter_last:Option<u32> = None;
                let mut missed_deadline_count:u32 = 0;
                let mut missed_deadline_already = false;
            }
        }

        // Geometry support: 0,0 is top left
        #[allow(dead_code)] // To remove if code changes
        fn ivec2_within(size:IVec2, at:IVec2) -> bool {
            IVec2::ZERO.cmple(at).all() && size.cmpgt(at).all()
        }
        fn ivec2_le(left:IVec2, right:IVec2) -> bool {
            left.cmple(right).all()
        }
        #[allow(dead_code)] // To remove if code changes
        fn ivec2_lt(left:IVec2, right:IVec2) -> bool { // Unused
            left.cmplt(right).all()
        }
        fn ivec2_ge(left:IVec2, right:IVec2) -> bool {
            left.cmpge(right).all()
        }
        fn ivec2_gt(left:IVec2, right:IVec2) -> bool {
            left.cmpgt(right).all()
        }
        #[derive(Debug, Clone, Copy)]
        struct IRect2 { // br is non-inclusive
            ul: IVec2,  // Upper left
            br: IVec2   // Bottom right
        }
        impl IRect2 {
            fn new(ul:IVec2, br:IVec2) -> Self { Self {ul, br} }
            fn new_centered(center:IVec2, size:IVec2) -> Self {
                let br = center + size/2; // Bias placement toward upper-left
                let ul = br - size;
                Self {ul, br}
            }
            fn within(&self, test:IVec2) -> bool {
                ivec2_le(self.ul, test) && ivec2_gt(self.br, test)
            }
            fn intersect(&self, test:IRect2) -> bool { // Will misbehave on 0-size rects
                self.within(test.ul) || {
                    let in_br = test.br+IVec2::NEG_ONE; // For testing within the point just inside must be in
                    self.within(in_br) || // All 4 corners
                    self.within(IVec2::new(test.ul.x, in_br.y)) ||
                    self.within(IVec2::new(in_br.x, test.ul.y))
                }
            }
            fn enclose(&self, test:IRect2) -> bool {
                ivec2_le(self.ul, test.ul) && ivec2_ge(self.br, test.br) // For testing enclose the rects only need to coincide
            }
            #[allow(dead_code)] // To remove if code changes
            fn size(&self) -> IVec2 {
                self.br - self.ul
            }
            fn center(&self) -> IVec2 {
                (self.br + self.ul)/2
            }
            fn offset(&self, by:IVec2) -> IRect2 {
                return IRect2::new(self.ul + by, self.br + by);
            }
            fn force_enclose_x(&self, test:IRect2) -> IRect2 { // ASSUMES SELF SMALLER THAN TEST
                let excess = test.ul.x - self.ul.x;
                if excess > 0 { return self.offset(IVec2::new(excess, 0)) }
                let excess = test.br.x - self.br.x;
                if excess < 0 { return self.offset(IVec2::new(excess, 0)) }
                self.clone()
            }
        }

        if false { // "FUNCTIONAL TESTS"
            let rect = IRect2::new(IVec2::new(5, 5), IVec2::new(15,15));
            for y in 0..3 {
                for x in 0..3 {
                    let v = IVec2::new(x*10,y*10);
                    assert_eq!(rect.within(v), (x==1 && y==1), "Incorrect within! rect: {:?} v: {:?}", rect, v);
                    let r2 = IRect2::new_centered(v, IVec2::ONE*2);
                    assert_eq!(rect.enclose(r2), (x==1 && y==1), "Incorrect enclose! rect: {:?} v: {:?}", rect, r2);
                }
            }
            for y in 0..5 {
                for x in 0..5 {
                    let v = IVec2::new(x*5,y*5);
                    let r2 = IRect2::new_centered(v, IVec2::ONE*2);
                    assert_eq!(rect.intersect(r2), !(x==0 || x==4 || y==0 || y==4), "Incorrect intersect! rect: {:?} v: {:?}", rect, r2);
                }
            }
        }

        let screen = IRect2::new(IVec2::ZERO, IVec2::new(DISPLAY_WIDTH as i32, DISPLAY_HEIGHT as i32));

        struct Vader {
            rect:IRect2
        }

        struct Ball {
            rect:IRect2,
            facing:IVec2
        }

        struct Player {
            rect:IRect2,
            facing:i32 // l/r
        }

        let mut vaders: Vec<Vader> = Default::default();
        let mut balls: Vec<Ball> = Default::default();
        let mut players: Vec<Player> = Default::default();

        const PLAYER_SIZE:IVec2 = IVec2::new(40, 8);
        const PLAYER_START:IVec2 = IVec2::new(DISPLAY_WIDTH as i32/2, DISPLAY_HEIGHT as i32-20-PLAYER_SIZE.y/2);
        const PLAYER_COLOR:u16 = 0b11111_101010_11111;
        const PLAYER_SPEED:i32 = 2;

        const BALL_SIZE:IVec2 = IVec2::new(4,4);
        let ball1_start:IVec2 = PLAYER_START + IVec2::new(0, -30);
        const BALL_COLOR:u16 = 0b00000_000000_11111 ^ 0xFFFF;
        const BALL_SPEED:i32 = 3;
        const BALL_FACING:IVec2 = IVec2::new(1,-1);

        const REFLECTS:[IVec2;2] = [ IVec2::new(-1,1), IVec2::new(1,-1) ];
        const REFLECT_BLEEP:u16 = 800*2;

        const DEATH_BLOOP_STROBE:u16 = 8*800;
        const DEATH_BLOOP:u16 = DEATH_BLOOP_STROBE*6;

        const VADER_COLS:i32 = 8;
        const VADER_ROWS:i32 = 4;
        const VADER_SIZE:IVec2 = IVec2::new(20, 12);
        const VADER_PADDING:IVec2 = IVec2::new(10,20);
        const VADER_ORIGIN:IVec2 = IVec2::new((DISPLAY_WIDTH as i32-(VADER_COLS*VADER_SIZE.x + (VADER_COLS-1)*VADER_PADDING.x))/2, 20);
        const VADER_COLOR:u16 = 0b11111_000000_00000 ^ 0xFFFF;

        assert_eq!(VADER_ORIGIN.x+VADER_PADDING.x >= 0, true, "Screen too narrow for vaders");

        players.push(Player { rect:IRect2::new_centered(PLAYER_START, PLAYER_SIZE), facing:0 });

        let ball_facing = { // Randomly start moving left or right; use the current UTC as a very weak RNG
            let mut ball_facing = BALL_FACING;
            if 0 == peripherals.APF_RTC.unix_seconds.read().bits() % 2 { ball_facing.x *= -1 }
            ball_facing
        };
        balls.push(Ball { rect:IRect2::new_centered(ball1_start, BALL_SIZE), facing:ball_facing });

        for y in 0..VADER_ROWS {
            for x in 0..VADER_COLS {
                let ul = VADER_ORIGIN + IVec2::new(x, y)*(VADER_SIZE + VADER_PADDING);
                vaders.push(Vader { rect:IRect2::new(ul, ul+VADER_SIZE) });
            }
        }

        // Gfx support

        fn fill(fb: *mut u16, rect:IRect2, color:u16) {
            for y in rect.ul.y..rect.br.y {
                for x in rect.ul.x..rect.br.x {
                    *pixel(fb, x as usize,y as usize) ^= color; // Assume positive
                }
            }
        }

        // Initial draw
        for player in &players { fill(fb, player.rect, PLAYER_COLOR); }
        for ball in &balls { fill(fb, ball.rect, BALL_COLOR); }
        for vader in &vaders { fill(fb, vader.rect, VADER_COLOR); }

        loop {
            loop {
                let video = peripherals.APF_VIDEO.video.read();
                let frame_ready = video.vblank_triggered().bit();

                // Complex tracking to see if frames were skipped
                cfg_if::cfg_if! {
                    if #[cfg(feature = "speed-debug")] {
                        let frame_ready = frame_ready || frame_already_overdue;
                        if frame_ready {
                            let video_frame_counter = video.frame_counter().bits();
                            if let Some(video_frame_counter_last) = video_frame_counter_last {
                                let gap = video_frame_counter as i32 - video_frame_counter_last as i32;
                                if gap > 1 {
                                    if 0== missed_deadline_count % SPEED_DEBUG_RATE {
                                        println!("Too slow! Dropped an entire frame (frames missing {}; fail #{})", gap-1, missed_deadline_count);
                                    }
                                    missed_deadline_count += 1;
                                } else {
                                    if missed_deadline_already { missed_deadline_count += 1 }
                                    if gap <= 0 {
                                        println!("Catastrophic failure: Video counts no frames between frames (gap of {})", gap);
                                    }
                                }
                            }
                            video_frame_counter_last = Some(video_frame_counter);
                            frame_already_overdue = false;
                            missed_deadline_already = false;
                        }
                    }
                }

                if frame_ready { break; }
            }

            // Controls
            let cont1_key = peripherals.APF_INPUT.cont1_key.read().bits() as u16; // Crop out analog sticks
            let cont1_key_edge = (!cont1_key_last) & cont1_key;
            cont1_key_last = cont1_key;

            use PocketControls::*;

            // Reset
            if cont1_key_edge & FaceStart as u16 != 0 {
                unsafe { peripherals.CTRL.reset.write(|w| w.bits(1)); } // 1 resets entire SOC
            }

            // Pause
            if !dead && cont1_key_edge & FaceSelect as u16 != 0 {
                paused = !paused;
            }

            // Mechanics

            if !paused && !dead && !won {
                // Vader
                // (When one block is left, have it start moving so you aren't stuck unable to hit it.)
                if vaders.len() == 1 {
                    let vader = &mut vaders[0];
                    fill(fb, vader.rect, VADER_COLOR);
                    if final_vader_facing == 0 {
                        final_vader_facing = if vader.rect.center().x > DISPLAY_WIDTH as i32/2
                            { -1 } else { 1 }
                    }
                    let vader_move = IVec2::new(final_vader_facing, 0);
                    let rect = vader.rect.offset(vader_move);
                    vader.rect = if screen.enclose(rect) { rect } else {
                        final_vader_facing = -final_vader_facing;
                        vader.rect.offset(-vader_move)
                    };
                    fill(fb, vader.rect, VADER_COLOR);
                }

                // Player
                for player in &mut players {
                    // Controls: Movement
                    const LR_MASK:u16 = DpadLeft as u16 | DpadRight as u16;
                    player.facing = if cont1_key & LR_MASK == LR_MASK { // Impossible on Analogue builtin but who knows about bluetooth
                        if cont1_key_edge & DpadLeft as u16 != 0 { -1 }
                        else if cont1_key_edge & DpadRight as u16 != 0 { 1 }
                        else { player.facing }
                    } else {
                        if cont1_key & DpadLeft as u16 != 0 { -1 }
                        else if cont1_key & DpadRight as u16 != 0 { 1 }
                        else { 0 }
                    };

                    if player.facing != 0 {
                        if config_chaos < 2 {
                            fill(fb, player.rect, PLAYER_COLOR);
                        }

                        player.rect = player.rect.offset(IVec2::new(player.facing*PLAYER_SPEED, 0))
                            .force_enclose_x(screen);

                        fill(fb, player.rect, PLAYER_COLOR);
                    }
                }

                // Ball
                for ball in &mut balls {
                    if config_chaos < 1 {
                        fill(fb, ball.rect, BALL_COLOR);
                    }

                    bleep_high = false;

                    // Step one pixel at a time, one axis at a time.
                    'step: for _ in 0..BALL_SPEED {
                        for (aid, axis) in IVec2::AXES.into_iter().enumerate() {
                            let v = axis*ball.facing;
                            //println!("TEST!! {}: {:?} = {:?} * {:?}", step, (aid,axis), ball.facing, v);
                            let rect = ball.rect.offset(v);
                            let mut reflect = false;
                            if !screen.enclose(rect) {
                                if v.y<=0 || CONFIG_IMMORTAL {
                                    reflect = true;
                                } else {
                                    // Touched bottom of screen. Game over.
                                    dead = true;
                                    blooping = DEATH_BLOOP;
                                    break 'step;
                                }
                            }
                            for player in &players {
                                if reflect { break; }
                                if player.rect.intersect(rect) {
                                    reflect = true;
                                    if v.y>0 {
                                        ball.facing.x = if ball.rect.center().x > player.rect.center().x
                                            { 1 } else { -1 }
                                    }
                                }
                            }
                            let mut destroy:Option<usize> = None;
                            for (idx,vader) in vaders.iter().enumerate() {
                                if reflect { break; }
                                if vader.rect.intersect(rect) {
                                    reflect = true;
                                    destroy = Some(idx);
                                    bleep_high = true;
                                    fill(fb, vader.rect, VADER_COLOR);
                                }
                            }
                            ball.rect = if !reflect {
                                rect
                            } else {
                                ball.facing *= REFLECTS[aid];
                                bleeping = REFLECT_BLEEP;
                                ball.rect.offset(-v)
                            };
                            if let Some(idx) = destroy { // Destroy one vader
                                vaders.remove(idx);
                                if vaders.len() == 0 {
                                    won = true;
                                    blooping = DEATH_BLOOP;
                                    break 'step;
                                }
                            }
                        }
                    }

                    fill(fb, ball.rect, BALL_COLOR);
                }
            }

            #[cfg(feature = "speed-debug")]
            {
                let video = peripherals.APF_VIDEO.video.read();
                if !video.vblank_status().bit() { // Status has already gone low
                    if 0== missed_deadline_count % SPEED_DEBUG_RATE {
                        println!("Too slow! Drawing finished outside vblank deadline (fail #{})", missed_deadline_count);
                    }
                    missed_deadline_already = true;
                }
                frame_already_overdue = video.vblank_triggered().bit();
            }

            // Audio gen
            let audio_needed = AUDIO_TARGET - peripherals.APF_AUDIO.buffer_fill.read().bits() as i32;
            for _ in 0..audio_needed { // 800 samples = 1/60 of a second. This will pause us long enough for a frame to pass
                let mut lfo_engaged = false;
                if (!paused) {
                    let freq_delta = if bleeping>0 {
                        bleeping -= 1;
                        if bleep_high { FREQ_DELTA*4 } else { FREQ_DELTA*2 }
                    } else if blooping>0 {
                        blooping -= 1;
                        if blooping == 0 { paused = true; }
                        if 0!=(blooping/DEATH_BLOOP_STROBE)%2 {
                            if !won { FREQ_DELTA/2 } else { FREQ_DELTA*4 }
                        } else { 0 }
                    } else {
                        lfo_engaged = true;
                        FREQ_DELTA
                    };
                    wave = wave.wrapping_add(freq_delta);

                    lfo = (lfo+1)%LFO_MAX;
                }

                let mut value:u32 = wave as u32;
                value = value >> 4;
                if lfo_engaged { value *= lfo as u32; value /= LFO_MAX as u32; }
                value = value | (value << 16);

                unsafe { peripherals.APF_AUDIO.out.write(|w| w.bits(value)) };
            }

            unsafe { peripherals.APF_AUDIO.playback_en.write(|w| w.bits(1)) };

            // Progress
            // if (!paused) {
            //     first_frame = false;
            // }
        }
    }

    // Unreachable
}
