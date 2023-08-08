//------------------------------------------------------------------------------
// uart_rx.sv
// published as part of https://github.com/pConst/basic_verilog
// Konstantin Pavlov, pavlovconst@gmail.com
//------------------------------------------------------------------------------

// INFO ------------------------------------------------------------------------
//  Straightforward yet simple UART receiver implementation
//    for FPGA written in SystemVerilog
//
//  Expects at least one stop bit
//  Features continuous data aquisition at BAUD levels up to CLK_HZ / 2
//  Features early asynchronous 'busy' reset
//
//  see also "uart_rx.v" for equivalent Verilog version
//

/* --- INSTANTIATION TEMPLATE BEGIN ---

uart_rx #(
  .CLK_HZ( 200_000_000 ),  // in Hertz
  .BAUD( 9600 )            // max. BAUD is CLK_HZ / 2
)(
  .clk(  ),
  .nrst(  ),

  .rx_data(  ),
  .rx_done(  ),
  .rxd(  )
);

--- INSTANTIATION TEMPLATE END ---*/


module uart_rx #( parameter
  CLK_HZ = 200_000_000,
  BAUD = 9600,
  bit [15:0] BAUD_DIVISOR_2 = CLK_HZ / BAUD / 2
)(
  input clk,
  input nrst,

  output logic [7:0] rx_data = '0,
  output logic rx_busy = 1'b0,
  output logic rx_done,         // read strobe
  output logic rx_err,
  input rxd
);

logic [15:0] rx_sample_cntr = (BAUD_DIVISOR_2 - 1'b1);

logic rx_do_sample;
assign rx_do_sample = (rx_sample_cntr[15:0] == '0);

logic prev_rxd = 0;

// {rx_data[7:0],rx_data_9th_bit} is actually a shift register
logic rx_data_9th_bit = 1'b0;
always_ff @ (posedge clk) begin
  if( ~nrst ) begin
    rx_busy <= 1'b0;
    rx_sample_cntr <= (BAUD_DIVISOR_2 - 1'b1);
    {rx_data[7:0],rx_data_9th_bit} <= '0;
  end else begin
    prev_rxd <= rxd;

    if( ~rx_busy ) begin
      // On falling edge
      if( prev_rxd && ~rxd ) begin
        // wait for 1,5-bit period till next sample
        rx_sample_cntr[15:0] <= (BAUD_DIVISOR_2 * 3 - 1'b1);
        rx_busy <= 1'b1;
        {rx_data[7:0],rx_data_9th_bit} <= 9'b10000000_0;

      end
    end else begin

      if( rx_sample_cntr[15:0] == '0 ) begin
        // wait for 1-bit-period till next sample
        rx_sample_cntr[15:0] <= (BAUD_DIVISOR_2 * 2 - 1'b1);
      end else begin
        // counting and sampling only when 'busy'
        rx_sample_cntr[15:0] <= rx_sample_cntr[15:0] - 1'b1;
      end

      if( rx_do_sample ) begin
        if( rx_data_9th_bit == 1'b1 ) begin
          // early asynchronous 'busy' reset
          rx_busy <= 1'b0;
        end else begin
          {rx_data[7:0],rx_data_9th_bit} <= {rxd, rx_data[7:0]};
        end
      end

    end // ~rx_busy
  end // ~nrst
end

always_comb begin
  // rx_done and rx_busy fall simultaneously
  rx_done <= rx_data_9th_bit && rx_do_sample && rxd;
  rx_err <= rx_data_9th_bit && rx_do_sample && ~rxd;
end

endmodule

