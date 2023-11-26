# USER CONFIGURATION
LITEX_ROOT_DIRECTORY = ../../../../litex
OUTPUT_DIRECTORY = ./build

# crt0 MUST be included to properly bootstrap the program
# INSERT YOUR OBJECT FILES HERE:
OBJECTS   = crt0.o hello.o

# Enable to display all commands
# V = 1

# -----------------------------------------------------

BUILD_DIR = $(LITEX_ROOT_DIRECTORY)/build/litex
SOC_DIRECTORY = $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc
LINKER_DIRECTORY = ../../../linker
BASE_LINKER_FILE = $(LINKER_DIRECTORY)/c-linker.ld

# This uses LiteX generated variables, relative to the build directory, which will not be correct if you are using the committed artifacts
include $(BUILD_DIR)/software/include/generated/variables.mak
# These set all of the variables used here (like LDFLAGs)
include $(SOC_DIRECTORY)/software/common.mak

all: build.bin

%.bin: %.elf
	$(OBJCOPY) -O binary $(OUTPUT_DIRECTORY)/$< $(OUTPUT_DIRECTORY)/$@
ifneq ($(OS),Windows_NT)
	chmod -x $(OUTPUT_DIRECTORY)/$@
endif

vpath %.a $(PACKAGES:%=../%)

# -L $(LINKER_DIRECTORY) allows c-linker.ld to:
# INCLUDE output_format.ld
# INCLUDE regions.ld

# -Wl,--whole-archive: Include all objects from subarchives. The unused ones will be removed by --gc-sections
# -Wl,--gc-sections: Garbage collect code and data sections
# -Wl,-Map,$@.map: Build the elf map file

build.elf: $(OBJECTS:%=$(OUTPUT_DIRECTORY)/%)
	$(CC) $(LDFLAGS) -L $(LINKER_DIRECTORY) -T $(BASE_LINKER_FILE) -N -o $(OUTPUT_DIRECTORY)/$@ \
		$(OBJECTS:%=$(OUTPUT_DIRECTORY)/%) \
		$(PACKAGES:%=-L$(BUILD_DIR)/software/%) \
		-Wl,--whole-archive \
		-Wl,--gc-sections \
		-Wl,-Map,$(OUTPUT_DIRECTORY)/$@.map \
		$(LIBS:lib%=-l%)

ifneq ($(OS),Windows_NT)
	chmod -x $(OUTPUT_DIRECTORY)/$@
endif

# Pull in dependency info for *existing* .o files
-include $(OBJECTS:.o=.d)

build.o: CFLAGS   += -w

VPATH = $(BIOS_DIRECTORY):$(BIOS_DIRECTORY)/cmds:$(CPU_DIRECTORY)

# Build the object files from source
$(OUTPUT_DIRECTORY)/%.o: %.cpp
# mkdir is duplicated due to how `common.mak` uses the dependency name
	mkdir -p $(OUTPUT_DIRECTORY)
	$(compilexx)

$(OUTPUT_DIRECTORY)/%.o: %.c
	mkdir -p $(OUTPUT_DIRECTORY)
	$(compile)

$(OUTPUT_DIRECTORY)/%.o: %.S
	mkdir -p $(OUTPUT_DIRECTORY)
	$(assemble)

clean:
	$(RM) -R $(OUTPUT_DIRECTORY)

.PHONY: all clean
