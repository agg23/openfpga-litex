module risc_v (
    input wire clk,
    input wire reset,

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
      .rstz(~reset),

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
  wire uart_tx_start;
  wire uart_tx_busy;

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

      // UART
      .uart_tx_data(uart_tx_data),
      .uart_tx_req (uart_tx_req)
  );

  uart #(
      .CLK_SPEED(100_000_000),
      .BAUDRATE (115200)
  ) uart (
      .clk  (clk),
      .reset(reset),

      .tx_data(uart_tx_data),
      .tx_req(uart_tx_req),
      .tx_busy(uart_tx_busy),
      .txd(uart_tx)
  );
endmodule
