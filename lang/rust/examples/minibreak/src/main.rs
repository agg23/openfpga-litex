// Very difficult "breakout" clone
// Sample contributed by Andi McClure, available under Creative Commons Zero (public domain)
// If you substantially reuse this code a credit would be appreciated but is not required

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::slice::from_raw_parts_mut;

extern crate alloc;

use embedded_alloc::Heap;
use litex_hal as hal;
use litex_openfpga::*;
use litex_pac as pac;
use riscv_rt::entry;

mod irect2;

// Basic platform support

// Definition is required for uart_printer.rs to work
hal::uart! {
    UART: pac::UART,
}

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

// Drawing support

const DISPLAY_WIDTH: usize = 266;
const DISPLAY_HEIGHT: usize = 240;

const READ_LENGTH: usize = 0x10000;

fn render_init(framebuffer_address: *mut u16) {
    let framebuffer =
        unsafe { from_raw_parts_mut(framebuffer_address, DISPLAY_WIDTH * DISPLAY_HEIGHT) };

    const PIXEL_MAX: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
    for idx in 0..PIXEL_MAX {
        framebuffer[idx] = 0xFFFF;
    }
}

fn pixel(framebuffer_address: *mut u16, x: usize, y: usize) -> &'static mut u16 {
    let framebuffer = unsafe { from_raw_parts_mut(framebuffer_address as *mut u16, READ_LENGTH) };

    &mut framebuffer[y * DISPLAY_WIDTH + x]
}

// Gamepad controls

#[repr(u16)]
#[allow(dead_code)]
enum PocketControls {
    DpadUp = 1 << 0,
    DpadDown = 1 << 1,
    DpadLeft = 1 << 2,
    DpadRight = 1 << 3,
    FaceA = 1 << 4,
    FaceB = 1 << 5,
    FaceX = 1 << 6,
    FaceY = 1 << 7,
    TrigL1 = 1 << 8,
    TrigR1 = 1 << 9,
    TrigL2 = 1 << 10,
    TrigR2 = 1 << 11,
    TrigL3 = 1 << 12,
    TrigR3 = 1 << 13,
    FaceSelect = 1 << 14,
    FaceStart = 1 << 15,
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
    // Note we also had the option of simply picking an address and writing dma_base instead of reading it
    let fb: *mut u16 = peripherals.VIDEO_FRAMEBUFFER.dma_base.read().bits() as *mut u16;

    render_init(fb);

