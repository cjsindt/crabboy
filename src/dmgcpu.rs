/* ----- CONSTANT DECLARATIONS ----- */
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

/* ----- TYPE DECLARATIONS ----- */
pub struct DMGCPU {
    registers: Registers,
    pc: u16,
    memory: Memory,
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

struct Memory {
    memory: [u8; 0xFFFF]
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
        self.a = ((value * 0xFF00) >> 8) as u8;
        self.f = FlagRegister::from((value & 0xF0) as u8);
    }

    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn write_bc(&mut self, value: u16) {
        self.b = ((value * 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn write_de(&mut self, value: u16) {
        self.d = ((value * 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn write_hl(&mut self, value: u16) {
        self.h = ((value * 0xFF00) >> 8) as u8;
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

impl Memory {
    fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF]
        }
    }

    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write(&mut self, address: usize, data: &[u8]) {
        // Ensure the address is within bounds
        assert!(address + data.len() <= self.memory.len(), "Address out of bounds");

        // Write data starting at the specified address
        self.memory[address..(address + data.len())].copy_from_slice(data);
    }
}

impl DMGCPU {
    /* ----- PUBLIC ----- */
    pub fn new() -> DMGCPU {
        let registers = Registers::new();
        let memory = Memory::new();
        DMGCPU {
            registers,
            pc: 0x0100,
            memory,
        }
    }

    // reset cpu state
    // return true if success, false if fail
    // pub fn reset(&mut self) -> bool {
        
    // }

    /* ----- PRIVATE ----- */
    fn cycle(&mut self) {
        let mut instr = self.memory.read(self.pc);
        self.pc = match self.execute(instr) {
            Some(value) => value,
            None => {
                panic!("Unknown instruction!");
                0
            }
        }
    }

    fn execute(&mut self, instr: u8) -> Option<u16> {
        match instr {
            0x00 => {Some(self.pc + 1)}
            1_u8..=u8::MAX => todo!()
        }
    }
}

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
            let cpu = DMGCPU::new();
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
    fn test_nop() {
        let mut test_cpu = TestDMGCPU::new();
        test_cpu.cpu.memory.write(0x0100, &[0x00]);
        test_cpu.cycle();

        assert_eq!(test_cpu.cpu.pc, test_cpu.initial_pc + 1);
        assert_eq!(test_cpu.cpu.registers, test_cpu.initial_registers);
    }
}