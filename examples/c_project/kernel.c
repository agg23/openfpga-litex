#include <stdint.h>
#include <stddef.h>

unsigned char *uart = (unsigned char *)0x00800028;
unsigned char *second_value = (unsigned char *)0x00800100;
void putchar(char c)
{
  *uart = c;
  return;
}

void print(const char *str)
{
  while (*str != '\0')
  {
    putchar(*str);
    str++;
  }
  return;
}

void main()
{
  print("Hello world!\r\n");
  while (1)
  {
    // Read input from the UART
    putchar(*uart);

    *second_value = *second_value + 1;
  }
  return;
}