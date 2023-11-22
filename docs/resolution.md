# Changing the Resolution

If you don't like the default resolution selected by the core, you can change it to anything supported by the Analogue Pocket. Remember openFPGA is limited to 800x720, so you can't choose a resolution outside of those bounds.

1. Find your desired vertical and horizontal counts. [See my wiki](https://github.com/agg23/analogue-pocket-utils/wiki/Video) for additional information about how to select counts, but briefly, visit https://tomverbeure.github.io/video_timings_calculator and enter your desired resolution. It will show you the "conforms to protocol" options that you can select between, but you are not required to use one of these for the Pocket; it is just an example. If you choose an unusual resolution (like the default 266x240), there won't be a standard to base the calculations off of, so the tool will estimate what it would be.  
Note that you want all of your system clocks (CPU, and 2x for SDRAM) to be integer multiples of this clock, and to pass timing you probably need the CPU clock to be < 60MHz. For example, you could choose counts that require a 10MHz clock, which would either be a 5x or 6x multiple (50MHz or 60MHz) for the CPU clock.

2. In Quartus, open the `mf_pllbase` megafunction on the left side of the screen. This will open the tool where you can manipulate the clocks for the core. The last two clocks should be the video pixel clock that you chose, the first is your CPU clock (remember to keep it an integer multiple of the video clock), and the second and third clocks are your SDRAM clock (remember to keep it 2x the CPU clock). Enter those values and click finish.

3. In `/litex/analogue_pocket.py`, edit the `CLOCK_SPEED` constant to your new CPU clock speed. Scroll down to `add_video_framebuffer` and insert your vertical and horizontal counts. Please note that you _must_ change the name ("266x240@60Hz") because for some reason LiteX uses this string for some calculations.

4. Update the Pocket `video.json` to your new resolution. Without it, the Pocket won't know what resolution you want to display.

5. Build the LiteX project by running `make` in the `/litex` directory. See LiteX build instructions for more information.

6. Build the Quartus project by pressing "Build".

7. Reverse the produced bitstream (`*.rbf`). See https://www.analogue.co/developer/docs/packaging-a-core#fpga-bitstream

8. Update your clock speed constants in your software code, if any.

9. The core should now be ready to use at the new resolution.

Feel free to contact me and ask for help and/or clarification about any of this. I imagine I missed something here.