module memory_map (
    input wire clk,

    input  wire [31:0] data_addr,
    input  wire [31:0] data_data,
    output reg  [31:0] data_q,

    input wire data_req,
    input wire data_wren,
    input wire [3:0] data_mask,
    output reg data_ack,

    input wire [31:0] inst_addr,
    input wire inst_req,
    output reg inst_ack,
    output reg [31:0] inst_q,

    // Program upload
    input wire ioctl_download,
    input wire [16:0] ioctl_addr,
    input wire [31:0] ioctl_dout,
    input wire ioctl_wr,

    // UART
    output reg [7:0] uart_tx_data = 0,
    output reg uart_tx_req = 0
);

  // reg [31:0] mem[128*1024];

  // initial $readmemh("../examples/rust/rust.hex", ram.altsyncram_component);

  wire is_ram_data_addr = data_addr < 32'h1_0000;

  wire ram_data_req = data_req && is_ram_data_addr;

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
      .q_b(data_q),
      .rden_b(ram_data_req && ~data_wren)
  );

  always @(posedge clk) begin
    inst_ack <= 0;
    data_ack <= 0;

    if (inst_req) begin
      // Data is available in the next cycle
      inst_ack <= 1;
    end

    if (data_req) begin
      // Data is available in the next cycle
      data_ack <= 1;
    end
  end

  always @(posedge clk) begin
    uart_tx_req <= 0;

    if (data_req && ~is_ram_data_addr) begin
      // Memmapped addresses
      casex ({
        data_addr[31:2], 2'b00
      })
        32'h8000_0000: begin
          if (data_wren) begin
            // $display("Writing %c", data_data[7:0]);
            uart_tx_data <= data_data[7:0];
            uart_tx_req  <= 1;
          end
        end
        default: begin

        end
      endcase
    end
  end

  // Instruction
  // always @(posedge clk) begin
  //   inst_ack <= 0;

  //   if (inst_req) begin
  //     inst_ack <= 1;

  //     inst_q   <= mem[inst_addr[18:2]];
  //   end
  // end

  // Data
  // always @(posedge clk) begin
  //   if (ioctl_download) begin
  //     if (ioctl_wr) begin
  //       mem[ioctl_addr] <= ioctl_dout;
  //     end
  //   end else begin
  //     data_ack <= 0;

  //     uart_tx_req <= 0;

  //     if (data_req) begin
  //       data_ack <= 1;

  //       casex ({
  //         data_addr[31:2], 2'b00
  //       })
  //         32'h000X_XXXX, 32'h001X_XXXX: begin
  //           // Actual RAM
  //           if (data_wren) begin
  //             if (data_mask[0]) begin
  //               mem[data_addr[18:2]][7:0] <= data_data[7:0];
  //             end

  //             if (data_mask[1]) begin
  //               mem[data_addr[18:2]][15:8] <= data_data[15:8];
  //             end

  //             if (data_mask[2]) begin
  //               mem[data_addr[18:2]][23:16] <= data_data[23:16];
  //             end

  //             if (data_mask[3]) begin
  //               mem[data_addr[18:2]][31:24] <= data_data[31:24];
  //             end
  //           end else begin
  //             reg [31:0] read_data;
  //             read_data = mem[data_addr[18:2]];

  //             if (data_mask[0]) begin
  //               data_q[7:0] <= read_data[7:0];
  //             end

  //             if (data_mask[1]) begin
  //               data_q[15:8] <= read_data[15:8];
  //             end

  //             if (data_mask[2]) begin
  //               data_q[23:16] <= read_data[23:16];
  //             end

  //             if (data_mask[3]) begin
  //               data_q[31:24] <= read_data[31:24];
  //             end
  //           end
  //         end
  //         // 32'h8000_0000: begin
  //         //   if (data_wren) begin
  //         //     // UART write
  //         //     $display("%c %d", data_data[7:0], data_data[7:0]);
  //         //   end
  //         // end
  //         32'h8000_0000: begin
  //           if (data_wren) begin
  //             // $display("Writing %c", data_data[7:0]);
  //             uart_tx_data <= data_data[7:0];
  //             uart_tx_req  <= 1;
  //           end
  //         end
  //         default: begin

  //         end
  //       endcase
  //     end
  //   end
  // end

endmodule
