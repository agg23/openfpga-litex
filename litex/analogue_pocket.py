#!/usr/bin/env python3

#
# This file is part of LiteX-Boards.
#
# Copyright (c) 2023 Florent Kermarrec <florent@enjoy-digital.fr>
# SPDX-License-Identifier: BSD-2-Clause

# ./analog_pocket.py --uart-name=jtag_uart --build --load
# litex_term jtag --jtag-config=openocd_usb_blaster.cfg

# Set up the import paths for the LiteX packages
import vendor
from csr import APFID, APFRTC, APFAudio, APFBridge, APFInput, APFInteract, APFVideo
from litex.soc.cores.uart import UART
from replaced_components import (
    AS4C32M16Pocket,
    HalfRateGENSDRAMPocketPHY,
    UARTPHYMultiplexer,
    VideoPocketPHY,
)
from litex.soc.integration.soc import SoCRegion

from litex.soc.interconnect import wishbone

from migen import *

from litex.gen import *

import verilog_platform as analogue_pocket

from litex.soc.integration.soc_core import *
from litex.soc.integration.builder import *

from litex.build.io import DDROutput

# CRG ----------------------------------------------------------------------------------------------

# CLOCK_SPEED = 66.12e6
CLOCK_SPEED = 57.12e6


class _CRG(LiteXModule):
    def __init__(self, platform: analogue_pocket.Platform):
        # `rst` is a magic CRG signal that is automatically wired to the output of the SoC reset
        self.rst = Signal()
        # LiteX expects a `sys` clock domain, so we can't rename it
        self.cd_sys = ClockDomain()
        # LiteX also expects a `sys2x` clock domain when using double rate SDRAM, and it can't be renamed
        self.cd_sys2x = ClockDomain()
        self.cd_sys2x_90deg = ClockDomain()
        self.cd_vid = ClockDomain()

        reset_pin = platform.request("reset")

        clk_sys = platform.request("clk_sys")
        self.comb += self.cd_sys.clk.eq(clk_sys)
        self.comb += self.cd_sys.rst.eq(self.rst | reset_pin)

        clk_sys2x = platform.request("clk_sys2x")
        self.comb += self.cd_sys2x.clk.eq(clk_sys2x)
        self.comb += self.cd_sys2x.rst.eq(self.rst | reset_pin)

        clk_sys2x_90deg = platform.request("clk_sys2x_90deg")
        self.comb += self.cd_sys2x_90deg.clk.eq(clk_sys2x_90deg)

        clk_vid = platform.request("clk_vid")
        self.comb += self.cd_vid.clk.eq(clk_vid)

        # # #

        # SDRAM clock
        sdram_clk = clk_sys2x_90deg
        self.specials += DDROutput(1, 0, platform.request("sdram_clock"), sdram_clk)


# BaseSoC ------------------------------------------------------------------------------------------


