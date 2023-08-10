module risc_v #(
    parameter CLK_SPEED = 100_000_000,
    parameter BAUDRATE  = 115200
) (
    input wire clk,
    input wire reset,

    // Program upload
    input wire ioctl_download,
    input wire [16:0] ioctl_addr,
    input wire [31:0] ioctl_dout,
    input wire ioctl_wr,

    output wire reload_rom_active,

    input  wire uart_rx,
    output wire uart_tx
);

  wire [31:0] instr_addr;
  wire [31:0] instr_data;
  wire instr_req;
  wire instr_ack;

  wire [31:0] data_addr;
  wire [31:0] data_rd_data;
  wire [31:0] data_wr_data;
  wire data_wr_en;
  wire [3:0] data_mask;
  wire data_req;
  wire data_ack;

  kronos_core #(
      .BOOT_ADDR            (32'h0),
      .FAST_BRANCH          (1),
      .EN_COUNTERS          (1),
      .EN_COUNTERS64B       (0),
      .CATCH_ILLEGAL_INSTR  (1),
      .CATCH_MISALIGNED_JMP (1),
      .CATCH_MISALIGNED_LDST(1)
  ) riscv (
      .clk (clk),
      .rstz(~reset && ~reload_rom_active),

      .instr_addr(instr_addr),
      .instr_data(instr_data),
      .instr_req (instr_req),
      .instr_ack (instr_ack),

      .data_addr   (data_addr),
      .data_rd_data(data_rd_data),
      .data_wr_data(data_wr_data),
      .data_mask   (data_mask),
      .data_wr_en  (data_wr_en),
      .data_req    (data_req),
      .data_ack    (data_ack),

      // Interrupts must be disabled, as programs are forced to be single threaded
      .software_interrupt(1'b0),
      .timer_interrupt   (1'b0),
      .external_interrupt(1'b0)
  );

  wire [7:0] uart_tx_data;
  wire uart_tx_req;
  wire uart_tx_busy;

  wire reload_rom_wr;
  wire [31:0] reload_rom_addr;
  wire [31:0] reload_rom_data;

  memory_map memory_map (
      .clk(clk),

      .data_addr(data_addr),
      .data_data(data_wr_data),
      .data_q(data_rd_data),

      .data_req (data_req),
      .data_wren(data_wr_en),
      .data_mask(data_mask),
      .data_ack (data_ack),

      .inst_addr(instr_addr),
      .inst_req(instr_req),
      .inst_ack(instr_ack),
      .inst_q(instr_data),

      // Program upload
      .ioctl_download(ioctl_download || reload_rom_active),
      .ioctl_addr(reload_rom_active ? reload_rom_addr : ioctl_addr),
      .ioctl_dout(reload_rom_active ? reload_rom_data : ioctl_dout),
      .ioctl_wr(ioctl_wr || reload_rom_wr),

      // UART
      .uart_tx_data(uart_tx_data),
      .uart_tx_req (uart_tx_req)
  );

  uart #(
      .CLK_SPEED(CLK_SPEED),
      .BAUDRATE (BAUDRATE)
  ) uart (
      .clk  (clk),
      .reset(reset),

      .tx_data(uart_tx_data),
      .tx_req(uart_tx_req),
      .tx_busy(uart_tx_busy),
      .txd(uart_tx),

      .rxd(uart_rx),

      // Reloading ROM data
      .reload_rom_active(reload_rom_active),
      .reload_rom_wr(reload_rom_wr),
      .reload_rom_addr(reload_rom_addr),
      .reload_rom_data(reload_rom_data)
  );
endmodule
