MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 2048K /* BANK_1 + BANK_2 */
    RAM   : ORIGIN = 0x20000000, LENGTH =  768K /* SRAM + SRAM2 + SRAM3 */
}

/*
 * ram size = 768K
 * heap size = 512K
 */
/*__luped_ram_size = 512K; 
_stack_end = ORIGIN(RAM) + LENGTH(RAM) - __luped_ram_size;*/
