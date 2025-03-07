use cpu::{Cpu, CpuFlag};
use iset::{Instruction, OperandType, OperandValue, Program, Register};

pub mod iset;
pub mod cpu;

fn main() {
    let p: Program = vec![
        Instruction::Mov(
            (OperandType::Register, OperandValue::Reg(Register::Rax)),
            (OperandType::Memory, OperandValue::Imm(0)),
        ),
        Instruction::Hlt,
    ];
    let mut cpu: Cpu = Cpu::new();

    cpu.rsp = cpu.load_program(p);
    cpu.rbp = cpu.rsp;

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