    // "APP"
    {
        use crate::irect2::*;
        use alloc::vec::Vec;
        use glam::IVec2;

        // Top-level config

        const CONFIG_CHAOS: u32 = 0; // 0-2 inclusive, set above 0 for funny pixel garbage effect
        const CONFIG_IMMORTAL: bool = false; // Set true to test without death

        // Basic state

        let mut paused = false;
        let mut dead = false;
        let mut won = false;
        let mut cont1_key_last = 0; // State of controller on previous loop

        // Display

        let screen = IRect2::new(
            IVec2::ZERO,
            IVec2::new(DISPLAY_WIDTH as i32, DISPLAY_HEIGHT as i32),
        );

        // Audio properties

        const AUDIO_TARGET: i32 = 48000 / 60 + 200; // Try to always fill audio buffer to this point

        // On audio: There will be three types of sound:
        // 1. Low pitched hum, volume modified by an LFO. This can be overriden by
        // 2. A "sound effect" bleep, either high or low, used for wall/object bounces. Overriden by
        // 3. A pulsating "bloop", used to indicate death or victory

        const AUDIO_LFO_MAX: u16 = 48000; // Speed (period) of background humming envelope
        const AUDIO_FREQ_DELTA: u16 = 150; // Step of basic sawtooth wave; increase to increase pitch of bleeps/bloops
        const AUDIO_REFLECT_BLEEP: u16 = 800 * 2; // How long (in samples) does a single bleep last?

        const AUDIO_DEATH_BLOOP_STROBE: u16 = 8 * 800; // How long is a single bloop pulsate?
        const AUDIO_DEATH_BLOOP: u16 = AUDIO_DEATH_BLOOP_STROBE * 6; // How long is the entire bloop sound effect?

        // Audio state

        let mut audio_wave: u16 = 0; // Sawtooth wave state used for all sounds
        let mut audio_lfo: u16 = AUDIO_LFO_MAX / 4; // "Low frequency oscillator"
        let mut audio_bleep_high = false; // If true and audio_bleeping, use high frequency
        let mut audio_bleeping = 0; // Remaining samples to play bleep
        let mut audio_blooping = 0; // Remaining cycles to play bloop

        // Game properties

        // This is a simple brick break game. There are players (paddles), balls, and vaders (blocks).
        // When balls hit vaders, they are destroyed. When it hits the paddle it bounces left or right,
        // depending on which side of the paddle it hit. The final vader starts trying to run away.

        // On graphics: All drawing is done via XOR. This is nice and simple (drawing is the same as erasing)
        // and generalizes nicely to sprites. When an object needs to move, it draws itself once at its
        // previous position (to erase) then again after updating its position.
        // Note because our background is white, all color constants are inverted below.

        const PLAYER_SIZE: IVec2 = IVec2::new(40, 8);
        const PLAYER_START: IVec2 = IVec2::new(
            DISPLAY_WIDTH as i32 / 2,
            DISPLAY_HEIGHT as i32 - 20 - PLAYER_SIZE.y / 2,
        );
        const PLAYER_COLOR: u16 = 0b11111_101010_11111; // Remember colors are RGB 565
        const PLAYER_SPEED: i32 = 2; // Velocity when button down

        const BALL_SIZE: IVec2 = IVec2::new(4, 4);
        let ball1_start: IVec2 = PLAYER_START + IVec2::new(0, -30); // Initial position
        const BALL_COLOR: u16 = 0b00000_000000_11111 ^ 0xFFFF;
        const BALL_SPEED: i32 = 3; // Movement per frame. Notice this is faster than the player.
        const BALL_FACING_START: IVec2 = IVec2::new(1, -1); // Initial velocity

        // Multiply by one of these vectors to reflect on the X or Y axis.
        const REFLECTS: [IVec2; 2] = [IVec2::new(-1, 1), IVec2::new(1, -1)];

        const VADER_COLS: i32 = 8; // Vaders appear in cols X rows grid
        const VADER_ROWS: i32 = 4;
        const VADER_SIZE: IVec2 = IVec2::new(20, 12);
        const VADER_PADDING: IVec2 = IVec2::new(10, 20); // Space between vaders
                                                         // Upper left pixel position of upper left vader
        const VADER_ORIGIN: IVec2 = IVec2::new(
            (DISPLAY_WIDTH as i32
                - (VADER_COLS * VADER_SIZE.x + (VADER_COLS - 1) * VADER_PADDING.x))
                / 2,
            20,
        );
        const VADER_COLOR: u16 = 0b11111_000000_00000 ^ 0xFFFF;

        assert_eq!(
            VADER_ORIGIN.x + VADER_PADDING.x >= 0,
            true,
            "Screen too narrow for vaders"
        );

        // Game state

        struct Vader {
            // Block
            rect: IRect2,
        }

        struct Ball {
            rect: IRect2,
            facing: IVec2, // Each axis should be 1 or -1
        }

        struct Player {
            rect: IRect2,
            facing: i32, // -1 or 1 l/r, or 0 when still
        }

        let mut vaders: Vec<Vader> = Default::default();
        let mut balls: Vec<Ball> = Default::default();
        let mut players: Vec<Player> = Default::default();

        players.push(Player {
            rect: IRect2::new_centered(PLAYER_START, PLAYER_SIZE),
            facing: 0,
        });

        let ball_facing_start = {
            // Randomly start off moving left or right; use the current UTC as a very weak RNG
            let mut ball_facing = BALL_FACING_START;
            if 0 == peripherals.APF_RTC.unix_seconds.read().bits() % 2 {
                ball_facing.x *= -1
            }
            ball_facing
        };
        balls.push(Ball {
            rect: IRect2::new_centered(ball1_start, BALL_SIZE),
            facing: ball_facing_start,
        });

        let mut final_vader_facing = 0; // Becomes nonzero when 1 vader left

        for y in 0..VADER_ROWS {
            for x in 0..VADER_COLS {
                let ul = VADER_ORIGIN + IVec2::new(x, y) * (VADER_SIZE + VADER_PADDING);
                vaders.push(Vader {
                    rect: IRect2::new(ul, ul + VADER_SIZE),
                });
            }
        }

        // Gfx support

        fn fill(fb: *mut u16, rect: IRect2, color: u16) {
            // XOR rectangle with given color
            for y in rect.ul.y..rect.br.y {
                for x in rect.ul.x..rect.br.x {
                    *pixel(fb, x as usize, y as usize) ^= color;
                }
            }
        }

        // Initial draw
        for player in &players {
            fill(fb, player.rect, PLAYER_COLOR);
        }
        for ball in &balls {
            fill(fb, ball.rect, BALL_COLOR);
        }
        for vader in &vaders {
            fill(fb, vader.rect, VADER_COLOR);
        }

        loop {
            // Busy loop until VBLANK begins, signaling next frame ready to go.
            // We'd like to do all drawing inside VBLANK to prevent tearing.
            loop {
                let video = peripherals.APF_VIDEO.video.read();

                if video.vblank_triggered().bit() {
                    break;
                }
            }

            // Controls

            let cont1_key = peripherals.APF_INPUT.cont1_key.read().bits() as u16; // Bitmask (crop out analog sticks)
            let cont1_key_edge = (!cont1_key_last) & cont1_key; // Bitmask is 1 iff a button press is *new this frame*
            cont1_key_last = cont1_key;

            use PocketControls::*;

            // Controls: Reset
            if cont1_key_edge & FaceStart as u16 != 0 {
                unsafe {
                    peripherals.CTRL.reset.write(|w| w.bits(1));
                } // 1 resets entire SOC
            }

            // Controls: Pause
            if !dead && cont1_key_edge & FaceSelect as u16 != 0 {
                paused = !paused;
            }

            // Mechanics

            if !paused && !dead && !won {
                // In these cases, freeze screen and loop to handle audio
                // Vader mechanics
                // (When one block is left, have it start moving so you aren't stuck unable to hit it.)
                if vaders.len() == 1 {
                    let vader = &mut vaders[0];
                    fill(fb, vader.rect, VADER_COLOR); // Erase

                    if final_vader_facing == 0 {
                        // This is our first frame with only 1 vader
                        // For an initial direction, move toward the screen center
                        final_vader_facing = if vader.rect.center().x > DISPLAY_WIDTH as i32 / 2 {
                            -1
                        } else {
                            1
                        }
                    }

                    // Move vader per facing
                    let vader_move = IVec2::new(final_vader_facing, 0);
                    let rect = vader.rect.offset(vader_move);

                    // Bounce at screen edge
                    vader.rect = if screen.enclose(rect) {
                        rect
                    } else {
                        final_vader_facing = -final_vader_facing;
                        vader.rect.offset(-vader_move)
                    };

                    fill(fb, vader.rect, VADER_COLOR); // Draw
                }

                // Player mechanics
                for player in &mut players {
                    // 2 player mode left as exercise to reader
                    // Controls: Movement
                    // Here we go to quite some trouble to handle the case of left and right held down at once--
                    // Which is impossible on the Analogue builtin buttons. But maybe it could happen on bluetooth
                    const LR_MASK: u16 = DpadLeft as u16 | DpadRight as u16;
                    player.facing = if cont1_key & LR_MASK == LR_MASK {
                        // L+R both down case:
                        if cont1_key_edge & DpadLeft as u16 != 0 {
                            -1
                        } else if cont1_key_edge & DpadRight as u16 != 0 {
                            1
                        } else {
                            player.facing
                        }
                    } else {
                        // Only one of L+R down case:
                        if cont1_key & DpadLeft as u16 != 0 {
                            -1
                        } else if cont1_key & DpadRight as u16 != 0 {
                            1
                        } else {
                            0
                        }
                    };

                    if player.facing != 0 {
                        // If moving
                        if CONFIG_CHAOS < 2 {
                            fill(fb, player.rect, PLAYER_COLOR); // Erase
                        }

                        // Update based on facing, then force back inside screen.
                        player.rect = player
                            .rect
                            .offset(IVec2::new(player.facing * PLAYER_SPEED, 0))
                            .force_enclose_x(screen);

                        fill(fb, player.rect, PLAYER_COLOR); // Draw
                    }
                }

                // Ball
                for ball in &mut balls {
                    if CONFIG_CHAOS < 1 {
                        fill(fb, ball.rect, BALL_COLOR); // Erase
                    }

                    // Step one pixel at a time, one axis at a time.
                    'step: for _ in 0..BALL_SPEED {
                        for (aid, axis) in IVec2::AXES.into_iter().enumerate() {
                            let v = axis * ball.facing; // Velocity on this axis

                            //println!("TEST!! {}: {:?} = {:?} * {:?}", step, (aid,axis), ball.facing, v);

                            let rect = ball.rect.offset(v); // Candidate rectangle, if we move 1 pixel on that axis.
                            let mut reflect = false; // Code below will test for a collision, and set "reflect" to reject the new position.

                            if !screen.enclose(rect) {
                                // Test collision with edge of screen
                                if v.y <= 0 || CONFIG_IMMORTAL {
                                    reflect = true;
                                    audio_bleep_high = false;
                                } else {
                                    // Touched bottom of screen. Game over.
                                    dead = true;
                                    audio_blooping = AUDIO_DEATH_BLOOP;
                                    break 'step;
                                }
                            }

                            for player in &players {
                                if reflect {
                                    break;
                                } // Already rejected

                                // Test collision with player paddle
                                if player.rect.intersect(rect) {
                                    reflect = true;
                                    audio_bleep_high = false;

                                    // "Steer" based on where on the paddle you hit
                                    if v.y > 0 {
                                        ball.facing.x =
                                            if ball.rect.center().x > player.rect.center().x {
                                                1
                                            } else {
                                                -1
                                            }
                                    }
                                }
                            }

                            let mut destroy: Option<usize> = None; // Select a vader to destroy
                            for (idx, vader) in vaders.iter().enumerate() {
                                if reflect {
                                    break;
                                } // Already rejected

                                if vader.rect.intersect(rect) {
                                    reflect = true;
                                    destroy = Some(idx);
                                    audio_bleep_high = true; // Only vaders bleep high
                                    fill(fb, vader.rect, VADER_COLOR); // Erase vader (screen)
                                }
                            }

                            // Did we select a vader to destroy?
                            // (We have to do this afterward so we don't mutate the vec while iterating it.)
                            if let Some(idx) = destroy {
                                vaders.remove(idx); // Erase vader (object)
                                if vaders.len() == 0 {
                                    // Oh, that was the last vader
                                    won = true;
                                    audio_blooping = AUDIO_DEATH_BLOOP;
                                    break 'step; // Don't bother drawing new ball position
                                }
                            }

                            // Set ball position from candidate
                            ball.rect = if !reflect {
                                rect
                            } else {
                                // Candidate was rejected; set a rectangle in the opposite direction.
                                ball.facing *= REFLECTS[aid];
                                audio_bleeping = AUDIO_REFLECT_BLEEP;
                                ball.rect.offset(-v)
                            };
                        }
                    }

                    fill(fb, ball.rect, BALL_COLOR); // Draw
                }
            }

            // Audio generation

            // Generate enough samples to fill us up to our desired buffer (a frame plus a safety margin)
            let audio_needed =
                AUDIO_TARGET - peripherals.APF_AUDIO.buffer_fill.read().bits() as i32;
            for _ in 0..audio_needed {
                let mut lfo_engaged = false; // True if the background "low pitched hum" is playing

                if !paused {
                    // When we pause we still output audio, held at the last PCM value.
                    // Remember (see "On audio" above), we have one saw wave generator and an LFO.
                    // freq_delta will determine the frequency of the saw generator this sample
                    let freq_delta = if audio_blooping > 0 {
                        // Case 3, strobing bloop
                        audio_blooping -= 1;

                        // No matter what, the game ends when the bloop is done
                        if audio_blooping == 0 {
                            paused = true;
                        }

                        // The bloop silences itself 1/2 the time, on the STROBE boundary
                        if 0 != (audio_blooping / AUDIO_DEATH_BLOOP_STROBE) % 2 {
                            // Run at base - 1 octave when losing, or base + 2 octaves when winning
                            if !won {
                                AUDIO_FREQ_DELTA / 2
                            } else {
                                AUDIO_FREQ_DELTA * 4
                            }
                        } else {
                            0
                        }
                    } else if audio_bleeping > 0 {
                        // Case 2, single bleep
                        audio_bleeping -= 1;

                        // Run at base + 2 octaves for a vader, or base + 1 octave otherwise
                        if audio_bleep_high {
                            AUDIO_FREQ_DELTA * 4
                        } else {
                            AUDIO_FREQ_DELTA * 2
                        }
                    } else {
                        // Case 1, pulsating hum at exactly base frequency
                        lfo_engaged = true;
                        AUDIO_FREQ_DELTA
                    };

                    // Simplest waveform possible: Increment last sample's value by the delta, then wrap around at 2^16
                    audio_wave = audio_wave.wrapping_add(freq_delta);

                    // LFO state only increments when not paused
                    audio_lfo = (audio_lfo + 1) % AUDIO_LFO_MAX;
                }

                let mut value: u32 = audio_wave as u32;

                // Max volume is 2^12-1
                value = value >> 4;

                // Apply LFO envelope
                if lfo_engaged {
                    value *= audio_lfo as u32;
                    value /= AUDIO_LFO_MAX as u32;
                }

                // Output value is two stereo i16s packed into one u32
                // Notice we did our math above in u32; it doesn't matter because bit 15 is always 0
                value = value | (value << 16);

                unsafe { peripherals.APF_AUDIO.out.write(|w| w.bits(value)) };
            }

            unsafe { peripherals.APF_AUDIO.playback_en.write(|w| w.bits(1)) };
        }
    }

    // Unreachable
}
