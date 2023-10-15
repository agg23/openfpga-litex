module wishbone_master (
    input wire clk_74a,
    input wire clk,

    input wire reset,

    input wire trigger_button,

    output reg [29:0] addr = 0,
    // Wishbone registered feedback flags
    // Burst style extension. Specifies the type of burst
    output reg [1:0] bte = 0,
    // Cycle type identifier. Indicates what kind of burst cycle is being used
    output reg [2:0] cti = 0,
    // Cycle. High during the duration of the bus operations
    output reg cyc = 0,
    output reg [31:0] data_write = 0,
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

  reg [31:0] stored_data  /* synthesis noprune */;

  // reg requesting = 0;
  reg [3:0] state = 0;
  reg prev_trigger_button = 0;

  always @(posedge clk) begin
    if (reset) begin
      addr <= 30'h1000_0000;


      cti  <= 3'h0;
      sel  <= 4'hF;
    end else begin
      prev_trigger_button <= trigger_button;

      if (state > 4'h0) begin
        state <= state + 4'h1;
      end

      case (state)
        0: begin
          cyc <= 0;
          stb <= 0;

          if (trigger_button && ~prev_trigger_button) begin
            state <= 4'h1;

            // cti   <= 3'h2;
            cti <= 0;
            // cti   <= 3'h7;
            we <= 1;

            stb <= 1;
            cyc <= 1;
            // cti   <= 3'h1;
          end
        end
        1: begin
          state <= 4'h1;

          // stb   <= 1;

          if (ack) begin
            state <= 4'h0;

            // cti <= 3'h7;

            stored_data <= data_read;

            data_write <= data_write + 32'h1;
            addr <= addr + 30'h1;

            stb <= 0;
            cyc <= 0;
          end
        end
        // 3: begin
        //   state <= 4'h3;

        //   stb   <= 1;

        //   if (ack) begin
        //     state <= 4'h4;

        //     stored_data <= data_read;

        //     stb <= 0;
        //   end
        // end
        // 4: begin
        //   state <= 4'h0;

        //   cyc   <= 0;

        //   addr  <= addr + 30'h1;
        // end
      endcase

      // if (requesting || (trigger_button && ~prev_trigger_button)) begin
      //   requesting <= 1;

      //   cyc <= 1;
      //   stb <= 1;

      //   if (ack) begin
      //     requesting <= 0;

      //     cyc <= 0;
      //     stb <= 0;
      //     addr <= addr + 32'h4;

      //     stored_data <= data_read;
      //   end
      // end
    end
  end

endmodule
