use cpu::{Cpu, CpuFlag};
use iset::{Instruction, OperandType, OperandValue, Program, Register};

pub mod iset;
pub mod cpu;

fn main() {
    let p: Program = vec![
        Instruction::Mov(
            (OperandType::Immediate, OperandValue::Imm(12)),
            (OperandType::Register, OperandValue::Reg(Register::Rax)),
        ),
        Instruction::Mov(
            (OperandType::Immediate, OperandValue::Imm(7)),
            (OperandType::Register, OperandValue::Reg(Register::Rbx)),
        ),
        Instruction::Sub(
            (OperandType::Register, OperandValue::Reg(Register::Rax)),
            (OperandType::Register, OperandValue::Reg(Register::Rbx)),
        ),
        // Instruction::Inv((OperandType::Register, OperandValue::Reg(Register::Rax))),
        // Instruction::Inv((OperandType::Memory, OperandValue::Imm(0))),
        // Instruction::Not((OperandType::Register, OperandValue::Reg(Register::Rax))),
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
            break;
        }
    }
    cpu.print_registers();
    cpu.print_stack_context((0, 32));
}
