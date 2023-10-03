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
            ("clk74a", 0, Pins("clk74a")),

            # UART Pins
            ("serial", 0,
                Subsignal("tx", Pins("tx")),
                Subsignal("rx", Pins("rx"))
            ),

            ("test", 0, Pins("Foo")),
            ("sdram", 0, Pins("Bla foobar"))
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
