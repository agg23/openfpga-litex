#include <sys/time.h>
#include <stdio.h>

#include <generated/csr.h>

int gettimeofday( struct timeval *tv, void *tzvp )
{
    // Latch time
    timer0_uptime_latch_write(1);

    uint64_t cycle_count = timer0_uptime_cycles_read();

    // Convert to seconds
    tv->tv_sec = cycle_count / CONFIG_CLOCK_FREQUENCY;
    // Get remaining microseconds
    tv->tv_usec = ( cycle_count % 1000000000 ) / 1000;

    // Success
    return 0;
}