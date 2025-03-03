#include <cstdio>

#include "cpu.h"
#include "iset.h"

int main(void) {
    Program program = {
        /* mov rax, 1 */ { MovRegImm, 0b00000001, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 },
        /* mov rbx, 64 */  { MovRegImm, 0b00000010, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40 },
        /* add rbx, qword [rax] */ { AddRegReg, 0b10001010 },
        /* add rax, 68 */ { AddRegImm, 0b00000001, 0,0,0,0,0,0,0,0x44 },
    };

    Cpu cpu = { 0 };

    cpu.rbp = load_program(&cpu, program);
    cpu.rsp = cpu.rbp;
    execute_program(&cpu);

    print_stack_context(&cpu, 0, (int[2]){ 0, 11 });
    print_cpu_state(&cpu);

    return 0;
}
