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

    // UART
    output reg [7:0] uart_tx_data = 0,
    output reg uart_tx_req = 0
);

  reg [31:0] mem[128*1024];

  initial $readmemh("../examples/rust/rust.hex", mem);

  // Instruction
  always @(posedge clk) begin
    inst_ack <= 0;

    if (inst_req) begin
      inst_ack <= 1;

      inst_q   <= mem[inst_addr[18:2]];
    end
  end

  // Data
  always @(posedge clk) begin
    data_ack <= 0;

    uart_tx_req <= 0;

    if (data_req) begin
      data_ack <= 1;

      casex ({
        data_addr[31:2], 2'b00
      })
        32'h000X_XXXX, 32'h001X_XXXX: begin
          // Actual RAM
          if (data_wren) begin
            if (data_mask[0]) begin
              mem[data_addr[18:2]][7:0] <= data_data[7:0];
            end

            if (data_mask[1]) begin
              mem[data_addr[18:2]][15:8] <= data_data[15:8];
            end

            if (data_mask[2]) begin
              mem[data_addr[18:2]][23:16] <= data_data[23:16];
            end

            if (data_mask[3]) begin
              mem[data_addr[18:2]][31:24] <= data_data[31:24];
            end
          end else begin
            reg [31:0] read_data;
            read_data = mem[data_addr[18:2]];

            if (data_mask[0]) begin
              data_q[7:0] <= read_data[7:0];
            end

            if (data_mask[1]) begin
              data_q[15:8] <= read_data[15:8];
            end

            if (data_mask[2]) begin
              data_q[23:16] <= read_data[23:16];
            end

            if (data_mask[3]) begin
              data_q[31:24] <= read_data[31:24];
            end
          end
        end
        // 32'h8000_0000: begin
        //   if (data_wren) begin
        //     // UART write
        //     $display("%c %d", data_data[7:0], data_data[7:0]);
        //   end
        // end
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

endmodule
