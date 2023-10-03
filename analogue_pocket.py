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
        self.rst       = Signal()
        self.cd_sys    = ClockDomain()
        self.cd_sys_ps = ClockDomain()

        # # #

        # Clk / Rst
        clk74 = platform.request("clk74a")

        # PLL
        self.pll = pll = CycloneVPLL()
        self.comb += pll.reset.eq(self.rst)
        pll.register_clkin(clk74, 74.25e6)
        pll.create_clkout(self.cd_sys,    sys_clk_freq)
        pll.create_clkout(self.cd_sys_ps, sys_clk_freq, phase=90)

        # SDRAM clock
        sdram_clk = ClockSignal("sys_ps")
        # self.specials += DDROutput(1, 0, platform.request("sdram_clock"), sdram_clk)

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
    def __init__(self, sys_clk_freq=50e6, **kwargs):
        platform = analogue_pocket.Platform()

        # CRG --------------------------------------------------------------------------------------
        self.crg = _CRG(platform, sys_clk_freq)

        # SoCCore ----------------------------------------------------------------------------------
        SoCCore.__init__(self, platform, sys_clk_freq, ident="LiteX SoC on Analog Pocket", **kwargs)

        # UARTBone

        # self.add_uart(name="uart2", uart_name="cart_serial", baudrate=115200)
        # self.add_uart(name="jtag_uart", uart_name="jtag_uart", baudrate=115200, fifo_depth=16)

        # SDR SDRAM --------------------------------------------------------------------------------
        # if not self.integrated_main_ram_size:
        #     self.sdrphy = GENSDRPHY(platform.request("sdram"), sys_clk_freq)
        #     self.add_sdram("sdram",
        #         phy           = self.sdrphy,
        #         module        = AS4C32M16(sys_clk_freq, "1:1"),
        #         l2_cache_size = kwargs.get("l2_size", 8192)
        #     )

# Build --------------------------------------------------------------------------------------------

def main():
    from litex.build.parser import LiteXArgumentParser
    parser = LiteXArgumentParser(platform=analogue_pocket.Platform, description="LiteX SoC on Analog Pocket.")
    parser.add_target_argument("--sys-clk-freq", default=50e6, type=float, help="System clock frequency.")
    args = parser.parse_args()

    soc = BaseSoC(
        sys_clk_freq = args.sys_clk_freq,
        # Match up with Rust compiler target
        cpu_variant = "imac",
        **parser.soc_argdict
    )
    builder = Builder(soc, **parser.builder_argdict)
    if args.build:
        builder.build(**parser.toolchain_argdict)

    if args.load:
        prog = soc.platform.create_programmer()
        prog.load_bitstream(builder.get_bitstream_filename(mode="sram").replace(".sof", ".rbf"))

if __name__ == "__main__":
    main()
