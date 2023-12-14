#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// #include <irq.h>
// #include <libbase/uart.h>
// #include <libbase/console.h>
// #include <generated/csr.h>

int main(void)
{
	printf("C: Hello, world!\n");

	char *alloced_pointer = (char *)malloc(18 * sizeof(char));

	strcpy(alloced_pointer, "helloworld");
	printf("Pointer value: %p\n", alloced_pointer);
	printf("Pointer address: %p\n", &alloced_pointer);
	printf("String: %s\n", alloced_pointer);

	while (1) {}

	return 0;
}
