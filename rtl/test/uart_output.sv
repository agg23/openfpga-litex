module uart_output (
    input wire clk,
    input wire reset,

    input wire uart_tx
);

  wire [7:0] uart_data;
  wire uart_done;

  uart_rx #(
      .CLK_HZ(100_000_000),
      .BAUD  (115200)
  ) uart_rx (
      .clk (clk),
      .nrst(~reset),

      .rx_data(uart_data),
      // .rx_busy(),
      .rx_done(uart_done),
      // .rx_err(),
      .rxd(uart_tx)
  );

  always @(posedge clk) begin
    if (uart_done) begin
      case (uart_data)
        0: begin
          // Do nothing
        end
        default: $write("%c", uart_data);
      endcase
    end
  end

endmodule
