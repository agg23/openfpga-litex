# BSD 2-Clause License
#
# The LiteX framework provides a convenient and efficient infrastructure to create
# FPGA Cores/SoCs, to explore various digital design architectures and create full
# FPGA based systems.
#
# Unless otherwise noted, LiteX is copyright (C) 2012-2022 Enjoy-Digital & LiteX developers.
# Unless otherwise noted, MiSoC is copyright (C) 2012-2015 Enjoy-Digital.
# Unless otherwise noted, MiSoC is copyright (C) 2007-2015 M-Labs Ltd.
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without modification,
# are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
# ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
# ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
# (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
# LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
# ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
# SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

# Other authors retain ownership of their contributions. If a submission can
# reasonably be considered independently copyrightable, it's yours and we
# encourage you to claim it with appropriate copyright notices. This submission
# then falls under the "otherwise noted" category. All submissions are strongly
# encouraged to use the two-clause BSD license reproduced above.

from litedram.common import *
from litedram.phy.dfi import *
from litex.gen.fhdl.module import LiteXModule
from litex.soc.interconnect import stream

from migen import *

from migen.fhdl.specials import Tristate

# Internal Pocket SDR PHY ---------------------------------------------------------------------------

class GENSDRAMPocketPHY(Module):
    # This is copied and modified from GENSDRPHY in `litedram/gensdrphy.py`
    def __init__(self, pads, sys_clk_freq=100e6, cl=None):
        pads        = PHYPadsCombiner(pads)
        addressbits = len(pads.a)
        bankbits    = len(pads.ba)
        nranks      = 1 if not hasattr(pads, "cs_n") else len(pads.cs_n)
        databits    = len(pads.dq)
        assert databits%8 == 0

        # Parameters -------------------------------------------------------------------------------
        cl = get_default_cl(memtype="SDR", tck=1/sys_clk_freq) if cl is None else cl

        # PHY settings -----------------------------------------------------------------------------
        self.settings = PhySettings(
            phytype       = "GENSDRPHY",
            memtype       = "SDR",
            databits      = databits,
            dfi_databits  = databits,
            nranks        = nranks,
            nphases       = 1,
            rdphase       = 0,
            wrphase       = 0,
            cl            = cl,
            read_latency  = cl + 1,
            write_latency = 0
        )

        # DFI Interface ----------------------------------------------------------------------------
        self.dfi = dfi = Interface(addressbits, bankbits, nranks, databits)

        # # #

        # Iterate on pads groups -------------------------------------------------------------------
        for pads_group in range(len(pads.groups)):
            pads.sel_group(pads_group)


            # Commands -----------------------------------------------------------------------------
            commands = {
                # Pad name: (DFI name,   Pad type (required or optional))
                "cs_n"    : ("cs_n",    "optional"),
                "a"       : ("address", "required"),
                "ba"      : ("bank"   , "required"),
                "ras_n"   : ("ras_n"  , "required"),
                "cas_n"   : ("cas_n"  , "required"),
                "we_n"    : ("we_n"   , "required"),
                "cke"     : ("cke"    , "optional"),
            }
            for pad_name, (dfi_name, pad_type) in commands.items():
                pad = getattr(pads, pad_name, None)
                if (pad is None):
                    if (pad_type == "required"):
                        raise ValueError(f"DRAM pad {pad_name} required but not found in pads.")
                    continue
                for i in range(len(pad)):
                    self.sync += pad[i].eq(getattr(dfi.p0, dfi_name)[i])

        # DQ/DM Data Path --------------------------------------------------------------------------
        for i in range(len(pads.dq)):
            input_reg = Signal()
            input_reg2 = Signal()
            output_en_reg = Signal()
            output_reg = Signal()

            self.sync += [
                # One cycle read delay
                dfi.p0.rddata[i].eq(input_reg2),
                input_reg2.eq(input_reg),
                output_en_reg.eq(dfi.p0.wrdata_en),
                output_reg.eq(dfi.p0.wrdata[i]),
            ]

            # IO, O, OE, I
            self.specials += Tristate(
                pads.dq[i],
                output_reg,
                output_en_reg,
                input_reg,
            )
            
        if hasattr(pads, "dm"):
            for i in range(len(pads.dm)):
                self.sync += pads.dm[i].eq(dfi.p0.wrdata_en & dfi.p0.wrdata_mask[i])

        # DQ/DM Control Path -----------------------------------------------------------------------
        rddata_en = Signal(self.settings.read_latency)
        self.sync += rddata_en.eq(Cat(dfi.p0.rddata_en, rddata_en))
        self.sync += dfi.p0.rddata_valid.eq(rddata_en[-1])

# Half-rate Pocket SDR PHY -------------------------------------------------------------------------

