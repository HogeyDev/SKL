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
        println!("rax: {}", self.rax);
        println!("rbx: {}", self.rbx);
        println!("rcx: {}", self.rcx);
        println!("rdx: {}", self.rdx);

        println!("rbp: {}", self.rbp);
        println!("rsp: {}", self.rsp);
        println!("rip: {}", self.rip);
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
                    0b1100 => {}
                    0b1101 => {}
                    0b1110 => {}
                    x => panic!("illegal mod bits: {x:x} (opcode: {opcode:x})"),
                }
            }
            opcode => panic!("unknown opcode: {opcode:x}"),
        }
    }
}
