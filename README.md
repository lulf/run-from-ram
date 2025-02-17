# run-from-ram

Example embedded Rust application that runs in RAM. At startup, the application is copied from flash to RAM before executing. By (ab)using the routines in `cortex-m-rt` for initializing .data section, it copies the entire .text section except the Reset into RAM.

The "special" [`link.x`](link.x) will generate a minimal vector table .vector_table_flash in flash consisting only of
* the stack pointer
* the location of the Reset entry
* the Reset entry generated by cortex-m-rt
 
It will then generate the full .vector_table in located in RAM, and set the `__sdata`, `__edata` and
`__sidata` symbols so that the cortex-m-rt code for copying the `.data` section also copies .text and .rodata into RAM.

It can be run on a Nordic nRF52840 devkit. The application only blinks an LED once it has verified that it's running from RAM.
