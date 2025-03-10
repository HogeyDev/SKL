use cpu::{Cpu, CpuFlag};
use iset::{Instruction, OperandType, OperandValue, Program, Register};

pub mod iset;
pub mod cpu;

fn main() {
    let p: Program = vec![
        Instruction::Jmp((OperandType::Immediate, OperandValue::Imm(21))),
        Instruction::Mov(
            (OperandType::Immediate, OperandValue::Imm(0x123)),
            (OperandType::Register, OperandValue::Reg(Register::Rax)),
        ),
        Instruction::Add(
            (OperandType::Immediate, OperandValue::Imm(0x456)),
            (OperandType::Register, OperandValue::Reg(Register::Rax)),
        ),
        Instruction::Hlt,
    ];
    let mut cpu: Cpu = Cpu::new();

    cpu.rsp = cpu.load_program(p);
    cpu.rbp = cpu.rsp;
    cpu.print_stack_context((0, 32));

    loop {
        if !cpu.get_flag(CpuFlag::Halt) {
            cpu.next_instruction();
        } else { // need interrupts for halt to do anything other than hard lock the cpu
            break; // break for now since it is impossible to leave anyway
        }
    }
    cpu.print_registers();
    cpu.print_stack_context((0, 32));
}
