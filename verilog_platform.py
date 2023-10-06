#
# Based on code in LiteX-Boards
#

from litex.build.altera import common, quartus
from litex.build.generic_platform import *
from litex.build.openfpgaloader import OpenFPGALoader

# Platform -----------------------------------------------------------------------------------------

class Platform(GenericPlatform):
    default_clk_name   = "clk74a"
    default_clk_period = 1e9/74.25e6

    _supported_toolchains = ["quartus"]

    def __init__(self):
        _io = [
            ("clk_sys", 0, Pins(1)),
            ("clk_sys_90deg", 0, Pins(1)),

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
            )
        ]
        _connectors = []

        GenericPlatform.__init__(self, "5CEBA4F23C8", _io, _connectors, name="litex")

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

        # v_output = self.get_verilog(fragment, name="litex", **kwargs)
        so = dict(common.altera_special_overrides)
        # so.update(special_overrides)
        print("Building Verilog")
        v_output = self.get_verilog(fragment,
            name=build_name,
            special_overrides = so,
            attr_translate    = quartus.AlteraQuartusToolchain().attr_translate,
            **kwargs)
                
        v_file = build_name + ".v"
        v_output.write(v_file)

    def do_finalize(self, fragment):
        # AlteraPlatform.do_finalize(self, fragment)
        # self.add_period_constraint(self.lookup_request("clk74a", loose=True), 1e9/74.25e6)
        # self.add_period_constraint(self.lookup_request("clk74b", loose=True), 1e9/74.25e6)
        print("Finalize")
