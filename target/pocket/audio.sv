module audio (
    input wire clk_74b,
    input wire clk_sys,

    input wire reset,

    input wire [31:0] audio_bus_in,
    input wire audio_bus_wr,

    input wire audio_playback_en,
    input wire audio_flush,

    output wire [11:0] audio_buffer_fill,

    output wire audio_mclk,  //! Serial Master Clock
    output wire audio_lrck,  //! Left/Right clock
    output wire audio_dac    //! Serialized data
);

  ////////////////////////////////////////////////////////////////////////////////////////
  // Audio PLL

  wire audio_sclk;

  mf_audio_pll audio_pll (
      .refclk  (clk_74b),
      .rst     (0),
      .outclk_0(audio_mclk),
      .outclk_1(audio_sclk)
  );

  ////////////////////////////////////////////////////////////////////////////////////////
  // FIFO

  wire audio_playback_en_s;

  synch_3 settings_synch (
      audio_playback_en,
      audio_playback_en_s,
      audio_mclk
  );

  wire [15:0] audio_l;
  wire [15:0] audio_r;

  wire empty;

  dcfifo dcfifo_component (
      .wrclk(clk_sys),
      .rdclk(audio_mclk),

      .data (audio_bus_in),
      .wrreq(audio_bus_wr),

      .q({audio_l, audio_r}),
      .rdreq(audio_req && audio_playback_en_s),

      .rdempty(empty),
      .wrusedw(audio_buffer_fill),

      .aclr(reset || audio_flush)
      // .eccstatus(),
      // .rdfull(),
      // .rdusedw(),
      // .wrempty(),
      // .wrfull()
  );
  defparam dcfifo_component.intended_device_family = "Cyclone V",
      dcfifo_component.lpm_numwords = 4096, dcfifo_component.lpm_showahead = "OFF",
      dcfifo_component.lpm_type = "dcfifo", dcfifo_component.lpm_width = 32,
      dcfifo_component.lpm_widthu = 12, dcfifo_component.overflow_checking = "ON",
      dcfifo_component.rdsync_delaypipe = 5, dcfifo_component.underflow_checking = "ON",
      dcfifo_component.use_eab = "ON", dcfifo_component.wrsync_delaypipe = 5;

  reg audio_req = 0;

  reg [7:0] mclk_div = 8'hFF;

  always @(posedge audio_mclk) begin
    // MClk is 12.288 MHz, we want 48kHz
    audio_req <= 0;

    if (mclk_div > 0) begin
      mclk_div <= mclk_div - 8'h1;
    end else begin
      mclk_div  <= 8'hFF;

      audio_req <= 1;
    end
  end

  ////////////////////////////////////////////////////////////////////////////////////////
  // i2s Generation

  sound_i2s #(
      .SIGNED_INPUT(1)
  ) sound_i2s (
      .audio_sclk(audio_sclk),

      .audio_l(audio_playback_en_s && ~empty ? audio_l : 16'h0),
      .audio_r(audio_playback_en_s && ~empty ? audio_r : 16'h0),

      .audio_lrck(audio_lrck),
      .audio_dac (audio_dac)
  );


endmodule
