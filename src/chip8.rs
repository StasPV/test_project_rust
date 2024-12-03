use crate::BaseObject;

pub struct Chip8;
impl BaseObject for Chip8 {
    fn new()->Self {
        Chip8{}
    }

    fn run(&self) {
        println!("Упражнение по имитации CPU");

        let mut cpu = CPU::new();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x21; mem[0x001] = 0x00;
        mem[0x002] = 0x21; mem[0x003] = 0x00;
        mem[0x004] = 0x00; mem[0x005] = 0x00;

        mem[0x100] = 0x80; mem[0x101] = 0x14;
        mem[0x102] = 0x80; mem[0x103] = 0x14;
        mem[0x104] = 0x00; mem[0x105] = 0xEE;
        
        cpu.run();
        println!("5 + (10*2) + (10*2) = {}", cpu.registers[0]);
    }

}

struct CPU{
    registers: [u8; 16],
    position_on_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU{
    fn new()->Self{
        CPU{
            registers: [0; 16],
            memory:[0; 4096],
            position_on_memory: 0,
            stack:[0; 16],
            stack_pointer: 0,
        }
    }
    
    fn read_opcode(&self)-> u16{
        let p = self.position_on_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self){
        let mut iter_count: u64 = 0;
        loop {
            let opcode = self.read_opcode();
            self.position_on_memory += 2;
    
            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;

            let nnn = opcode & 0x0fff;
    

            match (c, x, y, d){
                (0x0, 0x0, 0x0, 0x0) =>{
                    println!("Выполнено итераций: {}", iter_count);
                    return;
                },
                (0x0, 0x0, 0xe, 0xe)=> self.ret(),
                (0x2, _,   _,   _  )=> self.call(nnn),
                (0x8, _,   _,   0x4) => self.add_xy(x,y),
                _=>todo!("opcode {:04x}", opcode),
            }
            iter_count += 1;
        }
    }

    fn call(&mut self, addr: u16){
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len(){
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_on_memory as u16;
        self.stack_pointer += 1;
        self.position_on_memory = addr as usize;
    }

    fn ret(&mut self){
        if self.stack_pointer == 0{
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_on_memory = addr as usize;
    }
    
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow{
            self.registers[0xf] = 1;
        }else{
            self.registers[0xf] = 0;
        }
    }
}

#[cfg(test)]
mod chip8_tests{
    use crate::chip8::CPU;
    #[test]
    fn add_5_7(){
        let mut cpu = CPU::new();
        cpu.registers[0] = 5 as u8;
        cpu.registers[1] = 7 as u8;
        let mem = &mut cpu.memory;
        mem[0x000] = 0x80; mem[0x001] = 0x14;
        cpu.run();
        assert_eq!(cpu.registers[0], 12);
    }
}