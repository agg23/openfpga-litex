//--------------------------------------------------------------------------------
// Auto-generated by LiteX (bf081324) on 2023-11-21 11:55:44
//--------------------------------------------------------------------------------
#ifndef __GENERATED_SOC_H
#define __GENERATED_SOC_H
#define CONFIG_CLOCK_FREQUENCY 57120000
#define CONFIG_CPU_HAS_INTERRUPT
#define CONFIG_CPU_RESET_ADDR 0
#define CONFIG_CPU_COUNT 1
#define CONFIG_CPU_ISA "rv32i2p0_mafdc"
#define CONFIG_CPU_MMU "sv32"
#define CONFIG_CPU_DCACHE_SIZE 4096
#define CONFIG_CPU_DCACHE_WAYS 1
#define CONFIG_CPU_DCACHE_BLOCK_SIZE 64
#define CONFIG_CPU_ICACHE_SIZE 4096
#define CONFIG_CPU_ICACHE_WAYS 1
#define CONFIG_CPU_ICACHE_BLOCK_SIZE 64
#define CONFIG_CPU_DTLB_SIZE 4
#define CONFIG_CPU_DTLB_WAYS 4
#define CONFIG_CPU_ITLB_SIZE 4
#define CONFIG_CPU_ITLB_WAYS 4
#define CONFIG_CPU_TYPE_VEXRISCV_SMP
#define CONFIG_CPU_VARIANT_STANDARD
#define CONFIG_CPU_HUMAN_NAME "VexRiscv SMP-STANDARD"
#define CONFIG_CPU_NOP "nop"
#define ROM_BOOT_ADDRESS 1073741824
#define VIDEO_FRAMEBUFFER_BASE 1086324736
#define VIDEO_FRAMEBUFFER_HRES 266
#define VIDEO_FRAMEBUFFER_VRES 240
#define VIDEO_FRAMEBUFFER_DEPTH 16
#define CONFIG_CSR_DATA_WIDTH 32
#define CONFIG_CSR_ALIGNMENT 32
#define CONFIG_BUS_STANDARD "WISHBONE"
#define CONFIG_BUS_DATA_WIDTH 32
#define CONFIG_BUS_ADDRESS_WIDTH 32
#define CONFIG_BUS_BURSTING 0
#define CONFIG_CPU_INTERRUPTS 3
#define TIMER0_INTERRUPT 2
#define UART_INTERRUPT 1

#ifndef __ASSEMBLER__
static inline int config_clock_frequency_read(void) {
	return 57120000;
}
static inline int config_cpu_reset_addr_read(void) {
	return 0;
}
static inline int config_cpu_count_read(void) {
	return 1;
}
static inline const char * config_cpu_isa_read(void) {
	return "rv32i2p0_mafdc";
}
static inline const char * config_cpu_mmu_read(void) {
	return "sv32";
}
static inline int config_cpu_dcache_size_read(void) {
	return 4096;
}
static inline int config_cpu_dcache_ways_read(void) {
	return 1;
}
static inline int config_cpu_dcache_block_size_read(void) {
	return 64;
}
static inline int config_cpu_icache_size_read(void) {
	return 4096;
}
static inline int config_cpu_icache_ways_read(void) {
	return 1;
}
static inline int config_cpu_icache_block_size_read(void) {
	return 64;
}
static inline int config_cpu_dtlb_size_read(void) {
	return 4;
}
static inline int config_cpu_dtlb_ways_read(void) {
	return 4;
}
static inline int config_cpu_itlb_size_read(void) {
	return 4;
}
static inline int config_cpu_itlb_ways_read(void) {
	return 4;
}
static inline const char * config_cpu_human_name_read(void) {
	return "VexRiscv SMP-STANDARD";
}
static inline const char * config_cpu_nop_read(void) {
	return "nop";
}
static inline int rom_boot_address_read(void) {
	return 1073741824;
}
static inline int video_framebuffer_base_read(void) {
	return 1086324736;
}
static inline int video_framebuffer_hres_read(void) {
	return 266;
}
static inline int video_framebuffer_vres_read(void) {
	return 240;
}
static inline int video_framebuffer_depth_read(void) {
	return 16;
}
static inline int config_csr_data_width_read(void) {
	return 32;
}
static inline int config_csr_alignment_read(void) {
	return 32;
}
static inline const char * config_bus_standard_read(void) {
	return "WISHBONE";
}
static inline int config_bus_data_width_read(void) {
	return 32;
}
static inline int config_bus_address_width_read(void) {
	return 32;
}
static inline int config_bus_bursting_read(void) {
	return 0;
}
static inline int config_cpu_interrupts_read(void) {
	return 3;
}
static inline int timer0_interrupt_read(void) {
	return 2;
}
static inline int uart_interrupt_read(void) {
	return 1;
}
#endif // !__ASSEMBLER__

#endif
