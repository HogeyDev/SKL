#ifndef CPU_H
#define CPU_H

#include <cstdint>
#include <vector>

typedef uint64_t arch;
typedef unsigned char byte;
typedef std::vector<byte> Instruction;
typedef std::vector<Instruction> Program;
#define MEMORY_SIZE 16 * 1024 * 1024 // 16 MiB

typedef struct {
    arch rax;
    arch rbx;
    arch rcx;
    arch rdx;

    arch rbp;
    arch rsp;
    arch rip;

    byte *memory;
} Cpu;

#define print_reg(width, n) \
    printf(#n": %0*lx\n", width, cpu->n)

std::vector<byte> split_number(arch n);
Cpu initialize_cpu();
void print_cpu_state(Cpu *cpu);
void print_stack_context(Cpu *cpu, int addr, int w[2]);
arch load_program(Cpu *cpu, Program program);
void next_instruction(Cpu *cpu);
void execute_program(Cpu *cpu);

#endif
