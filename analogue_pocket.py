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
from csr import APFID, APFRTC, APFAudio, APFBridge, APFInput, APFVideo
from replaced_components import AS4C32M16Pocket, HalfRateGENSDRAMPocketPHY, VideoPocketPHY
from litex.soc.integration.soc import SoCRegion
from litex.soc.interconnect.csr import CSR, CSRStatus, CSRStorage

from litex.soc.interconnect import wishbone

from migen import *

from litex.gen import *

import verilog_platform as analogue_pocket

from litex.soc.integration.soc_core import *
from litex.soc.integration.builder import *

from litex.build.io import DDROutput

from litedram.modules import AS4C32M16
from litedram.phy.gensdrphy import HalfRateGENSDRPHY

# CRG ----------------------------------------------------------------------------------------------

# CLOCK_SPEED = 66.12e6
CLOCK_SPEED = 57.12e6

class _CRG(LiteXModule):
    def __init__(self, platform: analogue_pocket.Platform):
        # `rst` is a magic CRG signal that is automatically wired to the output of the SoC reset
        self.rst          = Signal()
        # LiteX expects a `sys` clock domain, so we can't rename it
        self.cd_sys         = ClockDomain()
        # LiteX also expects a `sys2x` clock domain when using double rate SDRAM, and it can't be renamed
        self.cd_sys2x       = ClockDomain()
        self.cd_sys2x_90deg = ClockDomain()
        self.cd_vid         = ClockDomain()

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
        SoCCore.__init__(self, platform, sys_clk_freq, ident="LiteX SoC on Analog Pocket", **kwargs)

        # Allow booting from the first address in SDRAM
        self.add_constant("ROM_BOOT_ADDRESS", 0x40000000)
        # self.add_constant("SDRAM_TEST_DISABLE", 1)

        # SDR SDRAM --------------------------------------------------------------------------------
        if not self.integrated_main_ram_size:
            self.sdrphy = HalfRateGENSDRAMPocketPHY(platform.request("sdram"), sys_clk_freq)
            self.add_sdram("sdram",
                phy           = self.sdrphy,
                module        = AS4C32M16Pocket(sys_clk_freq, "1:2"),
                # l2_cache_size = kwargs.get("l2_size", 8192)
                # Disable L2 as it seems to not being used for reads, and it is causing 0x2000 bytes to be not written
                # on file read
                l2_cache_size = 0
            )

        # This only works with modifications to vendor/litex/litex/soc/cores/video.py to remove the SDR and DDR outputs
        self.submodules.videophy = VideoPocketPHY(platform.request("vga"))
        # 57.12 MHz
        self.add_video_framebuffer(phy=self.videophy, timings=[
            "266x240@60Hz",
            {
                "pix_clk"       : CLOCK_SPEED / 10,
                "h_active"      : 266,
                "h_blanking"    : 74, # Max 340
                "h_sync_offset" : 8,
                "h_sync_width"  : 32,
                "v_active"      : 240,
                "v_blanking"    : 40, # Max 280
                "v_sync_offset" : 1,
                "v_sync_width"  : 8,
            }], format="rgb565", clock_domain="vid")

        # CSR definitions --------------------------------------------------------------------------
        self.add_module("apf_audio", APFAudio(platform))
        self.add_module("apf_bridge", APFBridge(platform))
        self.add_module("apf_id", APFID(platform))
        self.add_module("apf_input", APFInput(platform))
        self.add_module("apf_rtc", APFRTC(platform))
        self.add_module("apf_video", APFVideo(self))

        example_slave = wishbone.Interface()
        example_slave_region = SoCRegion(0x8000_0000, 0x10_0000, cached = False)
        
        self.bus.add_slave("example_slave", example_slave, example_slave_region)

        # For some reason this doesn't make the comb assignments itself?
        # Master, because the internal wishbone is a slave, and the Verilog one is "master"
        self.comb += example_slave.connect_to_pads(platform.request("wishbone"), mode="master")

        apf_bridge_master = wishbone.Interface()

        self.bus.add_master("apf_bridge_master", apf_bridge_master)

        self.comb += apf_bridge_master.connect_to_pads(platform.request("wishbone_master"), mode="slave")
    
# Build --------------------------------------------------------------------------------------------

def main():
    from litex.build.parser import LiteXArgumentParser
    import sys
    # LiteX directly reaches into sys.argv multiple times, so we have to inject all of our changed arguments at top level
    # Match up with Rust compiler target with FPU and RVC
    sys.argv.extend(["--cpu-type=vexriscv_smp", "--with-fpu", "--with-rvc", "--uart-baudrate=2000000", "--timer-uptime"])

    # Include if we are building for JTAG, not cart UART
    # Baudrate is ignored for JTAG
    # sys.argv.extend(["--uart-name=jtag_uart"])

    parser = LiteXArgumentParser(platform=analogue_pocket.Platform, description="LiteX SoC on Analog Pocket.")
    parser.add_target_argument("--sys-clk-freq", default=CLOCK_SPEED, type=float, help="System clock frequency.")
    args = parser.parse_args()

    soc_args = parser.soc_argdict

    soc = BaseSoC(
        sys_clk_freq = args.sys_clk_freq,
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
