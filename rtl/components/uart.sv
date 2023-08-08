module uart #(
    parameter CLK_SPEED = 100_000_000,
    parameter BAUDRATE  = 115200
) (
    input wire clk,
    input wire reset,

    input wire [7:0] tx_data,
    input wire tx_req,
    output wire tx_busy,
    // TODO: Rename
    output wire txd
);
  wire fifo_empty;
  wire [7:0] fifo_data;
  wire tx_start = ~fifo_empty && ~tx_busy;

  // NOTE: FIFO is in show-ahead mode. Data is available on `.data` before `.rdreq` is asserted
  uart_fifo uart_fifo (
      .clock(clk),

      .data(tx_data),
      .q(fifo_data),

      .rdreq(tx_start),
      .wrreq(tx_req),

      .empty(fifo_empty),
      // .full()

      .aclr(reset)
  );

  uart_tx #(
      .CLK_HZ(CLK_SPEED),
      .BAUD  (BAUDRATE)
  ) uart_tx (
      .clk (clk),
      .nrst(~reset),

      .tx_data(fifo_data),
      .tx_start(tx_start),
      .tx_busy(tx_busy),
      .txd(txd)
  );

endmodule