class HalfRateGENSDRAMPocketPHY(Module):
    # This is copied and modified from HalfRateGENSDRPHY in `litedram/gensdrphy.py`
    def __init__(self, pads, sys_clk_freq=100e6, cl=None):
        pads        = PHYPadsCombiner(pads)
        addressbits = len(pads.a)
        bankbits    = len(pads.ba)
        nranks      = 1 if not hasattr(pads, "cs_n") else len(pads.cs_n)
        databits    = len(pads.dq)
        nphases     = 2


        # Parameters -------------------------------------------------------------------------------
        cl = get_default_cl(memtype="SDR", tck=1/sys_clk_freq) if cl is None else cl
        cl = 3

        # FullRate PHY -----------------------------------------------------------------------------
        full_rate_phy = GENSDRAMPocketPHY(pads, 2*sys_clk_freq, cl)
        self.submodules += ClockDomainsRenamer("sys2x")(full_rate_phy)

        # Clocking ---------------------------------------------------------------------------------
        # Select active sys2x phase:
        #  sys_clk   ----____----____
        #  sys2x_clk --__--__--__--__
        #  phase_sel 0   1   0   1
        phase_sel   = Signal()
        phase_sys   = Signal()
        phase_sys2x = Signal()
        self.sync       += phase_sys.eq(phase_sys2x)
        self.sync.sys2x += phase_sys2x.eq(~phase_sel)
        self.sync.sys2x += phase_sel.eq(~phase_sel & (phase_sys2x ^ phase_sys))

        # PHY settings -----------------------------------------------------------------------------
        self.settings = PhySettings(
            phytype       = "HalfRateGENSDRPHY",
            memtype       = "SDR",
            databits      = databits,
            dfi_databits  = databits,
            nranks        = nranks,
            nphases       = nphases,
            rdphase       = 0,
            wrphase       = 0,
            cl            = cl,
            read_latency  = full_rate_phy.settings.read_latency//2 + 1,
            write_latency = 0
        )

        # DFI adaptation ---------------------------------------------------------------------------
        self.dfi = dfi = Interface(addressbits, bankbits, nranks, databits, nphases)
        self.comb += Case(phase_sel, {
            0: dfi.phases[0].connect(full_rate_phy.dfi.phases[0], omit={"rddata", "rddata_valid", "wrdata_en"}),
            1: dfi.phases[1].connect(full_rate_phy.dfi.phases[0], omit={"rddata", "rddata_valid", "wrdata_en"}),
        })

        # Write Datapath
        wr_data_en   = dfi.phases[self.settings.wrphase].wrdata_en & (phase_sel == 0)
        wr_data_en_d = Signal()
        self.sync.sys2x += wr_data_en_d.eq(wr_data_en)
        self.comb += full_rate_phy.dfi.phases[0].wrdata_en.eq(wr_data_en | wr_data_en_d)

        # Read Datapath
        rddata_d       = Signal(databits)
        self.sync.sys2x += rddata_d.eq(full_rate_phy.dfi.phases[0].rddata)
        self.comb += [
            dfi.phases[0].rddata.eq(rddata_d),
            dfi.phases[0].rddata_valid.eq(full_rate_phy.dfi.phases[0].rddata_valid),
            dfi.phases[1].rddata.eq(full_rate_phy.dfi.phases[0].rddata),
            dfi.phases[1].rddata_valid.eq(full_rate_phy.dfi.phases[0].rddata_valid),
        ]

# VideoPHY -----------------------------------------------------------------------------------------

class VideoPocketPHY(LiteXModule):
    # This is copied and modified from VideoGenericPHY in `video.py`
    def __init__(self, pads, clock_domain="sys", with_clk_ddr_output=True):
        video_data_layout = [
            # Synchronization signals.
            ("hsync", 1),
            ("vsync", 1),
            ("de",    1),
            # Data signals.
            ("r",     8),
            ("g",     8),
            ("b",     8),
        ]

        self.sink = sink = stream.Endpoint(video_data_layout)

        # # #

        # Always ack Sink, no backpressure.
        self.comb += sink.ready.eq(1)
        # Drive Controls.
        self.comb += pads.de.eq(sink.de)
        self.comb += pads.hsync.eq(sink.hsync)
        self.comb += pads.vsync.eq(sink.vsync)

        # Drive Datas.
        cbits  = len(pads.r)
        cshift = (8 - cbits)
        for i in range(cbits):
            # VGA monitors interpret minimum value as black so ensure data is set to 0 during blanking.
            self.comb += pads.r[i].eq(sink.r[cshift + i] & sink.de)

        cbits = len(pads.g)
        cshift = (8 - cbits)
        for i in range(cbits):
            self.comb += pads.g[i].eq(sink.g[cshift + i] & sink.de)

        cbits = len(pads.b)
        cshift = (8 - cbits)
        for i in range(cbits):
            self.comb += pads.b[i].eq(sink.b[cshift + i] & sink.de)
