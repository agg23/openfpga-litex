module memory_map (
    input wire clk,

    input wire reset,

    // Main RAM
    input  wire [31:0] data_addr,
    input  wire [31:0] data_data,
    output reg  [31:0] data_q,

    input wire data_req,
    input wire data_wren,
    input wire [3:0] data_mask,
    output wire data_ack,

    // Program RAM
    input wire [31:0] inst_addr,
    input wire inst_req,
    output reg inst_ack,
    output reg [31:0] inst_q,

    // Program upload
    input wire ioctl_download,
    input wire [16:0] ioctl_addr,
    input wire [31:0] ioctl_dout,
    input wire ioctl_wr,

    // Display IO
    output reg [19:0] display_addr = 20'h0,
    output reg [15:0] display_data = 16'h0,
    output wire display_wr,
    output reg display_flip_framebuffer = 0,
    input wire display_busy,

    // Other IO
    input wire vblank,

    // UART
    output reg [7:0] uart_tx_data = 0,
    output reg uart_tx_req = 0
);

  wire is_ram_data_addr = data_addr < 32'h1_0000;

  wire ram_data_req = data_req && is_ram_data_addr;

  wire [31:0] ram_data_q;
  reg [31:0] bus_data_q;

  ram ram (
      .clock(clk),

      .address_a(inst_addr[17:2]),
      .address_b(ioctl_download ? ioctl_addr : data_addr[17:2]),

      .byteena_b(ioctl_download ? 4'hF : data_mask),

      .data_a(32'h0),
      .wren_a(1'b0),
      .data_b(ioctl_download ? ioctl_dout : data_data),
      .wren_b(ioctl_download ? ioctl_wr : ram_data_req && data_wren),

      .q_a(inst_q),
      .rden_a(inst_req),
      .q_b(ram_data_q),
      .rden_b(ram_data_req && ~data_wren)
  );

  assign data_q = is_ram_data_addr ? ram_data_q : bus_data_q;

  reg inst_delay = 0;
  reg inst_delay2 = 0;

  always @(posedge clk) begin
    inst_ack <= 0;
    inst_delay <= 0;
    inst_delay2 <= 0;

    // if (inst_delay2) begin
    //   inst_ack <= 1;
    // end else if (inst_delay) begin
    //   // Data is available in the next cycle
    //   // inst_ack <= 1;
    //   inst_delay2 <= 1;
    // end else if (~reset && inst_req) begin
    //   inst_delay <= 1;
    // end
    if (inst_req) begin
      // Data is available in the next cycle
      inst_ack <= 1;
    end

  end

  reg [5:0] vblank_count = 0;

  reg prev_vblank = 0;

  always @(posedge clk) begin
    prev_vblank <= vblank;

    if (vblank && ~prev_vblank) begin
      if (vblank_count == 6'd59) begin
        vblank_count <= 0;
      end else begin
        vblank_count <= vblank_count + 6'h1;
      end
    end
  end

  reg prev_data_req = 0;

  reg [1:0] display_wr_tick_count = 0;

  reg primary_data_ack = 0;
  reg primary_delay_ack = 0;

  // When set, data_ack uses display_busy signal
  reg use_vid_ack = 0;

  assign data_ack   = use_vid_ack ? ~display_wr && ~display_busy : primary_data_ack;
  assign display_wr = display_wr_tick_count != 0;

  always @(posedge clk) begin
    prev_data_req <= data_req;

    primary_data_ack <= 0;
    primary_delay_ack <= 0;

    uart_tx_req <= 0;
    display_flip_framebuffer <= 0;

    if (data_ack) begin
      use_vid_ack <= 0;
    end

    if (display_wr_tick_count != 0) begin
      display_wr_tick_count <= display_wr_tick_count - 2'h1;
    end

    if (primary_delay_ack) begin
      primary_data_ack <= 1;
    end

    if (data_req && ~prev_data_req) begin
      // By default, data is available in the next cycle
      // primary_delay_ack <= 1;
      primary_data_ack <= 1;

      // Memmapped addresses
      casex ({
        data_addr[31:2], 2'b00
      })
        // 32'h0000_XXXX: begin
        //   // Main RAM
        // end
        32'h001X_XXXX: begin
          // Display RAM
          use_vid_ack <= 1;

          // TODO: Remove expanded address
          display_addr <= {2'b0, data_addr[19:2]};
          display_data <= data_data[15:0];

          // Three cycles for a vid RAM pulse to stay high
          display_wr_tick_count <= 2'h3;
        end
        32'h8000_0000: begin
          // UART data
          if (data_wren) begin
            // $display("Writing %c", data_data[7:0]);
            uart_tx_data <= data_data[7:0];
            uart_tx_req  <= 1;
          end
        end
        32'h8000_1000: begin
          // Swap framebuffer
          if (data_wren) begin
            display_flip_framebuffer <= 1;
          end
        end
        32'h8000_1004: begin
          // vblank status
          if (~data_wren) begin
            bus_data_q <= {18'h0, vblank_count, 7'b0, vblank};
          end
        end
        default: begin

        end
      endcase
    end
  end

endmodule
