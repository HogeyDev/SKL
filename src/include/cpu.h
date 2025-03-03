#ifndef CPU_H
#define CPU_H

#include <stdint.h>
#include <vector>

typedef uint64_t arch;
typedef unsigned char byte;
typedef std::vector<byte> Instruction;
typedef std::vector<Instruction> Program;
#define STACK_SIZE 64 * 1024 // 64 KiB

typedef struct {
    arch rax;
    arch rbx;
    arch rcx;
    arch rdx;

    arch rbp;
    arch rsp;
    arch rip;

    byte memory[STACK_SIZE];
} Cpu;

void print_cpu_state(Cpu *cpu);
void print_stack_context(Cpu *cpu, int addr, int w[2]);
arch load_program(Cpu *cpu, Program program);
void execute_program(Cpu *cpu);

#endif
