#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// #include <irq.h>
// #include <libbase/uart.h>
// #include <libbase/console.h>
// #include <generated/csr.h>

class HelloWorld
{
public:
	HelloWorld(const char *message) : message(message) {}

	void print()
	{
		printf("%s", message);
	}

private:
	const char *message;
};

int main()
{
	const char *message = "C++: Hello, World!";

	HelloWorld helloWorld(message);
	helloWorld.print();

	while (1)
	{
	}

	return 0;
}