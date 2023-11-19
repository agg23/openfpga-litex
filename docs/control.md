It is very important that you remain aware that register locations and offsets can change between hardware revisions, and instead you should be using the `SVD` file to derive constants to these registers.

# Audio

The system provides audio via a write only audio buffer of 4096 elements. The Pocket accepts solely 48khz `i16` audio, thus the exposed write register takes the form of `{i16, i16}`, or a single 32 bit word containing both channels. This is one sample at 48khz

## CSR

Base address (`MAIN` block): `0xF000_1000` + `0x50`

| Name                 | Offset | Dir | Width | Description                                                                                                                                                                                           |
| -------------------- | ------ | --- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `audio_out`          | `0x0` | W   | 32    | The entrypoint to the audio buffer. Write two 16 bit signed values (for the left and right audio channels) here. This will push one value into the 4096 record FIFO that represents the audio buffer. |
| `audio_playback_en`  | `0x4` | RW  | 1     | Enable audio playback (reading of the audio buffer) when set to 1. No audio playback otherwise.                                                                                                       |
| `audio_buffer_flush` | `0x8` | W   | 1     | Writing 1 to this register will immediately clear the audio buffer.                                                                                                                                   |
| `audio_buffer_fill`  | `0xC` | R   | 12    | The current fill level of the audio buffer. The buffer is full when set to `0xFFF`.                                                                                                                   |

# Bridge

The main control mechanism from the user's core (the RISC-V soft-processor) to the host hardware, PIC, and scaler FPGA. Of primary relevance to this system is the file IO processes, which are exposed here:

**NOTE:** Write (to SD card) mechanism is currently not exposed, and will come in a future release.

## File API

Base address (`MAIN` block): `0xF000_1000` + `0x30`

### Common

