#ifndef __GENERATED_SDRAM_PHY_H
#define __GENERATED_SDRAM_PHY_H

#include <hw/common.h>
#include <generated/csr.h>

#define DFII_CONTROL_SEL 0x01
#define DFII_CONTROL_CKE 0x02
#define DFII_CONTROL_ODT 0x04
#define DFII_CONTROL_RESET_N 0x08

#define DFII_COMMAND_CS 0x01
#define DFII_COMMAND_WE 0x02
#define DFII_COMMAND_CAS 0x04
#define DFII_COMMAND_RAS 0x08
#define DFII_COMMAND_WRDATA 0x10
#define DFII_COMMAND_RDDATA 0x20

#define SDRAM_PHY_HALFRATEGENSDRPHY
#define SDRAM_PHY_XDR 1
#define SDRAM_PHY_DATABITS 16
#define SDRAM_PHY_DFI_DATABITS 16
#define SDRAM_PHY_PHASES 2
#define SDRAM_PHY_CL 3
#define SDRAM_PHY_CWL 3
#define SDRAM_PHY_RDPHASE 0
#define SDRAM_PHY_WRPHASE 0
#define SDRAM_PHY_DQ_DQS_RATIO 8
#define SDRAM_PHY_MODULES 2
#define SDRAM_PHY_SDR
#define SDRAM_PHY_SUPPORTED_MEMORY 0x0000000004000000ULL

void cdelay(int i);

__attribute__((unused)) static inline void command_p0(int cmd)
{
	sdram_dfii_pi0_command_write(cmd);
	sdram_dfii_pi0_command_issue_write(1);
}
__attribute__((unused)) static inline void command_p1(int cmd)
{
	sdram_dfii_pi1_command_write(cmd);
	sdram_dfii_pi1_command_issue_write(1);
}

#define DFII_PIX_DATA_SIZE CSR_SDRAM_DFII_PI0_WRDATA_SIZE

static inline unsigned long sdram_dfii_pix_wrdata_addr(int phase)
{
	switch (phase) {
		case 0: return CSR_SDRAM_DFII_PI0_WRDATA_ADDR;
		case 1: return CSR_SDRAM_DFII_PI1_WRDATA_ADDR;
		default: return 0;
	}
}
static inline unsigned long sdram_dfii_pix_rddata_addr(int phase)
{
	switch (phase) {
		case 0: return CSR_SDRAM_DFII_PI0_RDDATA_ADDR;
		case 1: return CSR_SDRAM_DFII_PI1_RDDATA_ADDR;
		default: return 0;
	}
}

static inline void init_sequence(void)
{
	/* Bring CKE high */
	sdram_dfii_pi0_address_write(0x0);
	sdram_dfii_pi0_baddress_write(0);
	sdram_dfii_control_write(DFII_CONTROL_CKE|DFII_CONTROL_ODT|DFII_CONTROL_RESET_N);
	cdelay(20000);

	/* Precharge All */
	sdram_dfii_pi0_address_write(0x400);
	sdram_dfii_pi0_baddress_write(0);
	command_p0(DFII_COMMAND_RAS|DFII_COMMAND_WE|DFII_COMMAND_CS);

	/* Load Mode Register / Reset DLL, CL=3, BL=2 */
	sdram_dfii_pi0_address_write(0x131);
	sdram_dfii_pi0_baddress_write(0);
	command_p0(DFII_COMMAND_RAS|DFII_COMMAND_CAS|DFII_COMMAND_WE|DFII_COMMAND_CS);
	cdelay(200);

	/* Precharge All */
	sdram_dfii_pi0_address_write(0x400);
	sdram_dfii_pi0_baddress_write(0);
	command_p0(DFII_COMMAND_RAS|DFII_COMMAND_WE|DFII_COMMAND_CS);

	/* Auto Refresh */
	sdram_dfii_pi0_address_write(0x0);
	sdram_dfii_pi0_baddress_write(0);
	command_p0(DFII_COMMAND_RAS|DFII_COMMAND_CAS|DFII_COMMAND_CS);
	cdelay(4);

	/* Auto Refresh */
	sdram_dfii_pi0_address_write(0x0);
	sdram_dfii_pi0_baddress_write(0);
	command_p0(DFII_COMMAND_RAS|DFII_COMMAND_CAS|DFII_COMMAND_CS);
	cdelay(4);

	/* Load Mode Register / CL=3, BL=2 */
	sdram_dfii_pi0_address_write(0x31);
	sdram_dfii_pi0_baddress_write(0);
	command_p0(DFII_COMMAND_RAS|DFII_COMMAND_CAS|DFII_COMMAND_WE|DFII_COMMAND_CS);
	cdelay(200);

}

#endif /* __GENERATED_SDRAM_PHY_H */
