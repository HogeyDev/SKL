use std::u8;

use crate::cpu::{Arch, CpuFlag};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperandType {
    Register,
    Immediate,
    Memory,
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,

    Rbp,
    Rsp,
    Rip,
}

#[derive(Debug, Copy, Clone)]
pub enum OperandValue {
    Imm(Arch),
    Reg(Register),
}

pub type Operand = (OperandType, OperandValue);

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    // TWO OPERANDS
    // [____ 00 11] [2222 3333]
    // 0: imm / reg
    //     first bit is source, second is destination
    //     on means register, off means immediate
    // 1: is memory
    //     first bit is source, second is destination
    //     on means memory, off means reg / mem
    // 2: source register (if applicable)
    // 3: destination register (if applicable)

    // ONE OPERAND
    // [9 8 0 1 2222]
    // 9: is inverse (if jump)
    //     ex: jnz (jump if not zero flag)
    // 8: is relative (if jump)
    // 0: imm / reg
    //     on means register, off means immediate
    // 1: is memory
    //     on means memory, off means reg / mem
    // 2: source register (if applicable)

    Nop,
    Hlt,

    Mov(Operand, Operand),

    Add(Operand, Operand),
    Sub(Operand, Operand),

    Not(Operand),
    Inv(Operand),

    Jmp(bool, Operand),
    JmpCond(bool, bool, CpuFlag, Operand),
    Cmp(Operand, Operand),
}

pub type Program = Vec<Instruction>;

impl Instruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        match *self {
            Self::Nop => vec![0x00],
            Self::Hlt => vec![0x01],
            Self::Mov(src, dest) => {
                let mut operand_bytes = Instruction::double_operand_bytes(src, dest);
                let mut bytes = Vec::from([0x02]);
                bytes.append(&mut operand_bytes);
                bytes
            }
            Self::Add(src, dest) => {
                let mut operand_bytes = Instruction::double_operand_bytes(src, dest);
                let mut bytes = Vec::from([0x03]);
                bytes.append(&mut operand_bytes);
                bytes
            }
            Self::Sub(src, dest) => {
                let mut operand_bytes = Instruction::double_operand_bytes(src, dest);
                let mut bytes = Vec::from([0x04]);
                bytes.append(&mut operand_bytes);
                bytes
            }
            Self::Not(op) => {
                let mut op_bytes = Instruction::single_operand_bytes(op);
                let mut bytes = Vec::from([0x05]);
                bytes.append(&mut op_bytes);
                bytes
            }
            Self::Inv(op) => {
                let mut op_bytes = Instruction::single_operand_bytes(op);
                let mut bytes = Vec::from([0x06]);
                bytes.append(&mut op_bytes);
                bytes
            }
            Self::Jmp(rel, op) => {
                let mut op_bytes = Instruction::single_operand_bytes(op);
                op_bytes[0] |= (rel as u8) << 6;
                let mut bytes = Vec::from([0x07]);
                bytes.append(&mut op_bytes);
                bytes
            }
            Self::JmpCond(rel, inverse, cond, op) => {
                let mut op_bytes = Instruction::single_operand_bytes(op);
                op_bytes[0] |= (inverse as u8) << 7;
                op_bytes[0] |= (rel as u8) << 6;
                let mut bytes = Vec::from([0x08, cond as u8]);
                bytes.append(&mut op_bytes);
                bytes
            }
            Self::Cmp(src, dest) => {
                let mut operand_bytes = Instruction::double_operand_bytes(src, dest);
                let mut bytes = Vec::from([0x09]);
                bytes.append(&mut operand_bytes);
                bytes
            }
        }
    }
    pub fn split_number(n: Arch) -> Vec<u8> {
        let mut bytes = Vec::new();
        for i in 0..size_of::<Arch>() {
            bytes.insert(0, (n >> (8 * i) & 0xff) as u8);
        }
        bytes
    }
    fn single_operand_bytes(src: Operand) -> Vec<u8> {
        let mem = (src.0 == OperandType::Memory) as u8;
        let (is_reg, reg_or_imm) = match src.1 {
            OperandValue::Imm(x) => (0, x),
            OperandValue::Reg(x) => (1, x as Arch),
        };

        let byte = is_reg << 5 | mem << 4 |
            if is_reg != 0 {
                reg_or_imm as u8
            } else { 0 } << 4;

        let mut bytes = vec![byte];
        if is_reg == 0 {
            bytes.append(&mut Instruction::split_number(reg_or_imm));
        }

        bytes
    }
    fn double_operand_bytes(src: Operand, dest: Operand) -> Vec<u8> {
        let src_mem = (src.0 == OperandType::Memory) as u8;
        let dest_mem = (dest.0 == OperandType::Memory) as u8;

        let (src_is_reg, src_reg_or_imm) = match src.1 {
            OperandValue::Imm(x) => (0, x),
            OperandValue::Reg(x) => (1, x as Arch),
        };
        let (dest_is_reg, dest_reg_or_imm) = match dest.1 {
            OperandValue::Imm(x) => (0, x),
            OperandValue::Reg(x) => (1, x as Arch),
        };

        let first = src_is_reg << 3
            | dest_is_reg << 2
            | src_mem << 1
            | dest_mem;
        let second = if src_is_reg != 0 {
                src_reg_or_imm as u8
            } else { 0 } << 4
            | if dest_is_reg != 0 {
                dest_reg_or_imm as u8
            } else { 0 };

        let mut bytes = vec![first, second];
        if src_is_reg == 0 {
            bytes.append(&mut Instruction::split_number(src_reg_or_imm));
        }
        if dest_is_reg == 0 {
            bytes.append(&mut Instruction::split_number(dest_reg_or_imm));
        }

        bytes
    }

    pub fn from_byte(byte: u8) -> Self {
        // only does the opcode for now, as i don't see a huge use in wasting all those cycles on
        // something unnecessessary atm.
        match byte {
            0x00 => Self::Nop,
            0x01 => Self::Hlt,
            opcode => panic!("unknown opcode: {opcode}"),
        }
    }
}
