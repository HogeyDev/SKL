#ifndef ISET_H
#define ISET_H

typedef enum {
    Nop,        // nothing proceeding
    MovRegReg,  // 00 111 222; mod, src, dest
                // mod bitfield: 2 bit is src, 1 bit is dest
                //     on means memory, off means register
                // src/dest, ordered: rax, rbx, rcx, rdx, rbp, rsp, rip
    MovRegImm,  // 00 ___ 111; mod, dest
                // mod bitfield: 2 bit is imm, 1 bit is dest
                //     on means memory, off means register/immediate
                // src, ordered: rax, rbx, rcx, rdx, rbp, rsp, rip

    AddRegReg,  // 00 111 222; mod, src, dest
                // mod bitfield: 2 bit is src, 1 bit is dest
                //     on means memory, off means register
                // src/dest, ordered: rax, rbx, rcx, rdx, rbp, rsp, rip
    AddRegImm,  // 00 ___ 111; mod, dest
                // mod bitfield: 2 bit is imm, 1 bit is dest
                //     on means memory, off means register/immediate
                // src, ordered: rax, rbx, rcx, rdx, rbp, rsp, rip
} OpCode;

#endif
