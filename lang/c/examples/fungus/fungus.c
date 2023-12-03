// Fluidly expanding colors and grumbling sound
// Sample contributed by Andi McClure, available under Creative Commons Zero (public domain)
// If you substantially reuse this code a credit would be appreciated but is not required

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <assert.h>

// #include <irq.h>
// #include <libbase/uart.h>
// #include <libbase/console.h>
#include <generated/csr.h>
#include <generated/soc.h>

// Platform support

#define DISPLAY_WIDTH MAX_DISPLAY_WIDTH
#define DISPLAY_HEIGHT MAX_DISPLAY_HEIGHT

#define BITS5 ((1<<5)-1)
#define BITS6 ((1<<6)-1)
// Takes 3 numbers in range 0..64. Lowest bit on R and B will be discarded.
#define COLOR(r,g,b) ( ( (((r)>>1)&BITS5)<<11 ) | ( ((g)&BITS6)<<5 ) | ( (((b)>>1)&BITS5) ))

typedef enum {
    dpad_up     = 1<<0,
    dpad_down   = 1<<1,
    dpad_left   = 1<<2,
    dpad_right  = 1<<3,
    face_a      = 1<<4,
    face_b      = 1<<5,
    face_x      = 1<<6,
    face_y      = 1<<7,
    trig_l1     = 1<<8,
    trig_r1     = 1<<9,
    trig_l2     = 1<<10,
    trig_r2     = 1<<11,
    trig_l3     = 1<<12,
    trig_r3     = 1<<13,
    face_select = 1<<14,
    face_start  = 1<<15,
} PocketControls;

// What index within the framebuffer is this pixel at?
#define AT(x,y) (((y)*DISPLAY_WIDTH)+(x))

// Standalone random number generator

// Set 1 to use the slower (??) 128-bit xoshiro RNG
#define XO_128 0

#if XO_128
#include "xoshiro128starstar.h"
#else
#include "xoroshiro64starstar.h"
#endif

// App properties

// Sizes and spacing of "pillar" squares
#define PILLAR_COUNT 3
#define PILLAR_SIZE 40
#define PILLAR_GAP 30
#define PILLAR_COLOR (COLOR(20, 0, 0))
#define PILLARS_SIZE (PILLAR_GAP*(PILLAR_COUNT-1)+PILLAR_SIZE*PILLAR_COUNT)
// Given an axis of size n, what offset is needed to center the group of pillars?
#define PILLARS_BASE(n) (((n)-PILLARS_SIZE)/2)

static_assert(PILLARS_SIZE <= DISPLAY_WIDTH && PILLARS_SIZE <= DISPLAY_HEIGHT, "The screen size you are compiling for is too small for this app. (Hint: Try reducing the PILLAR_SIZE constant.)");

// Candidate buffer size
#define CANDIDATE_TRUE_MAX 1600

// How many speeds does the B button cycle between
#define SPEED_COUNT 3

// How many colors to "avoid" in super grow mode
#define SUPER_GROW_MARGIN 0x100

// Audio properties (see "How the audio works" below)
// Note: setting AUDIO_SCALE to 256 and AUDIO_GAP to 4 is also a pretty fun sound

// How many samples do we want in the audio buffer at any one time? (currently "2 frames worth")
#define AUDIO_TARGET (48000/60 * 2)
// What is the rise/fall rate of our triangle wave?
#define AUDIO_SCALE 128
// What is the loudest our triangle wave can get?
#define AUDIO_CEILING (1<<15)
// If AUDIO_GAP is above 2 (it must be at least 2), gaps will be put between triangle-wave bumps
#define AUDIO_GAP 2
// Determines pitch of "beeps" when a button is pressed
#define AUDIO_BEEP_BASE 5
// Length of "beep" when a button is pressed
#define AUDIO_BEEP_TIME (48000/AUDIO_BEEP_BASE/2)
// Amplitude of "beep" when a button is pressed
#define AUDIO_BEEP_VOLUME ((1<<16)/32)

