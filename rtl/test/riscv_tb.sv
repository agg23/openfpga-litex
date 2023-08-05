module riscv_tb;

  reg clk = 0;

  reg reset = 1;

  risc_v risc_uut (
      .clk  (clk),
      .reset(reset)
  );

  task cycle();
    #1 clk = ~clk;
    #1 clk = ~clk;
  endtask

  initial begin
    int i;

    for (i = 0; i < 1024; i += 1) begin
      risc_uut.memory_map.mem[i] = 0;
    end

    $readmemh("../examples/rust/rust.hex", risc_uut.memory_map.mem);

    cycle();
    cycle();
    cycle();
    cycle();
    cycle();
    cycle();
    cycle();
    cycle();

    reset = 0;

    forever begin
      cycle();
    end
  end

endmodule
