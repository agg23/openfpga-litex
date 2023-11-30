/*  Written in 2018 by David Blackman and Sebastiano Vigna (vigna@acm.org)

To the extent possible under law, the author has dedicated all copyright
and related and neighboring rights to this software to the public domain
worldwide. This software is distributed without any warranty.

See <http://creativecommons.org/publicdomain/zero/1.0/>. */

#include <stdint.h>

/* This is xoroshiro64** 1.0, our 32-bit all-purpose, rock-solid,
   small-state generator. It is extremely fast and it passes all tests we
   are aware of, but its state space is not large enough for any parallel
   application.

   For generating just single-precision (i.e., 32-bit) floating-point
   numbers, xoroshiro64* is even faster.

   The state must be seeded so that it is not everywhere zero. */

/* This is the xoroshiro64** algorithm. It gives supercheap randomness
 * for NON-CRYPTOGRAPHIC APPLICATIONS. See: https://prng.di.unimi.it/
 *
 * This file modified trivially in 2023 by Andi McClure as follows:
 *   - Functions namespaced with xo_ prefixes
 *   - xo_seed to set 2 seed arguments.
 *   - xo_next made static, for use as header
 *   - "xo_rand(ceiling)" function gives "next random int" with ceiling
 * These changes are likewise released as CC0/public domain */

static inline uint32_t xo_rotl(const uint32_t x, int k) {
	return (x << k) | (x >> (32 - k));
}


static uint32_t xo_s[2];

static uint32_t xo_next(void) {
	const uint32_t s0 = xo_s[0];
	uint32_t s1 = xo_s[1];
	const uint32_t result = xo_rotl(s0 * 0x9E3779BB, 5) * 5;

	s1 ^= s0;
	xo_s[0] = xo_rotl(s0, 26) ^ s1 ^ (s1 << 9); // a, b
	xo_s[1] = xo_rotl(s1, 13); // c

	return result;
}

static void xo_seed(uint32_t s0, uint32_t s1) {
	xo_s[0] = s0;
	xo_s[1] = s1;
}

static uint32_t xo_rand_last = 0;
static uint32_t xo_rand_remaining = 0;

// Frontend to xoroshiro**, gives random number UNDER given ceiling
// This is designed to reduce calls to next(); its statistics are unanalyzed
// This is probably NOT constant time
static uint32_t xo_rand(uint32_t ceiling) {
	if (xo_rand_remaining < ceiling) {
		xo_rand_last = xo_next();
		xo_rand_remaining = 0xFFFFFFFF;
	}
	uint32_t result = xo_rand_last % ceiling;
	xo_rand_last /= ceiling;
	xo_rand_remaining /= ceiling;
	return result;
}