| Name                     | Offset | Dir | Width | Description                                                                                                                                                                                                           |
| ------------------------ | ------ | --- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bridge_slot_id`         | `0x4`  | RW  | 16    | The slot ID defined in `data.json` for the desired asset/slot.                                                                                                                                                        |
| `bridge_data_offset`     | `0x8`  | RW  | 32    | The offset from the start of the asset in the selected data slot to operate on.                                                                                                                                       |
| `bridge_length`          | `0xC`  | RW  | 32    | The length of data to transfer as part of this bridge operation. A length of `0xFFFFFFFF` will request the entire file (NOTE: As of Pocket firmware 1.1, this is bugged, and you just request the file size instead). |
| `ram_data_address`       | `0x10` | RW  | 32    | The address of RISC-V RAM to be manipulated in this operation. It is either the first write address for a read request, or the first read address for a write request.                                                |
| `bridge_file_size`       | `0x14` | R   | 32    | The file size on disk of the current selected asset in slot `bridge_slot_id`.                                                                                                                                         |
| `bridge_status`          | `0x18` | R   | 1     | Indicates when the bridge is currently transferring a file. 1 when transferring, 0 otherwise. Clears its value on read.                                                                                               |
| `bridge_current_address` | `0x1C` | R   | 32    | The current address the bridge is operating on. Can be used to show a progress bar/estimate time until completion.                                                                                                    |


### Reading

| Name                  | Offset | Dir | Width | Description                                                                                               |
| --------------------- | ------ | --- | ----- | --------------------------------------------------------------------------------------------------------- |
| `bridge_request_read` | `0x0`  | R   | 1     | Writing 1 to this register will trigger a read request with the contents of the other registers set here. |

### Writing

**TODO:** Currently unimplemented

# Internals

## Control

Base address (`CTRL` block): `0xF000_0000` + `0x0`

| Name    | Offset | Dir | Width | Description                                             |
| ------- | ------ | --- | ----- | ------------------------------------------------------- |
| `reset` | `0x0`  | W   | 2     | High bit resets the CPU. Low bit resets the entire SoC. |


## Timer0

Provides a cycle counter timer (trigger after X cycles) and a global cycle count.

### CSR

Base address (`TIMER0` block): `0xF000_2000` + `0x0`

**NOTE:** These registers marked "TODO" may have documentation in the SVD file.

| Name             | Offset | Dir | Width | Description                                                |
| ---------------- | ------ | --- | ----- | ---------------------------------------------------------- |
| `load`           | `0x0`  | RW  | 32    | TODO: Unknown                                              |
| `reload`         | `0x4`  | RW  | 32    | TODO: Unknown                                              |
| `en`             | `0x8`  | RW  | 1     | Enable the timer                                           |
| `update_value`   | `0xC`  | W   | 1     | TODO: Unknown                                              |
| `value`          | `0x10` | R   | 32    | TODO: Unknown                                              |
| `ev_status`      | `0x14` | R   | 1     | TODO: Unknown                                              |
| `ev_pending`     | `0x18` | R   | 1     | TODO: Unknown                                              |
| `ev_enable`      | `0x1C` | RW  | 1     | TODO: Unknown                                              |
| `uptime_latch`   | `0x20` | W   | 1     | Write 1 to latch uptime into `uptime_cycles1-2` registers. |
| `uptime_cycles1` | `0x24` | R   | 32    | High bits of latched uptime cycle count.                   |
| `uptime_cycles0` | `0x28` | R   | 32    | Low bits of latched uptime cycle count. |

## UART

There are two variants of the core, each supporting one type of UART (a limitation of how the core was built). You can use the Analogue Pocket Devkit cart, or the JTAG adapter used to program the Pocket. UART is bidirectional, and allows for upload of a new program on start/reset.

The dev cart baud rate is set to 2,000,000 bps.

### CSR

Base address (`UART` block): `0xF000_2800` + `0x0`

| Name         | Offset | Dir | Width | Description                                 |
| ------------ | ------ | --- | ----- | ------------------------------------------- |
| `rxtx`       | `0x0`  | RW  | 8     | The current read/write value from the UART. |
| `txfull`     | `0x4`  | R   | 1     | Indicates if transmit FIFO is full.         |
| `rxempty`    | `0x8`  | R   | 1     | Indicates if receive FIFO is empty.         |
| `ev_status`  | `0xC`  | R   | 2     | TODO: Unknown                               |
| `ev_pending` | `0x10` | R   | 2     | TODO: Unknown                               |
| `ev_enable`  | `0x14` | RW  | 2     | TODO: Unknown                               |
| `txempty`    | `0x18` | R   | 1     | Indicates if transmit FIFO is empty         |
| `rxfull`     | `0x1C` | R   | 1     | Indicates if receive FIFO is empty          |

## Video

### CSR

Base address (`VIDEO_FRAMEBUFFER` block): `0xF000_3000` + `0x0`

| Name         | Offset | Dir | Width | Description                                                                                                          |
| ------------ | ------ | --- | ----- | -------------------------------------------------------------------------------------------------------------------- |
| `dma_base`   | `0x0`  | RW  | 32    | The base address of the framebuffer. Defaults to `0x40C0_0000`.                                                      |
| `dma_length` | `0x4`  | RW  | 32    | The number of bytes read per "frame" of the framebuffer. Defaults to `0x1_F2C0`.                                     |
| `dma_enable` | `0x8`  | RW  | 1     | Enable framebuffer DMA when set to 1. Disabling DMA can be used to decrease bus contention for faster memory access. |
| `dma_done`   | `0xC`  | R   | 1     | Indicates completion of a DMA when 1.                                                                                |
| `dma_loop`   | `0x10` | RW  | 1     | When 1, DMA will continue to loop when it completes a frame. When 0, it stops.                                       |
| `dma_offset` | `0x14` | RW  | 32    | The current offset of the DMA into a frame. This can be used to restart drawing partially through a frame.           |

Base address (`VIDEO_FRAMEBUFFER_VTG` block): `0xF000_3800` + `0x0`

| Name     | Offset | Dir | Width | Description |
| -------- | ------ | --- | ----- | ----------- |
| `enable` | `0x0`  | RW  | 1     | When 1, video sync signals will be produced. When 0, video generation halts. |


# IO - Interfaces

Connections to the link port and cartridge slot will be coming soon. This will probably change the register offsets defined above.

# IO - User

## CSR

Base address (`MAIN` block): `0xF000_1000` + `0x0`

Input data is directly exposed through read registers exactly how they are exposed through APF. No interrupts are available at this time; you must loop and watch for changes in inputs yourself (just like on old consoles).

| Name             | Offset                         | Dir | Width | Description                                                                                                                                        |
| ---------------- | ------------------------------ | --- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CONT[1-4]_KEY`  | `0x0`, `0x4`, `0x8`, `0xC`     | R   | 32    | Controller 1-4 inputs. See associated bitmap                                                                                                       |
| `CONT[1-4]_JOY`  | `0x10`, `0x14`, `0x18`, `0x1C` | R   | 32    | Controller 1-4 joystick values. See associated bitmap                                                                                              |
| `CONT[1-4]_TRIG` | `0x20`, `0x24`, `0x28`, `0x2C` | R   | 16    | Controller 1-4 trigger values. Values are binary on Pocket (`0 and 0xFFFF`), and analog on controllers with analog triggers. See associated bitmap |

## Controller Bitmap

| Bit Indexes | Name            |
| ----------- | --------------- |
| 0           | `dpad_up`       |
| 1           | `dpad_down`     |
| 2           | `dpad_left`     |
| 3           | `dpad_right`    |
| 4           | `face_a`        |
| 5           | `face_b`        |
| 6           | `face_x`        |
| 7           | `face_y`        |
| 8           | `trig_l1`       |
| 9           | `trig_r1`       |
| 10          | `trig_l2`       |
| 11          | `trig_r2`       |
| 12          | `trig_l3`       |
| 13          | `trig_r3`       |
| 14          | `face_select`   |
| 15          | `face_start`    |
| [28:16]     | _unused_        |
| [31:29]     | controller type |

## Joystick Bitmap

| Bit Indexes | Name       |
| ----------- | ---------- |
| [7:0]       | `lstick_x` |
| [15:8]      | `lstick_y` |
| [23:16]     | `rstick_x` |
| [31:24]     | `rstick_y` |

## Trigger Bitmap

| Bit Indexes | Name    |
| ----------- | ------- |
| [7:0]       | `ltrig` |
| [15:8]      | `rtrig` |
