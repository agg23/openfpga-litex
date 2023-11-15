#
# Based on code in LiteX-Boards
#

from litex.build.altera import common, quartus
from litex.build.altera.platform import AlteraPlatform
from litex.build.generic_platform import *
from litex.build.openfpgaloader import OpenFPGALoader

# Platform -----------------------------------------------------------------------------------------

class Platform(AlteraPlatform):
    default_clk_name   = "clk74a"
    default_clk_period = 1e9/74.25e6

    _supported_toolchains = ["quartus"]

    def __init__(self):
        _io = [
            ("clk_sys", 0, Pins(1)),
            ("clk_sys2x", 0, Pins(1)),
            ("clk_sys2x_90deg", 0, Pins(1)),

            ("clk_vid", 0, Pins(1)),

            ("reset", 0, Pins(1)),

            # UART Pins
            # Automatically connected by internal LiteX UART
            ("serial", 0,
                Subsignal("tx", Pins(1)),
                Subsignal("rx", Pins(1))
            ),

            ("sdram_clock", 0, Pins(1)),
            ("sdram", 0,
                Subsignal("a",     Pins(13)),
                Subsignal("ba",    Pins(2)),
                #Subsignal("cs_n",  Pins("")),
                Subsignal("cke",   Pins(1)),
                Subsignal("ras_n", Pins(1)),
                Subsignal("cas_n", Pins(1)),
                Subsignal("we_n",  Pins(1)),
                Subsignal("dq",    Pins(16)),
                Subsignal("dm",    Pins(2)),
            ),

            ("vga", 0,
                Subsignal("hsync", Pins(1)),
                Subsignal("vsync", Pins(1)),
                Subsignal("de",    Pins(1)),
                Subsignal("r",     Pins(5)),
                Subsignal("g",     Pins(6)),
                Subsignal("b",     Pins(5)),
            ),

            ("wishbone", 0,
                Subsignal("adr",   Pins(30)),
                Subsignal("dat_w", Pins(32)),
                Subsignal("dat_r", Pins(32)),
                Subsignal("sel",   Pins(4)),
                Subsignal("cyc",   Pins(1)),
                Subsignal("stb",   Pins(1)),
                Subsignal("ack",   Pins(1)),
                Subsignal("we",    Pins(1)),
                Subsignal("cti",   Pins(3)),
                Subsignal("bte",   Pins(2)),
                Subsignal("err",   Pins(1)),
            ),

            ("wishbone_master", 0,
                Subsignal("adr",   Pins(30)),
                Subsignal("dat_w", Pins(32)),
                Subsignal("dat_r", Pins(32)),
                Subsignal("sel",   Pins(4)),
                Subsignal("cyc",   Pins(1)),
                Subsignal("stb",   Pins(1)),
                Subsignal("ack",   Pins(1)),
                Subsignal("we",    Pins(1)),
                Subsignal("cti",   Pins(3)),
                Subsignal("bte",   Pins(2)),
                Subsignal("err",   Pins(1)),
            ),

            ("apf_input", 0,
                Subsignal("cont1_key", 0, Pins(32)),
                Subsignal("cont2_key", 0, Pins(32)),
                Subsignal("cont3_key", 0, Pins(32)),
                Subsignal("cont4_key", 0, Pins(32)),

                Subsignal("cont1_joy", 0, Pins(32)),
                Subsignal("cont2_joy", 0, Pins(32)),
                Subsignal("cont3_joy", 0, Pins(32)),
                Subsignal("cont4_joy", 0, Pins(32)),

                Subsignal("cont1_trig", 0, Pins(32)),
                Subsignal("cont2_trig", 0, Pins(32)),
                Subsignal("cont3_trig", 0, Pins(32)),
                Subsignal("cont4_trig", 0, Pins(32)),
            ),

            ("apf_bridge", 0,
                Subsignal("request_read", Pins(1)),

                Subsignal("slot_id", Pins(16)),
                Subsignal("data_offset", Pins(32)),
                # Subsignal("local_address", Pins(32)),
                Subsignal("length", Pins(32)),

                Subsignal("ram_data_address", Pins(32)),

                Subsignal("file_size", Pins(32)),

                Subsignal("current_address", Pins(32)),
                Subsignal("complete_trigger", Pins(1))
            ),

            ("apf_audio", 0,
                Subsignal("bus_out", Pins(32)),
                Subsignal("bus_wr", Pins(1)),

                Subsignal("playback_en", Pins(1)),
                Subsignal("flush", Pins(1)),

                Subsignal("buffer_fill", Pins(12))
            )
        ]
        _connectors = []

        AlteraPlatform.__init__(self, "5CEBA4F23C8", _io, _connectors, name="litex")

    def create_programmer(self):
        return OpenFPGALoader(cable="usb-blaster")
    
    # def build(self, platform, fragment, **kwargs):
    def build(self, *args, **kwargs):
        self.build_without_toolchain_args(*args, **kwargs)

    def build_without_toolchain_args(self, fragment,
        build_dir      = "build",
        build_name     = "litex",
        synth_opts     = "",
        run            = True,
        build_backend  = "litex",
        **kwargs):
        os.makedirs(build_dir, exist_ok=True)
        cwd = os.getcwd()
        os.chdir(build_dir)

        self.toolchain._build_name = build_name
        self.toolchain._build_dir  = build_dir
        self.toolchain._synth_opts += synth_opts
        self.toolchain.platform    = self
        self.toolchain.fragment    = fragment

        # v_output = self.get_verilog(fragment, name="litex", **kwargs)

        self.finalize(fragment)

        # so.update(special_overrides)
        print("Building Verilog")
        v_output = self.get_verilog(fragment, name=build_name, **kwargs)
        v_file = build_name + ".v"
        v_output.write(v_file)
        
        # Finalize toolchain (after gateware is complete)
        self.toolchain.finalize()

        # Get signals and platform constraints
        self.named_sc, self.named_pc = self.resolve_signals(v_output.ns)
        self.add_source(v_file)
        
        print("Building timing constraints")
        self.toolchain.build_timing_constraints(v_output.ns)

    # Taken from altera/platform.py
    def add_reserved_jtag_decls(self):
        self.add_extension([*[(pad, 0, Pins(pad)) for pad in common.altera_reserved_jtag_pads]])

    def get_reserved_jtag_pads(self):
        r = {}
        for pad in common.altera_reserved_jtag_pads:
            r[pad] = self.request(pad)
        return r

    def do_finalize(self, fragment):
        # AlteraPlatform.do_finalize(self, fragment)
        # self.add_period_constraint(self.lookup_request("clk74a", loose=True), 1e9/74.25e6)
        # self.add_period_constraint(self.lookup_request("clk74b", loose=True), 1e9/74.25e6)
        print("Finalize")
