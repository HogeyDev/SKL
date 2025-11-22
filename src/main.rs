use cpu::{Cpu, CpuFlag};
// use iset::{Instruction, OperandType, OperandValue, Program, Register};

pub mod iset;
pub mod cpu;

fn main() {
    // let p: Program = vec![
    //     Instruction::Mov(
    //         (OperandType::Immediate, OperandValue::Imm(612)),
    //         (OperandType::Register, OperandValue::Reg(Register::Rax)),
    //     ),
    //     Instruction::Mov(
    //         (OperandType::Immediate, OperandValue::Imm(0)),
    //         (OperandType::Register, OperandValue::Reg(Register::Rbx)),
    //     ),
    //     Instruction::Add(
    //         (OperandType::Immediate, OperandValue::Imm(1)),
    //         (OperandType::Register, OperandValue::Reg(Register::Rbx)),
    //     ),
    //     Instruction::Sub(
    //         (OperandType::Immediate, OperandValue::Imm(1)),
    //         (OperandType::Register, OperandValue::Reg(Register::Rax)),
    //     ),
    //     Instruction::Cmp(
    //         (OperandType::Register, OperandValue::Reg(Register::Rax)),
    //         (OperandType::Register, OperandValue::Reg(Register::Rbx)),
    //     ),
    //     Instruction::JmpCond(false, true, CpuFlag::Zero, (OperandType::Immediate, OperandValue::Imm(22))),
    //     Instruction::Hlt,
    // ];
    let args: Vec<String> = std::env::args().collect();

    let mut cpu: Cpu = Cpu::new();

    // cpu.rsp = cpu.load_program_vec(p);
    // let bin: Vec<u8> = cpu.memory[..(cpu.rsp as usize)].to_vec();
    // _ = std::fs::write("raw.bin", bin);
    cpu.rsp = cpu.load_program_file(args.get(1).expect("no input file given"));
    // cpu.rsp = cpu.load_program_vec(vec![
    //     Instruction::JmpCond(false, true, CpuFlag::Overflow, (OperandType::Immediate, OperandValue::Imm(23)))
    // ]);
    cpu.rbp = cpu.rsp;
    cpu.print_stack_context((0, 32));

    loop {
        if !cpu.get_flag(CpuFlag::Halt) {
            cpu.next_instruction();
        } else { // need interrupts for halt to do anything other than hard lock the cpu
            break; // break for now since it is impossible to leave anyway
        }
        // if cpu.rip % 10000 == 0 { eprintln!("{}", cpu.rip); }
    }
    cpu.print_registers();
    cpu.print_stack_context((0, 32));
    cpu.print_flags();
}