class BaseSoC(SoCCore):
    def __init__(self, sys_clk_freq, **kwargs):
        platform = analogue_pocket.Platform()

        # CRG --------------------------------------------------------------------------------------
        self.crg = _CRG(platform)

        # SoCCore ----------------------------------------------------------------------------------
        SoCCore.__init__(
            self, platform, sys_clk_freq, ident="LiteX SoC on Analog Pocket", **kwargs
        )

        self.add_constant("DEPLOYMENT_PLATFORM", "openfpga")
        self.add_constant("DEPLOYMENT_TARGET", "pocket")

        # Allow booting from the first address in SDRAM
        self.add_constant("ROM_BOOT_ADDRESS", 0x40000000)
        # self.add_constant("SDRAM_TEST_DISABLE", 1)

        # SDR SDRAM --------------------------------------------------------------------------------
        if not self.integrated_main_ram_size:
            self.sdrphy = HalfRateGENSDRAMPocketPHY(
                platform.request("sdram"), sys_clk_freq
            )
            self.add_sdram(
                "sdram",
                phy=self.sdrphy,
                module=AS4C32M16Pocket(sys_clk_freq, "1:2"),
                # l2_cache_size = kwargs.get("l2_size", 8192)
                # Disable L2 as it seems to not being used for reads, and it is causing 0x2000 bytes to be not written
                # on file read
                l2_cache_size=0,
            )

        self.submodules.videophy = VideoPocketPHY(platform.request("vga"))
        # 57.12 MHz
        timings = {
            "pix_clk": CLOCK_SPEED / 10,
            "h_active": 266,
            "h_blanking": 74,  # Max 340
            "h_sync_offset": 8,
            "h_sync_width": 32,
            "v_active": 240,
            "v_blanking": 40,  # Max 280
            "v_sync_offset": 1,
            "v_sync_width": 8,
        }
        
        self.add_video_framebuffer(
            phy=self.videophy,
            timings=[
                "266x240@60Hz",
                timings,
            ],
            format="rgb565",
            clock_domain="vid",
        )

        self.add_constant("MAX_DISPLAY_WIDTH", timings["h_active"])
        self.add_constant("MAX_DISPLAY_HEIGHT", timings["v_active"])

        # CSR definitions --------------------------------------------------------------------------
        self.add_module("apf_audio", APFAudio(platform))
        self.add_module("apf_bridge", APFBridge(platform))
        self.add_module("apf_id", APFID(platform))
        self.add_module("apf_input", APFInput(platform))
        self.add_module("apf_interact", APFInteract(platform))
        self.add_module("apf_rtc", APFRTC(platform))
        self.add_module("apf_video", APFVideo(self))

        self.add_uart(platform)

        example_slave = wishbone.Interface()
        example_slave_region = SoCRegion(0x8000_0000, 0x10_0000, cached=False)

        self.bus.add_slave("example_slave", example_slave, example_slave_region)

        # For some reason this doesn't make the comb assignments itself?
        # Master, because the internal wishbone is a slave, and the Verilog one is "master"
        self.comb += example_slave.connect_to_pads(
            platform.request("wishbone"), mode="master"
        )

        apf_bridge_master = wishbone.Interface()

        self.bus.add_master("apf_bridge_master", apf_bridge_master)

        self.comb += apf_bridge_master.connect_to_pads(
            platform.request("wishbone_master"), mode="slave"
        )

    def add_uart(self, platform):
        baudrate = 2000000

        uart_pads = platform.request("serial", loose=True)
        uart_phy = None
        uart = None
        fifo_depth = 16
        uart_kwargs = {
            "tx_fifo_depth": fifo_depth,
            "rx_fifo_depth": fifo_depth,
        }

        # JTAG UART
        from litex.soc.cores.jtag import JTAGPHY

        self.cd_sys_jtag = ClockDomain()
        self.comb += self.cd_sys_jtag.clk.eq(ClockSignal("sys"))
        jtag_uart_phy = JTAGPHY(
            device=platform.device, clock_domain="sys_jtag", platform=platform
        )

        # Physical UART
        from litex.soc.cores.uart import UARTPHY

        phys_uart_phy = UARTPHY(
            uart_pads, clk_freq=self.sys_clk_freq, baudrate=baudrate
        )

        uart_phy = UARTPHYMultiplexer([jtag_uart_phy, phys_uart_phy])

        use_jtag = platform.request("use_jtag")

        self.comb += uart_phy.sel.eq(~use_jtag)

        uart = UART(uart_phy, **uart_kwargs)

        self.submodules.jtag_uart_phy = jtag_uart_phy
        self.submodules.phys_uart_phy = phys_uart_phy
        self.submodules.combined_uart_phy = uart_phy
        self.submodules.uart = uart

        # IRQ.
        if self.irq.enabled:
            self.irq.add("uart", use_loc_if_exists=True)
        else:
            self.add_constant("UART_POLLING")


# Build --------------------------------------------------------------------------------------------


def rewrite_output_variables(root_dir: str, generated_dir: str):
    filename = os.path.join(generated_dir, "variables.mak")

    print(f"Rewriting {filename} from {root_dir}")

    if os.path.exists(filename):
        from fileinput import FileInput

        with FileInput(filename, inplace=True, backup=".bak") as file:
            for line in file:
                print(line.replace(root_dir, "$(LITEX_ROOT_DIRECTORY)"), end="")
    else:
        print("Cannot find `variables.mak`")


def main():
    from litex.build.parser import LiteXArgumentParser
    import sys

    # LiteX directly reaches into sys.argv multiple times, so we have to inject all of our changed arguments at top level
    # Match up with Rust compiler target with FPU and RVC
    sys.argv.extend(
        [
            "--cpu-type=vexriscv_smp",
            "--with-fpu",
            "--with-rvc",
            # UART is manually added
            "--no-uart",
            "--timer-uptime",
        ]
    )

    parser = LiteXArgumentParser(
        platform=analogue_pocket.Platform, description="LiteX SoC on Analog Pocket."
    )
    parser.add_target_argument(
        "--sys-clk-freq",
        default=CLOCK_SPEED,
        type=float,
        help="System clock frequency.",
    )
    args = parser.parse_args()

    soc_args = parser.soc_argdict

    soc = BaseSoC(sys_clk_freq=args.sys_clk_freq, **soc_args)
    builder_args = parser.builder_argdict
    builder_args["csr_svd"] = "pocket.svd"
    builder = Builder(soc, **builder_args)

    root_dir = os.path.abspath("")
    generated_dir = builder.generated_dir

    if args.build:
        builder.build(**parser.toolchain_argdict)

    if args.load:
        prog = soc.platform.create_programmer()
        prog.load_bitstream(
            builder.get_bitstream_filename(mode="sram").replace(".sof", ".rbf")
        )

    # Make `variables.mak` use relative paths off of `LITEX_ROOT_DIRECTORY`
    rewrite_output_variables(root_dir, generated_dir)


if __name__ == "__main__":
    main()
