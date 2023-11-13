It is very important that you remain aware that register locations and offsets can change between hardware revisions, and instead you should be using the `SVD` file to derive constants to these registers.

# Audio

The system provides audio via a write only audio buffer of 4096 elements. The Pocket accepts solely 48khz `i16` audio, thus the exposed write register takes the form of `{i16, i16}`, or a single 32 bit word containing both channels. This is one sample at 48khz

## CSR

Base address (`MAIN` block): `0xF000_1000`

| Name                 | Offset | Dir | Width | Description                                                                                                                                                                                           |
| -------------------- | ------ | --- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `audio_out`          | `0x24` | W   | 32    | The entrypoint to the audio buffer. Write two 16 bit signed values (for the left and right audio channels) here. This will push one value into the 4096 record FIFO that represents the audio buffer. |
| `audio_playback_en`  | `0x28` | RW  | 1     | Enable audio playback (reading of the audio buffer) when set to 1. No audio playback otherwise.                                                                                                       |
| `audio_buffer_flush` | `0x2C` | W   | 1     | Writing 1 to this register will immediately clear the audio buffer.                                                                                                                                   |
| `audio_buffer_fill`  | `0x30` | R   | 12    | The current fill level of the audio buffer. The buffer is full when set to `0xFFF`. |

# Input

## CSR

Base address (`MAIN` block): `0xF000_1000`

Input data is directly exposed through read registers exactly how they are exposed through APF. No interrupts are available at this time; you must loop and watch for changes in inputs yourself (just like on old consoles).

| Name | Offset | Dir | Width | Description |
| ---- | ------ | --- | ----- | ----------- |
|      |        |     |       |             |

# Interfaces

Connections to the link port and cartridge slot will be coming soon. This will probably change the register offsets defined above.