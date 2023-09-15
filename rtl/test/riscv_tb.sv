`timescale 1ns / 1ns

module riscv_tb;

  reg clk = 0;

  reg reset = 1;

  wire uart_tx;
  wire [7:0] uart_data;
  wire uart_done;

  risc_v risc_uut (
      .clk  (clk),
      .reset(reset),

      // Program upload
      .ioctl_download(1'b0),
      .ioctl_addr(17'h0),
      .ioctl_dout(32'h0),
      .ioctl_wr(1'b0),

      // Display IO
      // Automatically say write has completed to PSRAM
      .display_busy(1'b0),

      .uart_tx(uart_tx)
  );

  uart_output uart_output (
      .clk  (clk),
      .reset(reset),

      .uart_tx(uart_tx)
  );

  // Register file
  wire [31:0] ra = risc_uut.riscv.u_if.u_rf.REG[5'd1];
  wire [31:0] sp = risc_uut.riscv.u_if.u_rf.REG[5'd2];
  wire [31:0] gp = risc_uut.riscv.u_if.u_rf.REG[5'd3];
  wire [31:0] tp = risc_uut.riscv.u_if.u_rf.REG[5'd4];
  wire [31:0] t0 = risc_uut.riscv.u_if.u_rf.REG[5'd5];
  wire [31:0] t1 = risc_uut.riscv.u_if.u_rf.REG[5'd6];
  wire [31:0] t2 = risc_uut.riscv.u_if.u_rf.REG[5'd7];
  wire [31:0] s0 = risc_uut.riscv.u_if.u_rf.REG[5'd8];
  wire [31:0] s1 = risc_uut.riscv.u_if.u_rf.REG[5'd9];
  wire [31:0] a0 = risc_uut.riscv.u_if.u_rf.REG[5'd10];
  wire [31:0] a1 = risc_uut.riscv.u_if.u_rf.REG[5'd11];
  wire [31:0] a2 = risc_uut.riscv.u_if.u_rf.REG[5'd12];
  wire [31:0] a3 = risc_uut.riscv.u_if.u_rf.REG[5'd13];
  wire [31:0] a4 = risc_uut.riscv.u_if.u_rf.REG[5'd14];
  wire [31:0] a5 = risc_uut.riscv.u_if.u_rf.REG[5'd15];
  wire [31:0] a6 = risc_uut.riscv.u_if.u_rf.REG[5'd16];
  wire [31:0] a7 = risc_uut.riscv.u_if.u_rf.REG[5'd17];
  wire [31:0] s2 = risc_uut.riscv.u_if.u_rf.REG[5'd18];
  wire [31:0] s3 = risc_uut.riscv.u_if.u_rf.REG[5'd19];
  wire [31:0] s4 = risc_uut.riscv.u_if.u_rf.REG[5'd20];
  wire [31:0] s5 = risc_uut.riscv.u_if.u_rf.REG[5'd21];
  wire [31:0] s6 = risc_uut.riscv.u_if.u_rf.REG[5'd22];
  wire [31:0] s7 = risc_uut.riscv.u_if.u_rf.REG[5'd23];
  wire [31:0] s8 = risc_uut.riscv.u_if.u_rf.REG[5'd24];
  wire [31:0] s9 = risc_uut.riscv.u_if.u_rf.REG[5'd25];
  wire [31:0] s10 = risc_uut.riscv.u_if.u_rf.REG[5'd26];
  wire [31:0] s11 = risc_uut.riscv.u_if.u_rf.REG[5'd27];
  wire [31:0] t3 = risc_uut.riscv.u_if.u_rf.REG[5'd28];
  wire [31:0] t4 = risc_uut.riscv.u_if.u_rf.REG[5'd29];
  wire [31:0] t5 = risc_uut.riscv.u_if.u_rf.REG[5'd30];
  wire [31:0] t6 = risc_uut.riscv.u_if.u_rf.REG[5'd31];

  task cycle();
    #1 clk = ~clk;
    #1 clk = ~clk;

    if (risc_uut.riscv.u_ex.exception) begin
      $display("Exception (%t): Trap Cause %h, value %h", $time(), risc_uut.riscv.u_ex.trap_cause,
               risc_uut.riscv.u_ex.trap_value);
    end
  endtask

  initial begin
    int i;

    // for (i = 0; i < 1024; i += 1) begin
    //   risc_uut.memory_map.mem[i] = 0;
    // end

    $readmemh("../examples/rust/rust.hex",
              risc_uut.memory_map.ram.altsyncram_component.m_default.altsyncram_inst.mem_data);

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
