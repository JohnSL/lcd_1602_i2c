/* memory.x - Linker script for the STM32F103RB Nucleo-64 Board */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}