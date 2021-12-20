# Changelog

### 0.13.0

- debug information is no longer displayed on stdout, you can now handle it the way you want
- you can define a read-only area in address space

### 0.12.0

- Optional execution speed limitation
- Execute functions return a u32

### 0.11.0

- FIX debug output for RST instructions
- Disassembler displays machine code with operand
- 2021 edition

### 0.10.0

- Disassembler
- Better debug output

### 0.9.0

- execute() returns the number of cycles consumed by the instruction

### 0.8.0

- Fixed a bug impacting direct RST instructions call
- Stabilized the I/O system
- This version runs my teletype emulator and Altair basic

### 0.7.0

- Fixed IN and OUT instructions pc increment
- Interrupts disabled on startup
- Reworked the experimental I/O system

### 0.6.0

- Passing TST8080, 8080PRE, CPUTEST and 8080EXM tests

### 0.5.0

- Interrupts
- Experimental I/O
- Passing TST8080 and 8080PRE tests