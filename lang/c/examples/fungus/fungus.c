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

#define CANDIDATES_COUNT 100
#define WINNERS_COUNT 10

// What index within the framebuffer is this pixel at?
#define at(x,y) (((y)*DISPLAY_WIDTH)+(x))

typedef struct {
	uint16_t x;
	uint16_t y;
} Candidate;

int main(void)
{
	printf("-- Fungus --\n");

	uint16_t *fb = (uint16_t *)(uintptr_t)video_framebuffer_dma_base_read();

	// Fill screen with black
	for(int c = 0; c < DISPLAY_WIDTH*DISPLAY_HEIGHT; c++)
		fb[c] = 0;

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
						fb[at(x_block+x, y_block+y)] = COLOR(32, 0, 0);
					}
				}
			}
		}
	}

	Candidate candidates[100][2];
	int current = 0;
	int color = 0;

	while (1)
	{
		while (1) {
			uint32_t video = apf_video_video_read();
			if (apf_video_video_vblank_triggered_extract(video))
				break;
		}

		// Placeholder: Rainbow block
		int y_block = y_root+1*(PILLAR_SIZE+PILLAR_GAP), x_block = x_root+1*(PILLAR_SIZE+PILLAR_GAP);
		for(int y = 0; y < PILLAR_SIZE; y++) {
			for(int x = 0; x < PILLAR_SIZE; x++) {
				fb[at(x_block+x, y_block+y)] = color;
			}
		}

		color++; // Treat the 565 bit-packed color as a single integer. Normally, you don't want this.
	}

	return 0;
}
