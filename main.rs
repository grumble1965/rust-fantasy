use std::vec::Vec;

#[derive(Debug)]
struct Cpu {
	r0: u32,
	r1: u32,
	r2: u32,
	r3: u32,
	stack: Vec<u32>,
}

impl Cpu {
	fn _dump(&self, memory: &Vec<u32>) {
		let mut idx = 0;
		while idx < memory.len() {
			print!("{:02}: ", idx);
			let inst = memory[idx];
			match inst {
				10 => {
					println!("MOVR {}, {}", memory[idx+1], memory[idx+2]);
					idx += 3;
				},
				11 => {
					println!("MOVV {}, {}", memory[idx+1], memory[idx+2]);
					idx += 3;
				},
				20 => {
					println!("ADD {}, {}", memory[idx+1], memory[idx+2]);
					idx += 3;
				},
				21 => {
					println!("SUB {}, {}", memory[idx+1], memory[idx+2]);
					idx += 3;
				},
				30 => {
					println!("PUSH {}", memory[idx+1]);
					idx += 2;
				},
				31 => {
					println!("POP {}", memory[idx+1]);
					idx += 2;
				},
				40 => {
					println!("JP {}", memory[idx+1]);
					idx += 2;
				},
				41 => {
					println!("JL {}, {}, {}", memory[idx+1], memory[idx+2], memory[idx+3]);
					idx += 4;
				},
				42 => {
					println!("CALL {}", memory[idx+1]);
					idx += 2;
				},
				50 => {
					println!("RET");
					idx += 1;
				},
				60 => {
					println!("PRINT {}", memory[idx+1]);
					idx += 2;
				},
				255 => {
					println!("HALT");
					idx += 1;
				},
				_ => {
					println!("??? {}", memory[idx]);
					idx += 1;
				},
			}
		}
	}

	fn run(&mut self, start_pc: usize, memory: &Vec<u32>) {
		let mut pc = start_pc;
		loop {
			pc = self.step(pc, &memory);
		}
	}

	fn step(&mut self, pc: usize, memory: &Vec<u32>) -> usize {
		let inst = memory[pc];
		match inst {
			10 => {
				self.write_register(memory[pc+1], self.read_register(memory[pc+2]));	
				pc + 3
			},
			11 => {
				self.write_register(memory[pc+1], memory[pc+2]);	
				pc + 3
			},
			20 => {
				self.write_register(memory[pc+1], self.read_register(memory[pc+1]) + self.read_register(memory[pc+2]));
				pc + 3
			},
			21 => {
				self.write_register(memory[pc+1], self.read_register(memory[pc+1]) - self.read_register(memory[pc+2]));
				pc + 3
			},
			30 => {
				self.stack.push(self.read_register(memory[pc+1]));
				pc + 2
			},
			31 => {
				let tos = self.stack.pop().unwrap();
				self.write_register(memory[pc+1], tos);
				pc + 2
			},
			40 => {
				memory[pc + 1].try_into().unwrap()
			},
			41 => {
				if self.read_register(memory[pc+1]) < self.read_register(memory[pc+2]) {
					memory[pc + 3].try_into().unwrap()
				} else {
					pc + 3
				}
			},
			42 => {
				self.stack.push( (pc + 2).try_into().unwrap() );
				memory[pc + 1].try_into().unwrap()
			},
			50 => {
				self.stack.pop().unwrap().try_into().unwrap()
			},
			60 => {
				println!("{}", self.read_register(memory[pc + 1]));
				pc + 2
			},
			255 => {
				panic!("HALT")
			},
			_ => panic!("How did we get here?"),
		}
	}

	fn read_register(&self, reg: u32) -> u32 {
		match reg {
			0 => self.r0,
			1 => self.r1,
			2 => self.r2,
			3 => self.r3,
			_ => panic!("attempted read from unknown register {}", reg)
		}
	}

	fn write_register(&mut self, reg: u32, value: u32) {
		match reg {
			0 => self.r0 = value,
			1 => self.r1 = value,
			2 => self.r2 = value,
			3 => self.r3 = value,
			_ => panic!("attempted write to unknown register {}", reg)
		}
	}
}

fn main() {

	let program = vec![
		11,0,10,42,6,255,30,0,11,0,0,11,1,1,11,3,1,60,1,10,2,0,20, 
        2,1,60,2,10,0,1,10,1,2,11,2,1,20,3,2,31,2,30,2,41,3,2,19,31,0,50
	];

	let mut cpu: Cpu = Cpu { 
		r0: 0,
		r1: 0,
		r2: 0,
		r3: 0,
		stack: Vec::<u32>::new()	
	};

	// println!("program has {} bytes", program.len());
	// println!("CPU looks like {:?}", cpu);
	// cpu.dump(&program);

	cpu.run(0, &program);
	// println!("CPU looks like {:?}", cpu);
}
