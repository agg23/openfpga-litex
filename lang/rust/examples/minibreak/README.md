# Rust Example: minibreak

`make minibreak`

A minimal, very difficult brick breaking game implementing damage-based framebuffer updates.

![A paddle, a ball, a grid of blocks.](./screenshot.png)

## Controls

| Button     | Functionality |
| ---------- | ------------- |
| Left/Right | Move paddle   |
| Select     | Pause         |
| Start      | System reset  |

## License

This sample is written by [Andi McClure](https://pocket.runhello.com/). It is available under [Creative Commons Zero](https://creativecommons.org/publicdomain/zero/1.0/legalcode), in other words, it is public domain. If you substantially reuse the code, a credit would be appreciated, but this is not legally required.

## Suggestions

This game is hard, but beatable. Keep trying!

If you're using this as sample code: This app demonstrates several different features of the Pocket RISC-V core.

* It draws to the framebuffer;
* It plays sound;
* It uses gamepad controls (look for cont1_key);
* It can reset the system (look for FaceStart);
* It checks the system time, for weak RNG (look for unix_seconds).

You can delete basically all of this, but you'll want to keep the basic structure: Your app should be one big loop, the loop should begin by spinning until `vblank_triggered` goes high, and you want to draw first, refill your audio buffers second and execute your logic third (so that you get visuals drawn before VBLANK ends, and audio submitted before the buffer runs dry). 

If you want to try improving the example as written, some things to try might be:

- Instead of drawing the ball, paddle and blocks as rectangles, draw some sort of sprite.
- The game is brutally simple, so to keep it interesting at all I made it brutally hard. If more ideas were introduced, it could be made easier while still being interesting. Things other brick breaking games have introduced are more ball control (the ball can only bounce off at two angles, currently), per-level brick patterns, power-ups like multiball (the game has support for this, but it's not turned on), and bricks that take more than one hit to destroy. 
- Some sort of score (or if the game is kept simple and hard, a timer that tells you how quickly you beat it).
- The current app implements a useful trick: It keeps draws fast by *only* updating those screen pixels that have changed. It does this by XORing colors against the framebuffer when it draws, which means that drawing a sprite and erasing a sprite are identical. At the moment, this means draws are interleaved with the game logic, which means if the game logic gets more complicated it will overflow VBLANK and start tearing. Queueing draws and executing them at the start of the next frame would be better.
- In my testing, the game always completes its draw/update section before VBLANK ends. Yet somehow, on my device, there is still some vertical tearing (like a flicker when the ball moves across a certain point near the center of the screen). Anybody know what's happening here? Were my tests wrong? Is something messed up with the pixel fifo in the LiteX core itself?  