// Cart Pin Synchronizer
// Implements two-stage synchronization for metastability prevention without debouncing
//
// This module provides a robust solution for synchronizing asynchronous cartridge signals
// into the core's clock domain while preventing metastability issues. Key features:
//
// 1. Two-Stage Synchronization: Uses a standard two-register synchronizer to capture
//    signals crossing from an asynchronous domain into the system clock domain
//
// 2. Conditional Sampling: Uses sample_enable to control when new inputs are captured,
//    allowing for controlled sampling only when pins are in input mode
//
// 3. Parameterized Width: Supports different bus widths through the WIDTH parameter,
//    enabling reuse for both 8-bit banks and individual control pins
//
// Usage Notes:
// - The sample_enable input should be driven by the inverse of the direction control
//   signal for each bank (i.e., sample when pins are inputs, not when they're outputs)
// - This synchronizer introduces a 2-cycle latency from raw input to synchronized output
// - All outputs are registered, providing clean transitions and stable timing

module cart_pin_synchronizer #(
    parameter WIDTH = 8          // Number of pins to synchronize
) (
    input                  clk,        // System clock domain
    input                  reset_n,    // Active-low reset
    // When high, capture raw_pins into the synchronizer; otherwise hold last value
    input      [WIDTH-1:0] sample_enable,
    input      [WIDTH-1:0] raw_pins,   // Raw asynchronous input signals
    output reg [WIDTH-1:0] sync_pins   // Synchronized output signals
);
    // Two-stage synchronizer to prevent metastability
    // Apply synthesis preservation attributes to prevent optimization
    (* preserve *) logic [WIDTH-1:0] sync_meta1;  // First stage synchronizer flip-flops
    (* preserve *) logic [WIDTH-1:0] sync_meta2;  // Second stage synchronizer flip-flops
    
    always_ff @(posedge clk or negedge reset_n) begin
        if (!reset_n) begin
            // Initialize all registers to known state on reset
            sync_meta1 <= {WIDTH{1'b0}};
            sync_meta2 <= {WIDTH{1'b0}};
            sync_pins  <= {WIDTH{1'b0}};
        end else begin
            // Two-stage synchronizer for metastability prevention using proper bitwise operations
            // Use bitwise AND/OR/NOT for per-bit control with sample_enable
            sync_meta1 <= (sample_enable & raw_pins) | (~sample_enable & sync_meta1);
            
            // Always propagate through pipeline to maintain synchronization chain
            sync_meta2 <= sync_meta1;
            sync_pins  <= sync_meta2;
        end
    end
endmodule