// App support

typedef struct {
    uint16_t x;
    uint16_t y;
} Candidate;

inline Candidate make_candidate(int x, int y) {
    Candidate candidate = {x,y};
    return candidate;
}

// Will use for colors
static inline uint32_t xo_rotr(const uint32_t x, int k) {
    return (x >> k) | (x << (32 - k));
}

// Fisher-Yates Shuffle, modeled on discussion at https://stackoverflow.com/a/42322025
// This will efficiently reorder an array randomly
static void fisher_yates(Candidate *array, int len) {
     Candidate temp;

     for (int idx_ceiling = len-1; idx_ceiling > 0; idx_ceiling--) { // Iterate array backward
         int idx_rand = xo_rand(idx_ceiling + 1); // Swap each member with a random member below it
         if (idx_ceiling != idx_rand) {
             temp = array[idx_ceiling];
             array[idx_ceiling] = array[idx_rand];
             array[idx_rand] = temp;
         }
     }
}

// To beep, call "BEEP" (do not call "beep" directly)
static void beep(uint16_t set_beep_speed, uint16_t audio_wave, bool *audio_beeping, uint16_t *audio_beep_speed, uint16_t *audio_beep_time, int16_t *audio_beep_sign) {
    *audio_beeping = true;
    *audio_beep_speed = set_beep_speed;
    *audio_beep_time = 0;
    *audio_beep_sign = audio_wave > (1<<15) ? -1 : 1;
}

#define BEEP(set_beep_speed) beep((set_beep_speed), audio_wave, &audio_beeping, &audio_beep_speed, &audio_beep_time, &audio_beep_sign);


