#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let mut cpu = Cpu::new();
    cpu.execute_cycle();
    // println!("{}", cpu.i);
    // cpu.load_program(vec![0x13, 0xC5]);
}

// #[derive(Debug)]
struct Cpu {
    delay_timer: u8,
    sound_timer: u8,
    key: [u8; 16],
    v: [u8; 16],
    memory: [u8; 4096],
    gfx: u16,
    i: u16,
    opcode: u16,
    pc: u16,
    sp: u16,
    stack: [u16; 16],
}

impl Cpu {
    fn new() -> Self {
        Self {
            pc: 0x200,
            opcode: 0,
            i: 0,
            sp: 0,
            v: [0; 16],
            sound_timer: 0,
            delay_timer: 0,
            memory: [0; 4096],
            gfx: 64 * 32,
            key: [0; 16],
            stack: [0; 16],
        }
    }

    fn execute_cycle(&mut self) {
        let opcode: u16 = fetch_opcode(self.memory, self.pc);
        self.execute_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 0 {
                println!("BEEEP! - execute_cycle function",)
            }
            self.sound_timer -= 1;
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {
        println!("{}", opcode & 0xF000);
        match opcode & 0xF000 {
            0xA000 => {
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }
            _ => println!("Unknown opcode: 0x{}", opcode),
        }
    }

    fn load_program(&mut self, program: Vec<u8>) {
        let mut data = vec![0; 0x200];
        for byte in program {
            data.push(byte);
        }
        for (index, &byte) in data.iter().enumerate() {
            self.memory[index] = byte;
        }
    }
}

fn fetch_opcode(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
}
