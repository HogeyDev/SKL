use std::{num::Wrapping, ptr::with_exposed_provenance};

use crate::iset::{Instruction, Program};

pub type Arch = u64;
const ARCH: usize = size_of::<Arch>();
pub static MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16 MiB

#[derive(Debug, Clone)]
pub struct Cpu {
    pub rax: Arch,
    pub rbx: Arch,
    pub rcx: Arch,
    pub rdx: Arch,

    pub rbp: Arch,
    pub rsp: Arch,
    pub rip: Arch,

    pub flags: u8,

    pub memory: Box<[u8]>,
}

pub enum CpuFlag { // u8
    Halt        = 0x01,
    Overflow    = 0x02,
    Negative    = 0x04,
    Zero        = 0x08,
    Carry       = 0x10,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,

            rbp: 0,
            rsp: 0,
            rip: 0,

            flags: 0x00,

            memory: vec![0; MEMORY_SIZE].into_boxed_slice(),
        }
    }

    pub fn load_program(&mut self, program: Program) -> Arch {
        let mut size: Arch = 0;
        for instruction in &program {
            let raw: Vec<u8> = instruction.to_bytes();
            for byte in raw {
                self.memory[(self.rip + size) as usize] = byte;
                size += 1
            }
        }

        size
    }

    pub fn get_mem(&self, addr: Arch) -> u8 {
        self.memory[addr as usize]
    }
    pub fn set_mem(&mut self, addr: Arch, val: u8) {
        self.memory[addr as usize] = val;
    }
    pub fn set_mem_slice(&mut self, addr: Arch, val: Box<[u8]>) {
        for (i, v) in val.iter().enumerate() {
            self.set_mem(addr + i as Arch, *v);
        }
    }

    pub fn print_stack_context(&self, window: (Arch, Arch)) {
        let left: Arch = Arch::max(window.0, 0);
        let right: Arch = Arch::min(window.1, MEMORY_SIZE as Arch);

        const GROUPING: u64 = 4;
        const BREAKS: u64 = 4;

        let address_width: usize = f32::ceil(f32::log2(MEMORY_SIZE as f32) / 4.0f32) as usize;
        print!("{:0width$x}: ", left, width = address_width);
        for i in left..=right {
            let di: Arch = i - left;
            print!("{:02x}", self.get_mem(i));
            if (di + 1) % GROUPING == 0 {
                print!(" ");
            }

            if (di + 1) % (GROUPING * BREAKS) == 0 {
                print!("\n{:0width$x}: ", i, width = address_width);
            }
            if i == right {
                print!("\n");
            }
        }
    }
    pub fn print_registers(&self) {
        fn disp_reg(name: &str, value: Arch, width: usize) {
            println!("{name}: {:0width$x}", value, width=width*2);
        }

        disp_reg("rax", self.rax, ARCH);
        disp_reg("rbx", self.rbx, ARCH);
        disp_reg("rcx", self.rcx, ARCH);
        disp_reg("rdx", self.rdx, ARCH);

        disp_reg("rbp", self.rbp, ARCH);
        disp_reg("rsp", self.rsp, ARCH);
        disp_reg("rip", self.rip, ARCH);
    }

    pub fn get_flag(&self, bit: CpuFlag) -> bool {
        return self.flags & (bit as u8) > 0
    }
    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        self.flags = self.flags | if value { flag as u8 } else { 0x00 };
    }

    pub fn reg_code(&mut self, code: u8) -> &mut Arch {
        match code {
            0b0000 => &mut self.rax,
            0b0001 => &mut self.rbx,
            0b0010 => &mut self.rcx,
            0b0011 => &mut self.rdx,

            0b0100 => &mut self.rbp,
            0b0101 => &mut self.rsp,
            0b0110 => &mut self.rip,

            x => panic!("unknown register code: {x:x}"),
        }
    }
    pub fn reg_value(&self, code: u8) -> Arch {
        match code {
            0b0000 => self.rax,
            0b0001 => self.rbx,
            0b0010 => self.rcx,
            0b0011 => self.rdx,

            0b0100 => self.rbp,
            0b0101 => self.rsp,
            0b0110 => self.rip,

            x => panic!("unknown register code: {x:x}"),
        }
    }

    pub fn next_instruction(&mut self) {
        let opcode = self.get_mem(self.rip);
        self.rip += 1;
        match opcode {
            0x00 => {},
            0x01 => self.set_flag(CpuFlag::Halt, true),
            0x02 => {
                let locb = self.get_mem(self.rip);
                self.rip += 1;

                let regs = self.get_mem(self.rip);
                let src_reg = regs >> 4 & 0xf;
                let dest_reg = regs & 0xf;

                self.rip += 1;
                match locb & 0xf {
                    0b0001 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;
                        let mut dest_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            dest_value = dest_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        self.set_mem_slice(dest_value, Instruction::split_number(src_value).into_boxed_slice());
                    }
                    0b0100 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        *self.reg_code(dest_reg) = src_value;
                    }
                    0b0101 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let reg_val = *self.reg_code(dest_reg);
                        self.set_mem_slice(reg_val, Instruction::split_number(src_value).into_boxed_slice());
                    }
                    0b0110 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let mem_val = self.get_mem(src_value);
                        *self.reg_code(dest_reg) = mem_val as Arch;
                    }
                    0b1001 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let mem_val = self.get_mem(src_value);
                        *self.reg_code(dest_reg) = mem_val as Arch;
                    }
                    0b1100 => {
                        *self.reg_code(dest_reg) = *self.reg_code(src_reg);
                    }
                    0b1101 => {
                        let val = Instruction::split_number(*self.reg_code(src_reg)).into_boxed_slice();
                        let addr = *self.reg_code(dest_reg);
                        self.set_mem_slice(addr, val);
                    }
                    0b1110 => {
                        let val = *self.reg_code(src_reg);
                        *self.reg_code(dest_reg) = self.get_mem(val) as Arch;
                    }
                    x => panic!("illegal mod bits: {x:x} (opcode: {opcode:x})"),
                }
            }
            0x03 => {
                let locb = self.get_mem(self.rip);
                self.rip += 1;

                let regs = self.get_mem(self.rip);
                let src_reg = regs >> 4 & 0xf;
                let dest_reg = regs & 0xf;

                self.rip += 1;
                match locb & 0xf {
                    0b0001 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;
                        let mut dest_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            dest_value = dest_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let value = Wrapping(self.get_mem(dest_value) as Arch) + Wrapping(src_value);
                        self.set_mem_slice(dest_value, Instruction::split_number(value.0).into_boxed_slice());
                    }
                    0b0100 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) + Wrapping(src_value)).0;
                    }
                    0b0101 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let reg_val = (Wrapping(*self.reg_code(dest_reg)) + Wrapping(src_value)).0;
                        self.set_mem_slice(reg_val, Instruction::split_number(reg_val).into_boxed_slice());
                    }
                    0b0110 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let mem_val = self.get_mem(src_value);
                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) + Wrapping(mem_val as Arch)).0;
                    }
                    0b1001 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let mem_val = self.get_mem(src_value);
                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) + Wrapping(mem_val as Arch)).0;
                    }
                    0b1100 => {
                        let src = self.reg_value(src_reg);
                        let dest = self.reg_code(dest_reg);
                        let val = Wrapping(*dest) + Wrapping(src);
                        *dest = val.0;
                    }
                    0b1101 => {
                        let dest_val = *self.reg_code(dest_reg);
                        let value = Wrapping(dest_val) + Wrapping(self.reg_value(src_reg));
                        let split = Instruction::split_number(value.0).into_boxed_slice();
                        self.set_mem_slice(dest_val, split);
                    }
                    0b1110 => {
                        let val = self.get_mem(self.reg_value(src_reg)) as Arch;
                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) + Wrapping(val)).0;
                    }
                    x => panic!("illegal mod bits: {x:x} (opcode: {opcode:x})"),
                }
            }
            0x04 => {

                let locb = self.get_mem(self.rip);
                self.rip += 1;

                let regs = self.get_mem(self.rip);
                let src_reg = regs >> 4 & 0xf;
                let dest_reg = regs & 0xf;

                self.rip += 1;
                match locb & 0xf {
                    0b0001 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;
                        let mut dest_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            dest_value = dest_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let value = Wrapping(self.get_mem(dest_value) as Arch) - Wrapping(src_value);
                        self.set_mem_slice(dest_value, Instruction::split_number(value.0).into_boxed_slice());
                    }
                    0b0100 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) - Wrapping(src_value)).0;
                    }
                    0b0101 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let reg_val = (Wrapping(*self.reg_code(dest_reg)) - Wrapping(src_value)).0;
                        self.set_mem_slice(reg_val, Instruction::split_number(reg_val).into_boxed_slice());
                    }
                    0b0110 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let mem_val = self.get_mem(src_value);
                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) - Wrapping(mem_val as Arch)).0;
                    }
                    0b1001 => {
                        let mut src_value: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            src_value = src_value << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let mem_val = self.get_mem(src_value);
                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) - Wrapping(mem_val as Arch)).0;
                    }
                    0b1100 => {
                        let src = self.reg_value(src_reg);
                        let dest = self.reg_code(dest_reg);
                        let val = Wrapping(*dest) - Wrapping(src);
                        *dest = val.0;
                    }
                    0b1101 => {
                        let dest_val = *self.reg_code(dest_reg);
                        let value = Wrapping(dest_val) - Wrapping(self.reg_value(src_reg));
                        let split = Instruction::split_number(value.0).into_boxed_slice();
                        self.set_mem_slice(dest_val, split);
                    }
                    0b1110 => {
                        let val = self.get_mem(self.reg_value(src_reg)) as Arch;
                        let dest = self.reg_code(dest_reg);
                        *dest = (Wrapping(*dest) - Wrapping(val)).0;
                    }
                    x => panic!("illegal mod bits: {x:x} (opcode: {opcode:x})"),
                }
            }
            0x05 => {
                let locb = self.get_mem(self.rip);
                self.rip += 1;

                let reg = self.get_mem(self.rip) & 0xf;
                match locb >> 4 & 0b11 {
                    0b01 => {
                        let mut imm: Arch = 0;
                        for i in 0..size_of::<Arch>() {
                            imm = imm << 8 | self.get_mem(self.rip + i as Arch) as Arch; // regs should already be processed, so we shouldn't be off by one (:pray)
                        }
                        self.rip += ARCH as Arch;

                        let val = !self.get_mem(imm);
                        self.set_mem(imm, val);
                    }
                    0b10 => {
                        let reg_addr = self.reg_code(reg);
                        let val = !*reg_addr;
                        *reg_addr = val;
                    }
                    0b11 => {
                        let addr = self.reg_value(reg);
                        let val = !self.get_mem(addr);
                        self.set_mem(addr, val);
                    }
                    x => panic!("illegal mod bits: {x:x} (opcode: {opcode:x})"),
                }
            }
            opcode => panic!("unknown opcode: {opcode:x}"),
        }
    }
}