int main(void)
{
    printf("-- Fungus --\n");

    uint16_t *fb = (uint16_t *)(uintptr_t)video_framebuffer_dma_base_read();

    // Fill screen with black
    for(int c = 0; c < DISPLAY_WIDTH*DISPLAY_HEIGHT; c++)
        fb[c] = 0;

    { // Primitive randomness seed
        uint32_t time = apf_rtc_unix_seconds_read();
#if XO_128
        xo_jump(time+5, time+3, time+2, time);
#else
        xo_seed(~time, time);
#endif
    }

    // Draw a 3x3 grid of rectangles with a hole in the middle, to break up the field
    int y_root = PILLARS_BASE(DISPLAY_HEIGHT), x_root = PILLARS_BASE(DISPLAY_WIDTH);
    {
        for(int by = 0; by < 3; by++) {
            for(int bx = 0; bx < 3; bx++) {
                if (bx == (PILLAR_COUNT/2) && by == (PILLAR_COUNT/2)) // Hole
                    continue;
                int y_block = y_root+by*(PILLAR_SIZE+PILLAR_GAP), x_block = x_root+bx*(PILLAR_SIZE+PILLAR_GAP);
                for(int y = 0; y < PILLAR_SIZE; y++) {
                    for(int x = 0; x < PILLAR_SIZE; x++) {
                        fb[AT(x_block+x, y_block+y)] = PILLAR_COLOR;
                    }
                }
            }
        }
    }

    // How the graphics work:
    // Essentially this app runs 100 to 1600 tiny agents that are taking a random walk on the screen.
    // Each agent leaves behind a trail of colored pixels; the color is shared and changes once per frame.
    // The current state of the screen, plus a "shadow framebuffer" that is cleared every frame, are
    // used to prevent the agents from stomping on each other.
    // The agents live in a pair of "candidate" arrays. On each frame we designate one array "current"
    // and one array "next". Each candidate in "current" attempts to spawn 4 children in "next".
    // After filling "next" we fill randomly sort it each frame, to prevent bias in any specific direction.

    // App state
    // We don't have a heap, so all variables need to be stack or static/global

    // Our two queues of "candidate"
    Candidate candidates[2][CANDIDATE_TRUE_MAX];
    int candidates_len[2] = {0,0}; // How many candidates are actually present in each queue?
    int current = 0; // Which of the two queues is "current"?
    int color = COLOR(0,32,0); // What color are the candidates currently drawing?
    #define SHADOW_FRAMEBUFFER_SIZE ((DISPLAY_WIDTH*DISPLAY_HEIGHT)/8)
    static uint8_t shadow_framebuffer[SHADOW_FRAMEBUFFER_SIZE]; // 1 bit per candidate. Too big for stack

    // "Candidates" are points which are currently spawning other candidates,
    // "Winners" are candidates that are currently allowed to draw to the screen.
    // Col 1 is candidate count, col 2 is winner ratio (when winner_cut on), col 3 is beep pitch (when pressing B)
    const int speeds[SPEED_COUNT][3] = { {100, 10, 1}, {400, 4, 2}, {1600, 2, 4} };

    // How the audio works:
    // We are generating a triangle wave which changes its frequency each cycle.
    // This generates a random combination of mostly low rumbling interspersed with squeaks.
    // The way this is done is starting from a "floor" of the DAC's lowest value (signed INT_MIN),
    // Each cycle we pick a "ceiling" and rise to it then descend from it at a constant rate.
    // (So audio_wave_ceil sets pitch AND amplitude; the low rumbles are louder than the squeaks.)
    // When the user presses a button, the triangle wave halts and a saw wave plays atop it.

    // Audio state
    // Note we calculate all sound in uint16_t space and convert to int16_t at the last moment.

    // audio_cycle determines if we are rising (0) resting (1) or waiting (>1)
    // Note: We start at cycle 1, wave value 2^15; after converting from uint16_t to int16_t,
    // this means we start by descending from signed 0. This avoids a pop when the app boots.
    uint16_t audio_cycle = 1;
    // When waiting (audio_cycle>1), this counter determines how long we've been waiting.
    uint16_t audio_silence = 0;
    // Triangle wave state/output
    uint16_t audio_wave = 1<<15;
    // Triangle wave target value
    uint16_t audio_wave_ceil = 0;
    // Is a beep currently occurring?
    bool audio_beeping = false;
    // If a beep is occurring, how much does it rise per frame?
    uint16_t audio_beep_speed = 1;
    // If a beep is occurring, this counter determines how long we've been beeping.
    uint16_t audio_beep_time = 0;
    // Determines whether the beep should "rise" or "fall" from the current audio_wave value
    // (so if we begin a beep when audio_wave is near UINT_MAX, we don't clip)
    int16_t audio_beep_sign = 1;

    // Controls state

    bool paused = false;
    uint16_t cont1_key_last = 0; // cont1_key on previous loop

    // Switches between two modes for determining which framebuffer colors block growth.
    // Normal mode: colors > 2^15 away are impassable. This causes the red pillars to act like walls.
    // Super grow mode: The last 256 colors written are impassable. This causes the fungus to avoid
    // where it's recently been and habitually grow "outward".
    bool super_grow = false;
    // Increases the rate at which the write color changes.
    bool super_cycle = false;
    // When true, only a subset of candidates actually draw to the screen. When false, all do.
    bool winner_cut = true;
    // Which configuration in "speeds" we are currently using (determines number of candiates/winners)
    int speed = 1;

    while (1)
    {
        // Busy loop until VBLANK begins, signaling next frame ready to go.
        // We'd like to do all drawing inside VBLANK to prevent tearing.
        while (1) {
            uint32_t video = apf_video_video_read();
            if (apf_video_video_vblank_triggered_extract(video))
                break;
        }

        // Designate the "growing from" and "growing into" candidate frames.
        int next = (current+1)%2;
        Candidate *candidates_current = &candidates[current][0];
        Candidate *candidates_next = &candidates[next][0];

        #define CANDIDATE_PUSH(candidate) { candidates_next[candidates_len[next]] = candidate; candidates_len[next]++; }

        // All the fungus has died. Attempt to regrow from screen center (will also run on first iteration)
        if (candidates_len[current] == 0) {
            candidates_current[0] = make_candidate(DISPLAY_WIDTH/2, DISPLAY_HEIGHT/2);
            candidates_len[current] = 1;
        }

        // Draw the current list
        int candidates_max = speeds[speed][0];
        int winner_count = candidates_max;
        if (winner_cut) winner_count /= speeds[speed][1]; // (But in winner_cut mode, only the lucky first few)
        for(int idx = 0; idx < winner_count && idx < candidates_len[current]; idx++) {
            Candidate winner = candidates_current[idx];
            fb[AT(winner.x, winner.y)] = color;
        }

        // Play audio
        size_t audio_needed = AUDIO_TARGET - apf_audio_buffer_fill_read();
        for(size_t idx = 0; idx < audio_needed; idx++) {
            if (paused || audio_beeping) {
                // Do nothing
            } else if (0 == audio_cycle % AUDIO_GAP) {
                if (audio_wave >= audio_wave_ceil) {
                    audio_cycle++;
                } else {
                    audio_wave += AUDIO_SCALE;
                }
            } else if (1 == audio_cycle % AUDIO_GAP) {
                if (audio_wave == 0) {
                    audio_wave_ceil = xo_rand(AUDIO_CEILING);
                    audio_silence = 0;
                    audio_cycle++;

                } else {
                    audio_wave -= AUDIO_SCALE;
                }
            } else { // Unused unless AUDIO_GAP goes above 1
                if (audio_silence >= audio_wave_ceil) {
                    audio_wave = 0;
                    audio_silence = 0;
                    audio_cycle++;
                } else {
                    audio_silence += AUDIO_SCALE;
                }
            }

            // Convert from mono unsigned to packed stereo signed
            uint32_t value = audio_wave;
            value = (value + (1<<15)) & 0xFFFF;
            if (audio_beeping) { // Beep even when paused
                int16_t signed_value = value;
                signed_value += (int16_t)((audio_beep_time*audio_beep_speed)%AUDIO_BEEP_VOLUME)*audio_beep_sign;
                value = (uint16_t)signed_value;
                audio_beep_time++;
                if (audio_beep_time >= AUDIO_BEEP_TIME)
                    audio_beeping = false;
            }
            apf_audio_out_write(value | (value<<16));
        }
        apf_audio_playback_en_write(1);

        // This frame complete; now prepare for next frame
        // Since we don't have to worry about vblank finishing, we can take our time now.

        // Handle controls
        uint16_t cont1_key = apf_input_cont1_key_read(); // Bitmask (crop out analog sticks)
        uint16_t cont1_key_edge = (~cont1_key_last) & cont1_key; // Bitmask is 1 iff a button press is *new this frame*
        cont1_key_last = cont1_key;

        if (cont1_key_edge & face_select) {
            paused = !paused;
        }

        if (cont1_key_edge & face_start) {
            ctrl_reset_write(1); // 1 resets entire SOC
        }

        if (cont1_key_edge & face_y) {
            winner_cut = !winner_cut;
            BEEP(AUDIO_BEEP_BASE*(winner_cut?2:4));
        }

        if (cont1_key_edge & face_x) {
            super_grow = !super_grow;
            BEEP(AUDIO_BEEP_BASE*(super_grow?4:2));
        }

        if (cont1_key_edge & face_b) {
            speed = (speed + 1) % SPEED_COUNT;
            BEEP(AUDIO_BEEP_BASE*speeds[speed][2]);
        }

        if (cont1_key_edge & face_a) {
            super_cycle = !super_cycle;
            BEEP(AUDIO_BEEP_BASE*(super_cycle?4:2));
        }

        // Triggers rotate color left or right.
        // Notice these are imbalanced, so mashing L and R will slowly rotate 1 bit
        if (cont1_key_edge & trig_l1) {
            color = xo_rotl(color, 5);
        }

        if (cont1_key_edge & trig_r1) {
            color = xo_rotr(color, 6);
        }

        // Grow fungus
        if (!paused) { // Note we DON'T halt drawing during pause, only updates and sound
            // Clear shadow framebuffer
            for(int idx = 0; idx < SHADOW_FRAMEBUFFER_SIZE; idx++)
                shadow_framebuffer[idx] = 0;

            // Iterate over candidates
            for(int idx = 0; idx < candidates_len[current] && candidates_len[next] < candidates_max; idx++) {
                Candidate check = candidates_current[idx];

                // Pixels we can grow into
                Candidate neighbors[4] = {
                    {check.x, (check.y+1)%DISPLAY_HEIGHT},
                    {(check.x+1)%DISPLAY_WIDTH, check.y},
                    {check.x, (check.y+DISPLAY_HEIGHT-1)%DISPLAY_HEIGHT},
                    {(check.x+DISPLAY_WIDTH-1)%DISPLAY_WIDTH, check.y}
                };

                // Controls: Move around when d-pad held
                if (cont1_key & (dpad_up | dpad_down | dpad_left | dpad_right)) {
                    for(int nidx = 0; nidx < 4; nidx++) {
                        Candidate *neighbor = &neighbors[nidx];
                        if (cont1_key & dpad_up) {
                            neighbor->y = (neighbor->y-1+DISPLAY_HEIGHT)%DISPLAY_HEIGHT;
                        }
                        if (cont1_key & dpad_down) {
                            neighbor->y = (neighbor->y+1)%DISPLAY_HEIGHT;
                        }
                        if (cont1_key & dpad_left) {
                            neighbor->x = (neighbor->x-1+DISPLAY_WIDTH)%DISPLAY_WIDTH;
                        }
                        if (cont1_key & dpad_right) {
                            neighbor->x = (neighbor->x+1)%DISPLAY_WIDTH;
                        }
                    }
                }

                // Act on neighbors
                for(int n = 0; n < 4; n++) {
                    if (candidates_len[next] >= candidates_max)
                        break;
                    size_t neighbor_at = AT(neighbors[n].x, neighbors[n].y);

                    // Use the shadow framebuffer to avoid pixels we've already checked this frame
                    size_t neighbor_shadow_idx = neighbor_at/8;
                    uint8_t neighbor_shadow_bit = 1<<(neighbor_at%8);
                    if (shadow_framebuffer[neighbor_shadow_idx] & neighbor_shadow_bit)
                        continue;
                    shadow_framebuffer[neighbor_shadow_idx] |= neighbor_shadow_bit;

                    // Use the real framebuffer to see which pixels are "blocked"
                    uint16_t color_prev = fb[neighbor_at];
                    uint16_t color_minus_current = color-color_prev;
                    if (super_grow) {
                        color_minus_current += SUPER_GROW_MARGIN;
                        if (color_prev && color_minus_current < SUPER_GROW_MARGIN*2) { // Multiply by 2 because I feel like it.
                            continue;
                        }
                    } else {
                        int16_t color_minus_current_diff = color_minus_current;
                        if (color_minus_current_diff <= 0)
                            continue;
                    }

                    // This pixel is allowed to grow
                    CANDIDATE_PUSH(neighbors[n]);
                }
            }

            // Trick: It's really easy in the specific "speed 2, super grow" configuration to get "stuck",
            // so instead of resetting to 0, we just hold until we get "unstuck", because it looks cool
            if (super_grow && speed == 2 && !candidates_len[next]) {
                // Do this by late-swapping current and next
                int tmp = next;
                next = current;
                current = tmp;
            }

            // Randomize candidates list so we don't just go continually downward
            fisher_yates(candidates_next, candidates_len[next]);

            // Swap next and current
            candidates_len[current] = 0;
            current = next;
        } else {
            // Also randomize candidates list while paused, for cool fuzz
            fisher_yates(candidates_current, candidates_len[current]);
        }

        // Color cycle: Treat the 565 bit-packed color as a single integer and increment it.
        // Normally, you don't want this, but in this case it's a very cheap way to get color cycling.
        if (super_cycle) {
            color += 16;
        } else {
            color++;
        }
    }

    return 0;
}
