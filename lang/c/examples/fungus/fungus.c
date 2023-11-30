// Fluidly expanding colors
// Sample contributed by Andi McClure, available under Creative Commons Zero (public domain)
// If you substantially reuse this code a credit would be appreciated but is not required

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// #include <irq.h>
// #include <libbase/uart.h>
// #include <libbase/console.h>
#include <generated/csr.h>

// Turn on to print warnings when frame fails to draw within vblank
#define SPEED_DEBUG 0

#define DISPLAY_WIDTH 266
#define DISPLAY_HEIGHT 240

#define BITS5 ((1<<5)-1)
#define BITS6 ((1<<6)-1)
// Takes 3 numbers in range 0..64. Lowest bit on R and B will be discarded.
#define COLOR(r,g,b) ( ( (((r)>>1)&BITS5)<<11 ) | ( ((g)&BITS6)<<5 ) | ( (((b)>>1)&BITS5) ))

#define PILLAR_COUNT 3
#define PILLAR_SIZE 10
#define PILLAR_GAP 60
#define PILLAR_COLOR 0x
#define PILLARS_SIZE (PILLAR_GAP*(PILLAR_COUNT-1)+PILLAR_SIZE*PILLAR_COUNT)
// Given an axis of size n, what offset is needed to center the group of pillars?
#define PILLARS_BASE(n) (((n)-PILLARS_SIZE)/2)

#define CANDIDATE_COUNT 100
#define WINNER_COUNT 100

// What index within the framebuffer is this pixel at?
#define AT(x,y) (((y)*DISPLAY_WIDTH)+(x))

typedef struct {
	uint16_t x;
	uint16_t y;
} Candidate;

inline Candidate make_candidate(int x, int y) {
	Candidate candidate = {x,y};
	return candidate;
}

// Standalone random number generator
#include "xoshiro128starstar.h"

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

int main(void)
{
	printf("-- Fungus --\n");

	uint16_t *fb = (uint16_t *)(uintptr_t)video_framebuffer_dma_base_read();

	// Fill screen with black
	for(int c = 0; c < DISPLAY_WIDTH*DISPLAY_HEIGHT; c++)
		fb[c] = 0;

	{ // Primitive randomness seed
		uint32_t time = apf_rtc_unix_seconds_read();
		xo_jump(time+5, time+3, time+2, time);
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
						fb[AT(x_block+x, y_block+y)] = COLOR(32, 0, 0);
					}
				}
			}
		}
	}

	// Who needs a heap anyway
	Candidate candidates[2][CANDIDATE_COUNT];
	int candidates_len[2] = {0,0};
	int current = 0;
	int color = COLOR(0,32,0);
	#define SHADOW_FRAMEBUFFER_SIZE ((DISPLAY_WIDTH*DISPLAY_HEIGHT)/8)
	static uint8_t shadow_framebuffer[SHADOW_FRAMEBUFFER_SIZE]; // Too big for heap

	while (1)
	{
		while (1) {
			uint32_t video = apf_video_video_read();
			if (apf_video_video_vblank_triggered_extract(video))
				break;
		}

		// At any one time we have two lists of points we can expand into;
		// one for the current frame, and one for the next frame.
		int next = (current+1)%2;
		Candidate *candidates_current = &candidates[current][0];
		Candidate *candidates_next = &candidates[next][0];

		#define CANDIDATE_PUSH(candidate) { candidates_next[candidates_len[next]] = candidate; candidates_len[next]++; }

		if (candidates_len[current] == 0) {
			candidates_current[0] = make_candidate(DISPLAY_WIDTH/2, DISPLAY_HEIGHT/2);
			candidates_len[current] = 1;
		}

		// Draw the current list (but only the lucky first handful)
		for(int idx = 0; idx < WINNER_COUNT && idx < candidates_len[current]; idx++) {
			Candidate winner = candidates_current[idx];
			fb[AT(winner.x, winner.y)] = color;
		}

		// Prepare for next frame
		// Since we don't have to worry about vblank finishing, we can take our time now.
		for(int idx = 0; idx < SHADOW_FRAMEBUFFER_SIZE; idx++)
			shadow_framebuffer[idx] = 0;
		for(int idx = 0; idx < candidates_len[current] && candidates_len[next] < CANDIDATE_COUNT; idx++) {
			Candidate check = candidates_current[idx];
			Candidate neighbors[4] = {
				{check.x, (check.y+1)%DISPLAY_HEIGHT},
				{(check.x+1)%DISPLAY_WIDTH, check.y},
				{check.x, (check.y+DISPLAY_HEIGHT-1)%DISPLAY_HEIGHT},
				{(check.x+DISPLAY_WIDTH-1)%DISPLAY_WIDTH, check.y}
			};
			for(int n = 0; n < 4; n++) {
				if (candidates_len[next] >= CANDIDATE_COUNT)
					break;
				size_t neighbor_at = AT(neighbors[n].x, neighbors[n].y);
				// Use the shadow framebuffer to check for pixels we've checked this frame
				size_t neighbor_shadow_idx = neighbor_at/8;
				uint8_t neighbor_shadow_bit = 1<<(neighbor_at%8); 
				if (shadow_framebuffer[neighbor_shadow_idx] & neighbor_shadow_bit)
					continue;
				shadow_framebuffer[neighbor_shadow_idx] |= neighbor_shadow_bit;
				uint16_t color_minus_current = color-fb[neighbor_at];
				int16_t color_minus_current_diff = color_minus_current;
				if (color_minus_current_diff <= 0)
					continue;
				CANDIDATE_PUSH(neighbors[n]);
			}
		}

		// Randomize candidates list so we don't just go continually downward
		fisher_yates(candidates_next, candidates_len[next]);

		candidates_len[current] = 0;
		current = next;
		color++; // Treat the 565 bit-packed color as a single integer. Normally, you don't want this.
	}

	return 0;
}
