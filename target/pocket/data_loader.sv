module data_loader #(
    // Upper 4 bits of address
    parameter ADDRESS_MASK_UPPER_4 = 0,
    parameter ADDRESS_SIZE = 28
) (
    input wire clk_74a,
    input wire clk_memory,

    input wire bridge_wr,
    input wire bridge_endian_little,
    input wire [31:0] bridge_addr,
    input wire [31:0] bridge_wr_data,

    // These outputs are synced to the memory clock
    output reg write_en = 0,
    output wire [ADDRESS_SIZE-1:0] write_addr,
    output wire [31:0] write_data
);

  wire mem_empty;
  wire [59:0] fifo_out;

  assign {write_addr, write_data} = fifo_out;

  dcfifo dcfifo_component (
      .wrclk(clk_74a),
      .rdclk(clk_memory),

      .wrreq(wrreq),
      .data ({bridge_addr[27:0], bridge_wr_data}),

      .rdreq(~mem_empty),
      .q(fifo_out),

      .rdempty(mem_empty)
      // .wrempty(),
      // .aclr(),
      // .eccstatus(),
      // .rdfull(),
      // .rdusedw(),
      // .wrfull(),
      // .wrusedw()
  );
  defparam dcfifo_component.clocks_are_synchronized = "FALSE",
      dcfifo_component.intended_device_family = "Cyclone V", dcfifo_component.lpm_numwords = 4,
      dcfifo_component.lpm_showahead = "OFF", dcfifo_component.lpm_type = "dcfifo",
      dcfifo_component.lpm_width = 60, dcfifo_component.lpm_widthu = 2,
      dcfifo_component.overflow_checking = "OFF", dcfifo_component.rdsync_delaypipe = 5,
      dcfifo_component.underflow_checking = "OFF", dcfifo_component.use_eab = "OFF",
      dcfifo_component.wrsync_delaypipe = 5;

  reg prev_bridge_wr = 0;

  wire wrreq = ~prev_bridge_wr && bridge_wr && bridge_addr[31:28] == ADDRESS_MASK_UPPER_4;

  // Receive APF writes and write to FIFO
  always @(posedge clk_74a) begin
    prev_bridge_wr <= bridge_wr;
  end

  always @(posedge clk_memory) begin
    // Delay write_en by one cycle so FIFO read finishes
    write_en <= ~mem_empty;
  end

endmodule
