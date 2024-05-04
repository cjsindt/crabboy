use std::fmt;
#[cfg(feature = "debug")]
use std::io::{Write};
use crate::memory::Memory;

/* ----- CONSTANT DECLARATIONS ----- */
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

/* ----- TYPE DECLARATIONS ----- */
pub struct DMGCPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    memory: Memory,
    halt: bool,
    #[cfg(feature = "debug")]
    cycle_count: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagRegister,
    h: u8,
    l: u8,
}

#[derive(PartialEq, Clone, Copy)]
struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

/* ----- IMPL DEFINITIONS ----- */
impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagRegister::from(0),
            h: 0,
            l: 0,
        }
    }

    fn af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    fn write_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagRegister::from((value & 0xF0) as u8);
    }

    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn write_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn write_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn write_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

impl std::convert::From<FlagRegister> for u8  {
    fn from(flag: FlagRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

impl fmt::Debug for FlagRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zero: {}, subtract: {}, half_carry: {}, carry: {}", 
            self.zero as u8, self.subtract as u8, self.half_carry as u8, self.carry as u8)
    }
}

impl DMGCPU {
    /* ----- PUBLIC ----- */
    pub fn new() -> DMGCPU {
        let registers = Registers::new();
        let memory = Memory::new();
        #[cfg(feature = "debug")]
        let cycle_count = 0;

        DMGCPU {
            registers,
            pc: 0x0100,
            sp: 0x0000,
            memory,
            halt: false,
            #[cfg(feature = "debug")]
            cycle_count,
        }
    }

    // reset cpu state
    // return true if success, false if fail
    // pub fn reset(&mut self) -> bool {
        
    // }

    pub fn run(&mut self) {
        
        while !self.halt {
            self.cycle();
        }
        #[cfg(feature = "debug")]
        self.cycle_debug();
    }

    /* ----- PRIVATE ----- */
    fn cycle(&mut self) {
        let instr = self.memory.read_byte(self.pc);
        #[cfg(feature = "debug")]
        self.cycle_debug();
        self.pc = match self.execute(instr) {
            Some(value) => value,
            None => {
                panic!("Unknown instruction!");
                0
            }
        };
    }

    fn execute(&mut self, instr: u8) -> Option<u16> {
        match instr {
            0x00 => {   // NOP
                Some(self.pc + 1)
            },    
            0x01 => {   // LD, BC, n16
                let v = self.memory.read_word(self.pc + 1);
                self.registers.write_bc(v);
                Some(self.pc + 3)
            },
            0x02 => {   // LD, [BC], A
                self.memory.write(self.registers.bc() as usize, &[self.registers.a]);
                Some(self.pc + 2)
            },
            0x03 => {   // INC BC
                self.registers.write_bc(self.registers.bc().wrapping_add(1));
                Some(self.pc + 2)
            },
            0x04 => {   //  INC B
                let r = self.registers.b.wrapping_add(1);
                self.registers.b = r;
                self.registers.f.zero = r == 0;
                self.registers.f.half_carry = (self.registers.b & 0x0F) + 1 > 0x0F;
                self.registers.f.subtract = false;
                Some(self.pc + 2)
            },
            0x05 => {   // DEC B
                let r = self.registers.b.wrapping_sub(1);
                self.registers.b = r;
                self.registers.f.zero = r == 0;
                self.registers.f.half_carry = (self.registers.b & 0x0F) + 1 > 0x0F;
                self.registers.f.subtract = false;
                Some(self.pc + 2)
            },
            0x06 => {   //  LD, B, d8
                self.registers.b = self.memory.read_byte(self.pc + 1);
                Some(self.pc + 2)
            },
            0x07 => {   //  RLCA
                let c = self.registers.a & 0x80 == 0x80;
                let r = (self.registers.a << 1) | (if self.registers.f.carry {1} else {0});
                self.registers.a = r;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
                self.registers.f.zero = r == 0;
                self.registers.f.carry = c;
                Some(self.pc + 2)
            },
            0x76 => {   // HALT
                self.halt = true;
                Some(self.pc + 1)
            }
            3_u8..=u8::MAX => todo!()
        }
    }

