#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

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

    /// Execute an instruction after decoding
    fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x0000007f;
        let rd =  ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;

        // Emulate that register x0 is hardwired with all bits equal to 0
        self.regs[0] = 0;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2])
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: zv64 <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    // Create an instance of CPU
    let mut cpu = CPU::new(binary);

    // Dump empty registers
    cpu.dump_registers();

    while cpu.pc < cpu.memory.len() as u64 {
        // 1. Fetch instruction
        let inst = cpu.fetch();

        // 2. Add 4 to the program counter
        cpu.pc += 4;
        print!("{}", cpu.pc);

        // 3. Decode

        // 4. Execute
        cpu.execute(inst);
    }
    cpu.dump_registers();

    // Exit with unix exit code 0
    Ok(())
}
