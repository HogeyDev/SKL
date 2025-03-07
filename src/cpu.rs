use crate::iset::Program;

pub type Arch = u64;
pub static MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16 MiB

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

    pub fn get_flag(&self, bit: CpuFlag) -> bool {
        return self.flags & (bit as u8) > 0
    }
    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        self.flags = self.flags | if value { flag as u8 } else { 0x00 };
    }

    pub fn next_instruction(&mut self) {
        match self.get_mem(self.rip) {
            0x00 => self.rip += 1,
            0x01 => self.set_flag(CpuFlag::Halt, true),
            opcode => panic!("unknown opcode: {opcode:x}"),
        }
    }
}
