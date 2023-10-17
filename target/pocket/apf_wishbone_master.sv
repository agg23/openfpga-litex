module apf_wishbone_master (
    input wire clk_74a,
    input wire clk_sys,

    input wire reset,

    input wire [31:0] bridge_addr,
    input wire [31:0] bridge_wr_data,
    input wire bridge_wr,
    input wire bridge_endian_little,

    input wire [31:0] ram_data_address,

    output wire [26:0] current_address,

    output wire [29:0] addr,
    // Wishbone registered feedback flags
    // Burst style extension. Specifies the type of burst
    output reg [1:0] bte = 0,
    // Cycle type identifier. Indicates what kind of burst cycle is being used
    output reg [2:0] cti = 0,
    // Cycle. High during the duration of the bus operations
    output reg cyc = 0,
    output wire [31:0] data_write,
    // Which bytes are active in read/write
    output reg [3:0] sel = 0,
    // Strobe. Need to receive ack or err after this
    output reg stb = 0,
    // Write enable
    output reg we = 0,

    input wire ack,
    input wire [31:0] data_read,
    input reg err
);

  wire [25:0] write_addr;

  assign current_address = write_addr;

  // TODO: Add read address
  // assign addr = we ? {6'h0, write_addr} + ram_data_address[31:2] : 30'h0;
  assign addr = {6'h0, write_addr} + ram_data_address[31:2];

  ////////////////////////////////////////////////////////////////////////////////////////
  // FIFO
  // Derived from data loader

  localparam ADDRESS_MASK_UPPER_4 = 4'h0;

  wire mem_empty;
  wire [57:0] fifo_out;

  assign {write_addr, data_write} = fifo_out;

  dcfifo dcfifo_component (
      .wrclk(clk_74a),
      .rdclk(clk_sys),

      .wrreq(wrreq),
      .data ({bridge_addr[27:2], bridge_wr_data}),

      // Immediately request if we're idle
      .rdreq(~mem_empty && state == NONE),
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
      dcfifo_component.lpm_width = 58, dcfifo_component.lpm_widthu = 2,
      dcfifo_component.overflow_checking = "OFF", dcfifo_component.rdsync_delaypipe = 5,
      dcfifo_component.underflow_checking = "OFF", dcfifo_component.use_eab = "OFF",
      dcfifo_component.wrsync_delaypipe = 5;

  reg prev_bridge_wr = 0;

  wire wrreq = ~prev_bridge_wr && bridge_wr && bridge_addr[31:28] == ADDRESS_MASK_UPPER_4;

  // Receive APF writes and write to FIFO
  always @(posedge clk_74a) begin
    prev_bridge_wr <= bridge_wr;
  end

  localparam NONE = 0;
  localparam WRITE = 1;
  // localparam WRITE_WAIT_ACK = 2;

  reg [ 2:0] state = 3'h0;

  reg [31:0] stored_data  /* synthesis noprune */;

  always @(posedge clk_sys) begin
    // Write all bytes
    sel <= 4'hF;

    cti <= 3'h0;

    case (state)
      NONE: begin
        cyc <= 0;
        stb <= 0;

        we  <= 0;

        if (~mem_empty) begin
          // Send request to SDRAM slave
          // FIFO read has already begun, will be done when we enter next state
          state <= WRITE;
        end
      end
      WRITE: begin
        cyc <= 1;
        stb <= 1;

        we  <= 1;

        if (ack) begin
          state <= NONE;

          stored_data <= data_read;

          cyc <= 0;
          stb <= 0;
        end
      end
    endcase
  end

endmodule
