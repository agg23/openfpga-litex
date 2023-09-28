//
// User core top-level
//
// Instantiated by the real top-level: apf_top
//

`default_nettype none

module core_top (

    //
    // physical connections
    //

    ///////////////////////////////////////////////////
    // clock inputs 74.25mhz. not phase aligned, so treat these domains as asynchronous

    input wire clk_74a,  // mainclk1
    input wire clk_74b,  // mainclk1

    ///////////////////////////////////////////////////
    // cartridge interface
    // switches between 3.3v and 5v mechanically
    // output enable for multibit translators controlled by pic32

    // GBA AD[15:8]
    inout  wire [7:0] cart_tran_bank2,
    output wire       cart_tran_bank2_dir,

    // GBA AD[7:0]
    inout  wire [7:0] cart_tran_bank3,
    output wire       cart_tran_bank3_dir,

    // GBA A[23:16]
    inout  wire [7:0] cart_tran_bank1,
    output wire       cart_tran_bank1_dir,

    // GBA [7] PHI#
    // GBA [6] WR#
    // GBA [5] RD#
    // GBA [4] CS1#/CS#
    //     [3:0] unwired
    inout  wire [7:4] cart_tran_bank0,
    output wire       cart_tran_bank0_dir,

    // GBA CS2#/RES#
    inout  wire cart_tran_pin30,
    output wire cart_tran_pin30_dir,
    // when GBC cart is inserted, this signal when low or weak will pull GBC /RES low with a special circuit
    // the goal is that when unconfigured, the FPGA weak pullups won't interfere.
    // thus, if GBC cart is inserted, FPGA must drive this high in order to let the level translators
    // and general IO drive this pin.
    output wire cart_pin30_pwroff_reset,

    // GBA IRQ/DRQ
    inout  wire cart_tran_pin31,
    output wire cart_tran_pin31_dir,

    // infrared
    input  wire port_ir_rx,
    output wire port_ir_tx,
    output wire port_ir_rx_disable,

    // GBA link port
    inout  wire port_tran_si,
    output wire port_tran_si_dir,
    inout  wire port_tran_so,
    output wire port_tran_so_dir,
    inout  wire port_tran_sck,
    output wire port_tran_sck_dir,
    inout  wire port_tran_sd,
    output wire port_tran_sd_dir,

    ///////////////////////////////////////////////////
    // cellular psram 0 and 1, two chips (64mbit x2 dual die per chip)

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
    output wire         cram1_lb_n,

    ///////////////////////////////////////////////////
    // sdram, 512mbit 16bit

    output wire [12:0] dram_a,
    output wire [ 1:0] dram_ba,
    inout  wire [15:0] dram_dq,
    output wire [ 1:0] dram_dqm,
    output wire        dram_clk,
    output wire        dram_cke,
    output wire        dram_ras_n,
    output wire        dram_cas_n,
    output wire        dram_we_n,

    ///////////////////////////////////////////////////
    // sram, 1mbit 16bit

    output wire [16:0] sram_a,
    inout  wire [15:0] sram_dq,
    output wire        sram_oe_n,
    output wire        sram_we_n,
    output wire        sram_ub_n,
    output wire        sram_lb_n,

    ///////////////////////////////////////////////////
    // vblank driven by dock for sync in a certain mode

    input wire vblank,

    ///////////////////////////////////////////////////
    // i/o to 6515D breakout usb uart

    output wire dbg_tx,
    input  wire dbg_rx,

    ///////////////////////////////////////////////////
    // i/o pads near jtag connector user can solder to

    output wire user1,
    input  wire user2,

    ///////////////////////////////////////////////////
    // RFU internal i2c bus

    inout  wire aux_sda,
    output wire aux_scl,

    ///////////////////////////////////////////////////
    // RFU, do not use
    output wire vpll_feed,


    //
    // logical connections
    //

    ///////////////////////////////////////////////////
    // video, audio output to scaler
    output wire [23:0] video_rgb,
    output wire        video_rgb_clock,
    output wire        video_rgb_clock_90,
    output wire        video_de,
    output wire        video_skip,
    output wire        video_vs,
    output wire        video_hs,

    output wire audio_mclk,
    input  wire audio_adc,
    output wire audio_dac,
    output wire audio_lrck,

    ///////////////////////////////////////////////////
    // bridge bus connection
    // synchronous to clk_74a
    output wire        bridge_endian_little,
    input  wire [31:0] bridge_addr,
    input  wire        bridge_rd,
    output reg  [31:0] bridge_rd_data,
    input  wire        bridge_wr,
    input  wire [31:0] bridge_wr_data,

    ///////////////////////////////////////////////////
    // controller data
    //
    // key bitmap:
    //   [0]    dpad_up
    //   [1]    dpad_down
    //   [2]    dpad_left
    //   [3]    dpad_right
    //   [4]    face_a
    //   [5]    face_b
    //   [6]    face_x
    //   [7]    face_y
    //   [8]    trig_l1
    //   [9]    trig_r1
    //   [10]   trig_l2
    //   [11]   trig_r2
    //   [12]   trig_l3
    //   [13]   trig_r3
    //   [14]   face_select
    //   [15]   face_start
    //   [28:16] <unused>
    //   [31:29] type
    // joy values - unsigned
    //   [ 7: 0] lstick_x
    //   [15: 8] lstick_y
    //   [23:16] rstick_x
    //   [31:24] rstick_y
    // trigger values - unsigned
    //   [ 7: 0] ltrig
    //   [15: 8] rtrig
    //
    input wire [31:0] cont1_key,
    input wire [31:0] cont2_key,
    input wire [31:0] cont3_key,
    input wire [31:0] cont4_key,
    input wire [31:0] cont1_joy,
    input wire [31:0] cont2_joy,
    input wire [31:0] cont3_joy,
    input wire [31:0] cont4_joy,
    input wire [15:0] cont1_trig,
    input wire [15:0] cont2_trig,
    input wire [15:0] cont3_trig,
    input wire [15:0] cont4_trig

);

  // not using the IR port, so turn off both the LED, and
  // disable the receive circuit to save power
  assign port_ir_tx              = 0;
  assign port_ir_rx_disable      = 1;

  // bridge endianness
  assign bridge_endian_little    = 1;

  // cart is unused, so set all level translators accordingly
  // directions are 0:IN, 1:OUT
  // assign cart_tran_bank3         = 8'hzz;
  // assign cart_tran_bank3_dir     = 1'b0;
  assign cart_tran_bank2         = 8'hzz;
  assign cart_tran_bank2_dir     = 1'b0;
  assign cart_tran_bank1         = 8'hzz;
  assign cart_tran_bank1_dir     = 1'b0;
  // assign cart_tran_bank0         = 4'hf;
  // assign cart_tran_bank0_dir     = 1'b1;
  assign cart_tran_pin30         = 1'b0;  // reset or cs2, we let the hw control it by itself
  assign cart_tran_pin30_dir     = 1'bz;
  assign cart_pin30_pwroff_reset = 1'b0;  // hardware can control this
  // assign cart_tran_pin31         = 1'bz;  // input
  // assign cart_tran_pin31_dir     = 1'b0;  // input

  // link port is input only
  assign port_tran_so            = 1'bz;
  assign port_tran_so_dir        = 1'b0;  // SO is output only
  assign port_tran_si            = 1'bz;
  assign port_tran_si_dir        = 1'b0;  // SI is input only
  assign port_tran_sck           = 1'bz;
  assign port_tran_sck_dir       = 1'b0;  // clock direction can change
  assign port_tran_sd            = 1'bz;
  assign port_tran_sd_dir        = 1'b0;  // SD is input and not used

  // tie off the rest of the pins we are not using
  // assign cram0_a                 = 'h0;
  // assign cram0_dq                = {16{1'bZ}};
  // assign cram0_clk               = 0;
  // assign cram0_adv_n             = 1;
  // assign cram0_cre               = 0;
  // assign cram0_ce0_n             = 1;
  // assign cram0_ce1_n             = 1;
  // assign cram0_oe_n              = 1;
  // assign cram0_we_n              = 1;
  // assign cram0_ub_n              = 1;
  // assign cram0_lb_n              = 1;

  // assign cram1_a                 = 'h0;
  // assign cram1_dq                = {16{1'bZ}};
  // assign cram1_clk               = 0;
  // assign cram1_adv_n             = 1;
  // assign cram1_cre               = 0;
  // assign cram1_ce0_n             = 1;
  // assign cram1_ce1_n             = 1;
  // assign cram1_oe_n              = 1;
  // assign cram1_we_n              = 1;
  // assign cram1_ub_n              = 1;
  // assign cram1_lb_n              = 1;

  assign dram_a                  = 'h0;
  assign dram_ba                 = 'h0;
  assign dram_dq                 = {16{1'bZ}};
  assign dram_dqm                = 'h0;
  assign dram_clk                = 'h0;
  assign dram_cke                = 'h0;
  assign dram_ras_n              = 'h1;
  assign dram_cas_n              = 'h1;
  assign dram_we_n               = 'h1;

  assign sram_a                  = 'h0;
  assign sram_dq                 = {16{1'bZ}};
  assign sram_oe_n               = 1;
  assign sram_we_n               = 1;
  assign sram_ub_n               = 1;
  assign sram_lb_n               = 1;

  assign dbg_tx                  = 1'bZ;
  assign user1                   = 1'bZ;
  assign aux_scl                 = 1'bZ;
  assign vpll_feed               = 1'bZ;


  // for bridge write data, we just broadcast it to all bus devices
  // for bridge read data, we have to mux it
  // add your own devices here
  always @(*) begin
    casex (bridge_addr)
      default: begin
        bridge_rd_data <= 0;
      end
      32'h10xxxxxx: begin
        // example
        // bridge_rd_data <= example_device_data;
        bridge_rd_data <= 0;
      end
      32'hF8xxxxxx: begin
        bridge_rd_data <= cmd_bridge_rd_data;
      end
    endcase
  end


  //
  // host/target command handler
  //
  wire reset_n;  // driven by host commands, can be used as core-wide reset
  wire [31:0] cmd_bridge_rd_data;

  // bridge host commands
  // synchronous to clk_74a
  wire status_boot_done = pll_core_locked_s;
  wire status_setup_done = pll_core_locked_s;  // rising edge triggers a target command
  wire status_running = reset_n;  // we are running as soon as reset_n goes high

  wire dataslot_requestread;
  wire [15:0] dataslot_requestread_id;
  wire dataslot_requestread_ack = 1;
  wire dataslot_requestread_ok = 1;

  wire dataslot_requestwrite;
  wire [15:0] dataslot_requestwrite_id;
  wire [31:0] dataslot_requestwrite_size;
  wire dataslot_requestwrite_ack = 1;
  wire dataslot_requestwrite_ok = 1;

  wire dataslot_update;
  wire [15:0] dataslot_update_id;
  wire [31:0] dataslot_update_size;

  wire dataslot_allcomplete;

  wire [31:0] rtc_epoch_seconds;
  wire [31:0] rtc_date_bcd;
  wire [31:0] rtc_time_bcd;
  wire rtc_valid;

  wire savestate_supported;
  wire [31:0] savestate_addr;
  wire [31:0] savestate_size;
  wire [31:0] savestate_maxloadsize;

  wire savestate_start;
  wire savestate_start_ack;
  wire savestate_start_busy;
  wire savestate_start_ok;
  wire savestate_start_err;

  wire savestate_load;
  wire savestate_load_ack;
  wire savestate_load_busy;
  wire savestate_load_ok;
  wire savestate_load_err;

  wire osnotify_inmenu;

  // bridge target commands
  // synchronous to clk_74a

  reg target_dataslot_read;
  reg target_dataslot_write;
  reg target_dataslot_getfile;  // require additional param/resp structs to be mapped
  reg target_dataslot_openfile;  // require additional param/resp structs to be mapped

  wire target_dataslot_ack;
  wire target_dataslot_done;
  wire [2:0] target_dataslot_err;

  reg [15:0] target_dataslot_id;
  reg [31:0] target_dataslot_slotoffset;
  reg [31:0] target_dataslot_bridgeaddr;
  reg [31:0] target_dataslot_length;

  wire    [31:0]  target_buffer_param_struct; // to be mapped/implemented when using some Target commands
  wire    [31:0]  target_buffer_resp_struct;  // to be mapped/implemented when using some Target commands

  // bridge data slot access
  // synchronous to clk_74a

  wire [9:0] datatable_addr;
  wire datatable_wren;
  wire [31:0] datatable_data;
  wire [31:0] datatable_q;

  core_bridge_cmd icb (

      .clk    (clk_74a),
      .reset_n(reset_n),

      .bridge_endian_little(bridge_endian_little),
      .bridge_addr         (bridge_addr),
      .bridge_rd           (bridge_rd),
      .bridge_rd_data      (cmd_bridge_rd_data),
      .bridge_wr           (bridge_wr),
      .bridge_wr_data      (bridge_wr_data),

      .status_boot_done (status_boot_done),
      .status_setup_done(status_setup_done),
      .status_running   (status_running),

      .dataslot_requestread    (dataslot_requestread),
      .dataslot_requestread_id (dataslot_requestread_id),
      .dataslot_requestread_ack(dataslot_requestread_ack),
      .dataslot_requestread_ok (dataslot_requestread_ok),

      .dataslot_requestwrite     (dataslot_requestwrite),
      .dataslot_requestwrite_id  (dataslot_requestwrite_id),
      .dataslot_requestwrite_size(dataslot_requestwrite_size),
      .dataslot_requestwrite_ack (dataslot_requestwrite_ack),
      .dataslot_requestwrite_ok  (dataslot_requestwrite_ok),

      .dataslot_update     (dataslot_update),
      .dataslot_update_id  (dataslot_update_id),
      .dataslot_update_size(dataslot_update_size),

      .dataslot_allcomplete(dataslot_allcomplete),

      .rtc_epoch_seconds(rtc_epoch_seconds),
      .rtc_date_bcd     (rtc_date_bcd),
      .rtc_time_bcd     (rtc_time_bcd),
      .rtc_valid        (rtc_valid),

      .savestate_supported  (savestate_supported),
      .savestate_addr       (savestate_addr),
      .savestate_size       (savestate_size),
      .savestate_maxloadsize(savestate_maxloadsize),

      .savestate_start     (savestate_start),
      .savestate_start_ack (savestate_start_ack),
      .savestate_start_busy(savestate_start_busy),
      .savestate_start_ok  (savestate_start_ok),
      .savestate_start_err (savestate_start_err),

      .savestate_load     (savestate_load),
      .savestate_load_ack (savestate_load_ack),
      .savestate_load_busy(savestate_load_busy),
      .savestate_load_ok  (savestate_load_ok),
      .savestate_load_err (savestate_load_err),

      .osnotify_inmenu(osnotify_inmenu),

      .target_dataslot_read    (target_dataslot_read),
      .target_dataslot_write   (target_dataslot_write),
      .target_dataslot_getfile (target_dataslot_getfile),
      .target_dataslot_openfile(target_dataslot_openfile),

      .target_dataslot_ack (target_dataslot_ack),
      .target_dataslot_done(target_dataslot_done),
      .target_dataslot_err (target_dataslot_err),

      .target_dataslot_id        (target_dataslot_id),
      .target_dataslot_slotoffset(target_dataslot_slotoffset),
      .target_dataslot_bridgeaddr(target_dataslot_bridgeaddr),
      .target_dataslot_length    (target_dataslot_length),

      .target_buffer_param_struct(target_buffer_param_struct),
      .target_buffer_resp_struct (target_buffer_resp_struct),

      .datatable_addr(datatable_addr),
      .datatable_wren(datatable_wren),
      .datatable_data(datatable_data),
      .datatable_q   (datatable_q)

  );



  ////////////////////////////////////////////////////////////////////////////////////////
  // Data loading

  reg ioctl_download;
  wire ioctl_wr;
  // Byte addresses
  wire [19:0] ioctl_addr;
  wire [31:0] ioctl_dout;

  always @(posedge clk_74a) begin
    if (dataslot_requestwrite) ioctl_download <= 1;
    else if (dataslot_allcomplete) ioctl_download <= 0;
  end

  data_loader #(
      .ADDRESS_MASK_UPPER_4(4'h1),
      .ADDRESS_SIZE(20)
  ) data_loader (
      .clk_74a(clk_74a),
      .clk_memory(clk_sys_150),

      .bridge_wr(bridge_wr),
      .bridge_endian_little(bridge_endian_little),
      .bridge_addr(bridge_addr),
      .bridge_wr_data(bridge_wr_data),

      .write_en  (ioctl_wr),
      .write_addr(ioctl_addr),
      .write_data(ioctl_dout)
  );

  ////////////////////////////////////////////////////////////////////////////////////////
  // Debug UART

  wire reload_rom_active;
  wire uart_rx;
  wire uart_tx;

  debug_key debug_key (
      .clk(clk_sys_150),

      .cart_tran_bank0_dir(cart_tran_bank0_dir),
      .cart_tran_bank0(cart_tran_bank0),

      .cart_tran_bank3_dir(cart_tran_bank3_dir),
      .cart_tran_bank3(cart_tran_bank3),

      .cart_tran_pin31_dir(cart_tran_pin31_dir),
      .cart_tran_pin31(cart_tran_pin31),

      // IO
      .led(reload_rom_active),
      // .button(cart_button),

      .uart_rx(uart_rx),
      .uart_tx(uart_tx)
  );

  ////////////////////////////////////////////////////////////////////////////////////////
  // Core

  wire [19:0] display_addr;
  wire [15:0] display_data;
  wire        display_wr;
  wire        display_flip_framebuffer;
  wire        display_busy;

  // TODO: Rename
  risc_v #(
      .CLK_SPEED(120_000_000),
      // .BAUDRATE (115200)
      .BAUDRATE (2_000_000)
  ) soc (
      .clk  (clk_sys_150),
      .reset(~reset_n),

      // Program upload
      .ioctl_download(ioctl_download),
      .ioctl_addr(ioctl_addr[19:2]),
      .ioctl_dout(ioctl_dout),
      .ioctl_wr(ioctl_wr),

      // Display IO
      .display_addr(display_addr),
      .display_data(display_data),
      .display_wr(display_wr),
      .display_flip_framebuffer(display_flip_framebuffer),
      .display_busy(display_busy),

      .reload_rom_active(reload_rom_active),

      // Other IO
      .vblank(v_blank),

      // UART
      .uart_rx(uart_rx),
      .uart_tx(uart_tx)
  );

  ////////////////////////////////////////////////////////////////////////////////////////
  // Video

  wire vsync;
  wire hsync;
  wire v_blank;
  wire de;
  wire [23:0] rgb;

  video #(
      .MEM_CLK_SPEED(40_000_000),
      .WIDTH(10'd267),
      .HEIGHT(10'd240)
  ) video (
      .clk_sys(clk_sys_150),
      .clk_mem(clk_mem_50),
      .clk_vid(clk_vid_actual_4),

      // Display IO
      .display_addr(display_addr),
      .display_data(display_data),
      .display_wr(display_wr),
      .display_flip_framebuffer(display_flip_framebuffer),
      .display_busy(display_busy),

      // Video
      .hsync(hsync),
      .vsync(vsync),
      .vblank(v_blank),
      .rgb(rgb),

      .de(de),

      // PSRAM signals
      .cram0_a(cram0_a),
      .cram0_dq(cram0_dq),
      .cram0_wait(cram0_wait),
      .cram0_clk(cram0_clk),
      .cram0_adv_n(cram0_adv_n),
      .cram0_cre(cram0_cre),
      .cram0_ce0_n(cram0_ce0_n),
      .cram0_ce1_n(cram0_ce1_n),
      .cram0_oe_n(cram0_oe_n),
      .cram0_we_n(cram0_we_n),
      .cram0_ub_n(cram0_ub_n),
      .cram0_lb_n(cram0_lb_n),

      .cram1_a(cram1_a),
      .cram1_dq(cram1_dq),
      .cram1_wait(cram1_wait),
      .cram1_clk(cram1_clk),
      .cram1_adv_n(cram1_adv_n),
      .cram1_cre(cram1_cre),
      .cram1_ce0_n(cram1_ce0_n),
      .cram1_ce1_n(cram1_ce1_n),
      .cram1_oe_n(cram1_oe_n),
      .cram1_we_n(cram1_we_n),
      .cram1_ub_n(cram1_ub_n),
      .cram1_lb_n(cram1_lb_n)
  );

  assign video_rgb_clock = clk_vid_10;
  assign video_rgb_clock_90 = clk_vid_10_90deg;
  assign video_rgb = de ? rgb : 24'h0;
  assign video_de = de;
  assign video_skip = 0;
  assign video_vs = vsync;
  assign video_hs = hsync;

  ////////////////////////////////////////////////////////////////////////////////////////
  // Sound

  wire [14:0] audio_l = 15'h0;

  sound_i2s #(
      .CHANNEL_WIDTH(15)
  ) sound_i2s (
      .clk_74a  (clk_74a),
      .clk_audio(clk_sys_150),

      .audio_l(audio_l),
      .audio_r(audio_l),

      .audio_mclk(audio_mclk),
      .audio_lrck(audio_lrck),
      .audio_dac (audio_dac)
  );

  ////////////////////////////////////////////////////////////////////////////////////////
  // PLL

  wire clk_sys_150;
  wire clk_mem_50;
  wire clk_vid_10;
  wire clk_vid_10_90deg;
  wire clk_vid_actual_4;

  wire pll_core_locked;
  wire pll_core_locked_s;
  synch_3 s01 (
      pll_core_locked,
      pll_core_locked_s,
      clk_74a
  );

  mf_pllbase mp1 (
      .refclk(clk_74a),
      .rst   (0),

      .outclk_0(clk_sys_150),
      .outclk_1(clk_mem_50),
      .outclk_2(clk_vid_10),
      .outclk_3(clk_vid_10_90deg),
      .outclk_4(clk_vid_actual_4),

      .locked(pll_core_locked)
  );



endmodule