    #[cfg(feature = "debug")]
    fn cycle_debug(&mut self) {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
    
        writeln!(handle, "------ CYCLE {} ------", self.cycle_count).expect("Failed to write to stdout");
        writeln!(handle, "PC: {:04X}", self.pc).expect("Failed to write to stdout");
        writeln!(handle, "Registers:").expect("Failed to write to stdout");
        writeln!(handle, "  A: {:02X}", self.registers.a).expect("Failed to write to stdout");
        writeln!(handle, "  B: {:02X}", self.registers.b).expect("Failed to write to stdout");
        writeln!(handle, "  C: {:02X}", self.registers.c).expect("Failed to write to stdout");
        writeln!(handle, "  D: {:02X}", self.registers.d).expect("Failed to write to stdout");
        writeln!(handle, "  E: {:02X}", self.registers.e).expect("Failed to write to stdout");
        writeln!(handle, "  F: {:?}", self.registers.f).expect("Failed to write to stdout");
        writeln!(handle, "  H: {:02X}", self.registers.h).expect("Failed to write to stdout");
        writeln!(handle, "  L: {:02X}", self.registers.l).expect("Failed to write to stdout");
        let instr = self.memory.read_byte(self.pc);
        let next_word = self.memory.read_word(self.pc + 1);
        writeln!(handle, "Instruction: {:02X}", instr).expect("Failed to write to stdout");
        writeln!(handle, "Next Word: {:04X}", next_word).expect("Failed to write to stdout");
    
        handle.flush().expect("Failed to flush stdout");
        self.cycle_count += 1;
    }
    
}

/* ----- TESTS ----- */
#[cfg(test)]
mod tests {
    use super::*;

    struct TestDMGCPU {
        cpu: DMGCPU,
        initial_pc: u16,
        initial_registers: Registers,
    }
    
    impl TestDMGCPU {
        fn new() -> Self {
            let mut cpu = DMGCPU::new();
            let initial_pc = cpu.pc;
            let initial_registers = cpu.registers.clone();
            TestDMGCPU {
                cpu,
                initial_pc,
                initial_registers,
            }
        }

        fn cycle(&mut self) {
            self.cpu.cycle();
        }
    }

    #[test]
    fn test_0x00() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x00]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 1);
        assert_eq!(test_cpu.cpu.registers, test_cpu.initial_registers);
    }

    #[test]
    fn test_0x01() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x01, 0xEF, 0xBE]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 3);
        assert_eq!(test_cpu.cpu.registers.bc(), 0xBEEF);
    }

    #[test]
    fn test_0x02() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x02]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 2);
        assert_eq!(test_cpu.cpu.memory.read_byte(test_cpu.cpu.registers.bc()), test_cpu.cpu.registers.a);
    }

    #[test]
    fn test_0x03() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x03]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 2);
        assert_eq!(test_cpu.cpu.registers.bc(), test_cpu.initial_registers.bc().wrapping_add(1));
    }

    #[test]
    fn test_0x04() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x04]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 2);
        assert_eq!(test_cpu.cpu.registers.b, test_cpu.initial_registers.b.wrapping_add(1));
    }

    #[test]
    fn test_0x05() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x05]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 2);
        assert_eq!(test_cpu.cpu.registers.b, test_cpu.initial_registers.b.wrapping_sub(1));
    }

    #[test]
    fn test_0x06() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x06, 0x77]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 2);
        assert_eq!(test_cpu.cpu.registers.b, test_cpu.cpu.memory.read_byte(test_cpu.initial_pc + 1));
    }

    #[test]
    fn test_0x07() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.registers.a = 0b10101010;
        test_cpu.cpu.memory.write(0x0100, &[0x07]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 2);
        assert_eq!(test_cpu.cpu.registers.a, 0b01010100);
        assert_eq!(test_cpu.cpu.registers.f.carry, true);
    }
}
