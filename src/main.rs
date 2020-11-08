#![allow(dead_code)]

/// Default memory size (128MiB)
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

/// This arbitrary CPU contains registers,
/// a program counter, and some memory
struct CPU {
    /// 32 64-bit integer registers
    regs: [u64; 32],

    /// Program counter to hold the memory
    /// address of the next instruction
    pc: u64,

    /// Computer memory to store instructions
    memory: Vec<u8>
}

impl CPU {
    /// Create a new `CPU` object
    fn new(binary: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = MEMORY_SIZE;

        Self {
            regs,
            pc: 0,
            memory: binary
        }
    }

    /// Dump values in all registers (x0 -> x31)
    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..31).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.regs[i],
                    i + 1,
                    abi[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    /// Fetch an instruction from memory
    fn fetch(&self) -> u32 {
        let idx = self.pc as usize;

        return (self.memory[idx] as u32)
            | ((self.memory[idx + 1] as u32) << 8)
            | ((self.memory[idx + 2] as u32) << 16)
            | ((self.memory[idx + 3] as u32) << 24);
    }
}

fn main() {
    println!("zv64");
}
