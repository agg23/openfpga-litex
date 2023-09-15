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
    output wire txd,

    input wire rxd,

    // Reloading ROM data
    output wire reload_rom_active,
    output reg reload_rom_wr = 0,
    output reg [31:0] reload_rom_addr = 0,
    output reg [31:0] reload_rom_data = 0
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

  reg start_upload_uart_wr = 0;
  reg reset_uart_wr = 0;

  wire [7:0] rx_data;
  wire rx_finished_byte;

  uart_tx #(
      .CLK_HZ(CLK_SPEED),
      .BAUD  (BAUDRATE)
  ) uart_tx (
      .clk (clk),
      .nrst(~reset),

      .tx_data(start_upload_uart_wr ? "s" : reset_uart_wr ? "r" : reload_rom_active ? rx_data : fifo_data),
      .tx_start(tx_start || start_upload_uart_wr || reset_uart_wr || reload_rom_active && rx_finished_byte),
      .tx_busy(tx_busy),
      .txd(txd)
  );

  uart_rx #(
      .CLK_HZ(CLK_SPEED),
      .BAUD  (BAUDRATE)
  ) uart_rx (
      .clk (clk),
      .nrst(~reset),

      .rx_data(rx_data),
      .rx_done(rx_finished_byte),
      .rxd(rxd)
  );

  localparam RESET_TIMEOUT = CLK_SPEED * 5;

  localparam STATE_NONE = 0;
  localparam STATE_ROM_DOWNLOAD = 1;
  // localparam STATE_RESETTING = 2;
  localparam STATE_CLEARING = 3;

  reg [1:0] state = STATE_NONE;

  assign reload_rom_active = state == STATE_ROM_DOWNLOAD || state == STATE_CLEARING;

  reg [31:0] shift_data = 0;
  reg [ 1:0] shift_count = 0;
  reg [31:0] rom_addr = 0;

  reg [31:0] rom_download_timeout = 0;

  always @(posedge clk) begin
    if (reset) begin
      state <= STATE_NONE;

      shift_count <= 0;
      rom_addr <= 0;
    end else begin
      reload_rom_wr <= 0;
      start_upload_uart_wr <= 0;
      reset_uart_wr <= 0;

      if (rom_download_timeout == 1) begin
        state <= STATE_CLEARING;

        // if (state == STATE_RESETTING) begin
        //   reset_uart_wr <= 1;
        // end
      end

      if (rom_download_timeout != 0) begin
        rom_download_timeout <= rom_download_timeout - 32'h1;
      end

      case (state)
        STATE_NONE: begin
          shift_count <= 0;
          rom_addr <= 0;
          reload_rom_addr <= 0;
        end
        STATE_CLEARING: begin
          if (rom_addr == 32'h1_0000) begin
            // Finished
            state <= STATE_NONE;

            reset_uart_wr <= 1;
          end else begin
            reload_rom_wr <= 1;
            reload_rom_addr <= rom_addr;
            reload_rom_data <= 32'h0;

            rom_addr <= rom_addr + 32'h1;
          end
        end
      endcase

      if (rx_finished_byte) begin
        // Received char
        case (state)
          STATE_NONE: begin
            case (rx_data)
              "d": begin
                rom_download_timeout <= RESET_TIMEOUT;

                state <= STATE_ROM_DOWNLOAD;

                start_upload_uart_wr <= 1;
              end
              // "r": begin
              //   // Reset
              //   rom_download_timeout <= 20;

              //   state <= STATE_RESETTING;
              // end
              default: begin
                // Do nothing
              end
            endcase
          end
          STATE_ROM_DOWNLOAD: begin
            rom_download_timeout <= RESET_TIMEOUT;

            shift_data <= {rx_data, shift_data[31:8]};

            shift_count <= shift_count + 1;

            if (shift_count == 3) begin
              // Received word
              reload_rom_wr <= 1;

              reload_rom_addr <= rom_addr;
              reload_rom_data <= {rx_data, shift_data[31:8]};

              rom_addr <= rom_addr + 32'h1;
            end
          end
        endcase
      end
    end
  end

endmodule
