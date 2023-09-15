module video #(
    parameter MEM_CLK_SPEED = 40_000_000
) (
    input wire clk_sys,
    input wire clk_mem,
    input wire clk_vid,

    // Display IO
    input  wire [19:0] display_addr,
    input  wire [15:0] display_data,
    input  wire        display_wr,
    input  wire        display_flip_framebuffer,
    output wire        display_busy,

    // Video
    output wire        hsync,
    output wire        vsync,
    output wire        hblank,
    output wire        vblank,
    output wire [23:0] rgb,

    output wire de,

    // PSRAM signals
    output wire [21:16] cram0_a,
    inout  wire [ 15:0] cram0_dq,
    input  wire         cram0_wait,
    output wire         cram0_clk,
    output wire         cram0_adv_n,
    output wire         cram0_cre,
    output wire         cram0_ce0_n,
    output wire         cram0_ce1_n,
    output wire         cram0_oe_n,
    output wire         cram0_we_n,
    output wire         cram0_ub_n,
    output wire         cram0_lb_n,

    output wire [21:16] cram1_a,
    inout  wire [ 15:0] cram1_dq,
    input  wire         cram1_wait,
    output wire         cram1_clk,
    output wire         cram1_adv_n,
    output wire         cram1_cre,
    output wire         cram1_ce0_n,
    output wire         cram1_ce1_n,
    output wire         cram1_oe_n,
    output wire         cram1_we_n,
    output wire         cram1_ub_n,
    output wire         cram1_lb_n
);
  reg render_bank = 0;

  wire [21:0] video_addr;
  wire video_rd;

  // If rendering bank 1, we're writing to bank 0
  wire [21:0] bank0_addr = render_bank && display_wr ? {2'b0, display_addr} : video_addr;
  wire [21:0] bank1_addr = ~render_bank && display_wr ? {2'b0, display_addr} : video_addr;

  wire [15:0] bank0_pixel;
  wire [15:0] bank1_pixel;

  wire bank0_busy;
  wire bank1_busy;

  assign display_busy = render_bank ? bank0_busy : bank1_busy;

  // By default PSRAM is configured for a 4 cycle at 50MHz latency. This is perfect for our use
  psram #(
      .CLOCK_SPEED(MEM_CLK_SPEED / 1_000_000)
  ) psram_bank0 (
      .clk(clk_mem),

      .bank_sel(0),

      .addr(bank0_addr),

      .write_en(render_bank && display_wr),
      .data_in(display_data),
      .write_high_byte(1),
      .write_low_byte(1),

      .read_en (~render_bank && video_rd),
      .data_out(bank0_pixel),

      .busy(bank0_busy),

      // PSRAM signals
      .cram_a(cram0_a),
      .cram_dq(cram0_dq),
      .cram_wait(cram0_wait),
      .cram_clk(cram0_clk),
      .cram_adv_n(cram0_adv_n),
      .cram_cre(cram0_cre),
      .cram_ce0_n(cram0_ce0_n),
      .cram_ce1_n(cram0_ce1_n),
      .cram_oe_n(cram0_oe_n),
      .cram_we_n(cram0_we_n),
      .cram_ub_n(cram0_ub_n),
      .cram_lb_n(cram0_lb_n)
  );

  psram #(
      .CLOCK_SPEED(MEM_CLK_SPEED / 1_000_000)
  ) psram_bank1 (
      .clk(clk_mem),

      .bank_sel(0),

      .addr(bank1_addr),

      .write_en(~render_bank && display_wr),
      .data_in(display_data),
      .write_high_byte(1),
      .write_low_byte(1),

      .read_en (render_bank && video_rd),
      .data_out(bank1_pixel),

      .busy(bank1_busy),

      // PSRAM signals
      .cram_a(cram1_a),
      .cram_dq(cram1_dq),
      .cram_wait(cram1_wait),
      .cram_clk(cram1_clk),
      .cram_adv_n(cram1_adv_n),
      .cram_cre(cram1_cre),
      .cram_ce0_n(cram1_ce0_n),
      .cram_ce1_n(cram1_ce1_n),
      .cram_oe_n(cram1_oe_n),
      .cram_we_n(cram1_we_n),
      .cram_ub_n(cram1_ub_n),
      .cram_lb_n(cram1_lb_n)
  );

  rgb565_to_rgb888 rgb565_to_rgb888 (
      .rgb565(render_bank ? bank1_pixel : bank0_pixel),
      .rgb888(rgb)
  );

  wire [9:0] video_x;
  wire [9:0] video_y;

  counts counts (
      .clk(clk_vid),

      .x(video_x),
      .y(video_y),

      .hsync (hsync),
      .vsync (vsync),
      .hblank(hblank),
      .vblank(vblank),

      .de(de)
  );

  assign video_addr = video_y * 10'd360 + {11'b0, video_x};

  reg prev_video_tick = 0;
  reg prev_prev_video_tick = 0;

  assign video_rd = prev_video_tick != prev_prev_video_tick;

  always @(posedge clk_vid) begin
    prev_video_tick <= ~prev_video_tick;
  end

  always @(posedge clk_mem) begin
    prev_prev_video_tick <= prev_video_tick;
  end

  reg prev_display_flip_framebuffer = 0;

  always @(posedge clk_sys) begin
    prev_display_flip_framebuffer <= display_flip_framebuffer;

    if (display_flip_framebuffer && ~prev_display_flip_framebuffer) begin
      render_bank <= ~render_bank;
    end
  end

endmodule
