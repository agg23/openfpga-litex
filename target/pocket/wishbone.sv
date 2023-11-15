// TODO: Unused example of Wishbone slave
module wishbone (
    input wire clk_74a,
    input wire clk,

    input wire reset,

    input wire [29:0] addr,
    // Wishbone registered feedback flags
    // Burst style extension. Specifies the type of burst
    input wire [1:0] bte,
    // Cycle type identifier. Indicates what kind of burst cycle is being used
    input wire [2:0] cti,
    // Cycle. High during the duration of the bus operations
    input wire cyc,
    input wire [31:0] data_write,
    // Which bytes are active in read/write
    input wire [3:0] sel,
    // Strobe. Need to assert ack or err after this
    input wire stb,
    // Write enable
    input wire we,

    output reg ack = 0,
    output reg [31:0] data_read = 0,
    // output wire [31:0] data_read,
    output reg err = 0
);

  reg [31:0] ram[16];
  //  = '{
  //     32'h102938,
  //     32'h02832,
  //     32'h89493,
  //     32'h09472,
  //     32'h29083,
  //     32'h09848,
  //     32'h8282,
  //     32'h9480,
  //     32'h1234,
  //     32'h9876,
  //     32'h9628,
  //     32'h1,
  //     32'h2,
  //     32'h3,
  //     32'h4,
  //     32'h5
  // };

  reg initialized = 0;
  reg [3:0] init_addr = 0;

  always @(posedge clk) begin
    if (reset) begin
      initialized <= 0;
      init_addr   <= 0;
    end else begin
      if (~initialized) begin
        ram[init_addr] <= {28'h0, init_addr};

        init_addr <= init_addr + 4'h1;

        if (init_addr == 4'hF) begin
          initialized <= 1;
        end
      end
    end
  end

  // assign data_read = 32'hDEADBEEF;

  always @(posedge clk) begin
    ack <= 0;

    if (stb && cyc) begin
      // Strobe
      if (we) begin
        // Write
        ram[addr[3:0]] <= data_write;
      end else begin
        data_read <= ram[addr[3:0]];
      end

      ack <= 1;
    end
  end

endmodule
