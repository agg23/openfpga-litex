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

from litex.soc.cores.video import VideoVGAPHY
from litex.soc.interconnect import wishbone

from migen import *

from litex.gen import *

import verilog_platform as analogue_pocket

from litex.soc.integration.soc_core import *
from litex.soc.integration.builder import *

from litex.build.io import DDROutput

from litex.soc.cores.clock import CycloneVPLL

from litedram.modules import AS4C32M16
from litedram.phy import GENSDRPHY

# CRG ----------------------------------------------------------------------------------------------

class _CRG(LiteXModule):
    def __init__(self, platform: analogue_pocket.Platform, sys_clk_freq):
        # `rst` is a magic CRG signal that is automatically wired to the output of the SoC reset
        # self.rst          = Signal()
        # LiteX expects a `sys` clock domain, so we can't rename it
        self.cd_sys       = ClockDomain()
        self.cd_sys_90deg = ClockDomain()
        self.cd_vid       = ClockDomain()

        clk_sys = platform.request("clk_sys")
        self.comb += self.cd_sys.clk.eq(clk_sys)

        clk_sys_90deg = platform.request("clk_sys_90deg")
        self.comb += self.cd_sys_90deg.clk.eq(clk_sys_90deg)

        clk_vid = platform.request("clk_vid")
        self.comb += self.cd_vid.clk.eq(clk_vid)

        # # #

        # SDRAM clock
        sdram_clk = clk_sys_90deg
        self.specials += DDROutput(1, 0, platform.request("sdram_clock"), sdram_clk)

        # UART
        # cart = platform.request("cart")
        # self.comb += cart.tran_bank0_dir.eq(1)

        # self.comb += cart.tran_pin31_dir.eq(0)

        # platform.add_extension([
        #     ("cart_serial", 0, 
        #         Subsignal("tx", cart.tran_bank0[2]), 
        #         Subsignal("rx", cart.tran_pin31)
        #     )])

# BaseSoC ------------------------------------------------------------------------------------------

class BaseSoC(SoCCore):
    def __init__(self, sys_clk_freq, **kwargs):
        platform = analogue_pocket.Platform()

        # CRG --------------------------------------------------------------------------------------
        self.crg = _CRG(platform, sys_clk_freq)

        # SoCCore ----------------------------------------------------------------------------------
        SoCCore.__init__(self, platform, sys_clk_freq, ident="LiteX SoC on Analog Pocket", **kwargs)

        reset = platform.request("reset")
        self.comb += self.cpu.reset.eq(reset)

        # UARTBone

        # self.add_uart(name="uart2", uart_name="cart_serial", baudrate=115200)
        # self.add_uart(name="jtag_uart", uart_name="jtag_uart", baudrate=115200, fifo_depth=16)

        # SDR SDRAM --------------------------------------------------------------------------------
        if not self.integrated_main_ram_size:
            self.sdrphy = GENSDRPHY(platform.request("sdram"), sys_clk_freq)
            self.add_sdram("sdram",
                phy           = self.sdrphy,
                module        = AS4C32M16(sys_clk_freq, "1:1"),
                l2_cache_size = kwargs.get("l2_size", 8192)
            )

        # This only works with modifications to vendor/litex/litex/soc/cores/video.py to remove the SDR and DDR outputs
        self.submodules.videophy = VideoVGAPHY(platform.request("vga"))
        self.add_video_framebuffer(phy=self.videophy, timings="320x200@60Hz", format="rgb565", clock_domain="vid")
        # self.add_video_terminal(phy=self.videophy, timings="320x200@60Hz", clock_domain="vid")

        # testSlave = wishbone.Interface()
        
        # self.bus.add_slave("test", testSlave)

# Build --------------------------------------------------------------------------------------------

def main():
    from litex.build.parser import LiteXArgumentParser
    parser = LiteXArgumentParser(platform=analogue_pocket.Platform, description="LiteX SoC on Analog Pocket.")
    parser.add_target_argument("--sys-clk-freq", default=51_600_000, type=float, help="System clock frequency.")
    args = parser.parse_args()

    soc_args = parser.soc_argdict
    soc_args["uart_baudrate"] = 2000000

    soc = BaseSoC(
        sys_clk_freq = args.sys_clk_freq,
        # Match up with Rust compiler target
        cpu_variant = "imac",
        **soc_args
    )
    builder_args = parser.builder_argdict
    builder_args["csr_svd"] = "pocket.svd"
    builder = Builder(soc, **builder_args)
    if args.build:
        builder.build(**parser.toolchain_argdict)

    if args.load:
        prog = soc.platform.create_programmer()
        prog.load_bitstream(builder.get_bitstream_filename(mode="sram").replace(".sof", ".rbf"))

if __name__ == "__main__":
    main()
