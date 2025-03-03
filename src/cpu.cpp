#include "cpu.h"
#include "iset.h"

#include <cmath>
#include <cstdio>

#define print_reg(width, n) \
    printf(#n": %0*lx\n", width, cpu->n)

void print_cpu_state(Cpu *cpu) {
    static const unsigned int width = sizeof(arch) * 2;
    print_reg(width, rax);
    print_reg(width, rbx);
    print_reg(width, rcx);
    print_reg(width, rdx);
    putchar('\n');
    print_reg(width, rbp);
    print_reg(width, rsp);
    print_reg(width, rip);
}

void print_stack_context(Cpu *cpu, int addr, int w[2]) {
    arch left = std::max(addr + w[0], 0);
    arch right = std::min(addr+ w[1], STACK_SIZE);

    const unsigned int grouping = 4;
    const unsigned int breaks = 4;

    unsigned int address_width = (unsigned int)std::ceil(std::log2((float)STACK_SIZE) / 4.0f);
    printf("%0*lx: ", address_width, left);
    for (arch i = left; i <= right; i++) {
        arch di = i - left;
        printf("%02x", cpu->memory[i]);
        if ((di + 1) % grouping == 0) {
            printf(" ");
        }

        if ((di + 1) % (grouping * breaks) == 0) {
            printf("\n%0*lx: ", address_width, i);
        }
        if (i - right == 0) {
            putchar('\n');
        }
    }
}

arch load_program(Cpu *cpu, Program program) {
    unsigned int offset = 0;
    for (unsigned int i = 0; i < program.size(); i++) {
        Instruction inst = program[i];
        for (unsigned int j = 0; j < inst.size(); j++) {
            cpu->memory[cpu->rip + offset] = program[i][j];
            offset++;
        }
    }
    return offset;
}

arch *id_to_reg(Cpu *cpu, byte id) {
    arch *reg = 0;
    switch (id) {
        case 0b001: reg = &cpu->rax; break;
        case 0b010: reg = &cpu->rbx; break;
        case 0b011: reg = &cpu->rcx; break;
        case 0b100: reg = &cpu->rdx; break;
        case 0b101: reg = &cpu->rbp; break;
        case 0b110: reg = &cpu->rsp; break;
        case 0b111: reg = &cpu->rip; break;
        default:
            fprintf(stderr, "unknown register: %03b\n", id);
    }
    return reg;
}

void execute_program(Cpu *cpu) {
    while (cpu->rip < STACK_SIZE) {
        byte opcode = cpu->memory[cpu->rip];
        cpu->rip++;

        switch (opcode) {
            case Nop: break;
            case MovRegReg: {
                                byte modrm = cpu->memory[cpu->rip];
                                cpu->rip++;
                                switch (modrm >> 6) {
                                    case 0b00: { // reg -> reg
                                                   arch *src = id_to_reg(cpu, modrm >> 3 & 0b111);
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = *src;
                                               }
                                               break;
                                    case 0b01: { // reg -> mem
                                                   arch *src = id_to_reg(cpu, modrm >> 3 & 0b111);
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   cpu->memory[*dest] = *src;
                                               }
                                               break;
                                    case 0b10: { // mem -> reg
                                                   arch *src = id_to_reg(cpu, modrm >> 3 & 0b111);
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = cpu->memory[*src];
                                               } break;
                                    default:
                                               fprintf(stderr, "illegal mod bits: %d (opcode: %d)\n", modrm >> 6, opcode);
                                }
                            }
                            break;
            case MovRegImm: {
                                byte modrm = cpu->memory[cpu->rip];
                                cpu->rip++;
                                switch (modrm >> 6) {
                                    case 0b00: { // imm -> reg
                                                   arch value = 0;
                                                   for (unsigned int i = 0; i < sizeof(arch); i++) {
                                                       byte tmp = cpu->memory[cpu->rip];
                                                       cpu->rip++;
                                                       value <<= 8;
                                                       value |= tmp;
                                                   }
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = value;
                                               }
                                               break;
                                    case 0b01: { // imm -> mem
                                                   arch value = 0;
                                                   for (unsigned int i = 0; i < sizeof(arch); i++) {
                                                       byte tmp = cpu->memory[cpu->rip];
                                                       cpu->rip++;
                                                       value <<= 8;
                                                       value |= tmp;
                                                   }
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   cpu->memory[*dest] = value;
                                               }
                                               break;
                                    case 0b10: { // mem -> reg
                                                   arch value = 0;
                                                   for (unsigned int i = 0; i < sizeof(arch); i++) {
                                                       byte tmp = cpu->memory[cpu->rip];
                                                       cpu->rip++;
                                                       value <<= 8;
                                                       value |= tmp;
                                                   }
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = cpu->memory[value];
                                               }
                                               break;
                                    default:
                                               fprintf(stderr, "illegal mod bits: %d (opcode: %d)\n", modrm >> 6, opcode);
                                }
                            }
                            break;
            case AddRegReg: {
                                byte modrm = cpu->memory[cpu->rip];
                                cpu->rip++;
                                switch (modrm >> 6) {
                                    case 0b00: {
                                                   arch *src = id_to_reg(cpu, modrm >> 3 & 0b111);
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = *dest + *src;
                                               }
                                               break;
                                    case 0b01: {
                                                   arch *src = id_to_reg(cpu, modrm >> 3 & 0b111);
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   cpu->memory[*dest] = cpu->memory[*dest] = *src;
                                               }
                                               break;
                                    case 0b10: {
                                                   arch *src = id_to_reg(cpu, modrm >> 3 & 0b111);
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = *dest + cpu->memory[*src];
                                               }
                                               break;
                                    default:
                                               fprintf(stderr, "illegal mod bits: %d (opcode: %d)\n", modrm >> 6, opcode);
                                }
                            }
                            break;
            case AddRegImm: {
                                byte modrm = cpu->memory[cpu->rip];
                                cpu->rip++;
                                switch (modrm >> 6) {
                                    case 0b00: {
                                                   arch value = 0;
                                                   for (unsigned int i = 0; i < sizeof(arch); i++) {
                                                       byte tmp = cpu->memory[cpu->rip];
                                                       cpu->rip++;
                                                       value <<= 8;
                                                       value |= tmp;
                                                   }
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = *dest + value;
                                               }
                                               break;
                                    case 0b01: {
                                                   arch value = 0;
                                                   for (unsigned int i = 0; i < sizeof(arch); i++) {
                                                       byte tmp = cpu->memory[cpu->rip];
                                                       cpu->rip++;
                                                       value <<= 8;
                                                       value |= tmp;
                                                   }
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   cpu->memory[*dest] = cpu->memory[*dest] + value;
                                               }
                                               break;
                                    case 0b10: {
                                                   arch value = 0;
                                                   for (unsigned int i = 0; i < sizeof(arch); i++) {
                                                       byte tmp = cpu->memory[cpu->rip];
                                                       cpu->rip++;
                                                       value <<= 8;
                                                       value |= tmp;
                                                   }
                                                   arch *dest = id_to_reg(cpu, modrm & 0b111);

                                                   *dest = *dest + cpu->memory[value];
                                               }
                                               break;
                                    default:
                                               fprintf(stderr, "illegal mod bits: %d (opcode: %d)\n", modrm >> 6, opcode);
                                }
                            }
                            break;
            default:
                      fprintf(stderr, "unknown opcode: %02x\n", opcode);
        }
    }
}
