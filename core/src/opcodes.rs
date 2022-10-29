use crate::{cpu::Cpu, display::Display, memory::Memory};

pub fn opcode0(cpu: &mut Cpu, display: &mut Display, n: u8) {
    match n {
        // clear screen
        0 => {
            println!("00");
            display.clear();
            cpu.pc += 2;
        }
        // Return from a subroutine.
        0xE => {
            println!("0E");
            cpu.sp -= 1;
            cpu.pc = cpu.stack[cpu.sp as usize];
            cpu.pc += 2;
        }
        _ => (),
    }
}

pub fn opcode1(cpu: &mut Cpu, nnn: u16) {
    // Jump to location nnn
    println!("1");
    cpu.pc = nnn;
}

pub fn opcode2(cpu: &mut Cpu, nnn: u16) {
    // Call subroutine at nnn.
    println!("2");
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = nnn;
}

pub fn opcode3(cpu: &mut Cpu, x: u8, lo: u8) {
    // Skip next instruction if Vx = kk.
    println!("3");
    if cpu.vx[x as usize] == lo {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn opcode4(cpu: &mut Cpu, x: u8, lo: u8) {
    // Skip next instruction if Vx != kk.
    println!("4");
    if cpu.vx[x as usize] != lo {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn opcode5(cpu: &mut Cpu, x: u8, y: u8) {
    // Skip next instruction if Vx = Vy.
    println!("5");
    if cpu.vx[x as usize] == cpu.vx[y as usize] {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn opcode6(cpu: &mut Cpu, x: u8, lo: u8) {
    // Set Vx = kk.
    println!("6");
    cpu.vx[x as usize] = lo;
    cpu.pc += 2;
}

pub fn opcode7(cpu: &mut Cpu, x: u8, lo: u8) {
    // Set Vx = Vx + kk.
    println!("7");
    cpu.vx[x as usize] = lo.wrapping_add(cpu.vx[x as usize]);
    cpu.pc += 2;
}

pub fn opcode8(cpu: &mut Cpu, n: u8, x: u8, y: u8) {
    match n {
        // Set Vx = Vy.
        0 => {
            println!("80");
            cpu.vx[x as usize] = cpu.vx[y as usize];
            cpu.pc += 2;
        }
        // Set Vx = Vx OR Vy.
        1 => {
            println!("81");
            cpu.vx[x as usize] |= cpu.vx[y as usize];
            cpu.pc += 2;
        }
        // Set Vx = Vx AND Vy.
        2 => {
            println!("82");
            cpu.vx[x as usize] &= cpu.vx[y as usize];
            cpu.pc += 2;
        }
        // Set Vx = Vx XOR Vy.
        3 => {
            println!("83");
            cpu.vx[x as usize] = cpu.vx[x as usize] ^ cpu.vx[y as usize];
            cpu.pc += 2;
        }
        // Set Vx = Vx + Vy, set VF = carry.
        4 => {
            println!("84");
            let one = cpu.vx[x as usize];
            let two = cpu.vx[y as usize];

            if (one as u16 + two as u16) > u8::MAX as u16 {
                cpu.vx[0xF] = 1
            } else {
                cpu.vx[0xF] = 0
            }

            cpu.vx[x as usize] = one.wrapping_add(two); // MAYBE NOT WRAPPING ADD
            cpu.pc += 2
        }
        // Set Vx = Vx - Vy, set VF = NOT borrow.
        5 => {
            println!("85");
            let one = cpu.vx[x as usize];
            let two = cpu.vx[y as usize];

            if one < two {
                cpu.vx[0xF] = 0
            } else {
                cpu.vx[0xF] = 1
            }

            cpu.vx[x as usize] = one.wrapping_sub(two); // MAYBE NOT WRAPPING SUB
            cpu.pc += 2;
        }
        // Set Vx = Vx SHR 1.
        6 => {
            println!("86");
            let one = cpu.vx[x as usize];

            if (one << 7) >> 7 == 1 {
                cpu.vx[0xF] = 1
            } else {
                cpu.vx[0xF] = 0
            }

            cpu.vx[x as usize] = cpu.vx[x as usize] >> 1;
            cpu.pc += 2
        }
        // Set Vx = Vy - Vx, set VF = NOT borrow.
        7 => {
            println!("87");
            let one = cpu.vx[x as usize];
            let two = cpu.vx[y as usize];

            if two > one {
                cpu.vx[0xF] = 1
            } else {
                cpu.vx[0xF] = 0
            }

            cpu.vx[x as usize] = two.wrapping_sub(one); // MAYBE NOT WRAPPING SUB
            cpu.pc += 2
        }
        // Set Vx = Vx SHL 1.
        0xE => {
            println!("8E");
            let one = cpu.vx[x as usize];

            if one >> 7 == 1 {
                cpu.vx[0xF] = 1
            } else {
                cpu.vx[0xF] = 0
            }

            cpu.vx[x as usize] = cpu.vx[x as usize] << 1;
            cpu.pc += 2
        }
        _ => (),
    }
}

pub fn opcode9(cpu: &mut Cpu, x: u8, y: u8) {
    // Skip next instruction if Vx != Vy.
    println!("9");
    if cpu.vx[x as usize] != cpu.vx[y as usize] {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn opcodeA(cpu: &mut Cpu, nnn: u16) {
    // Set I = nnn.
    println!("A");
    cpu.i = nnn;
    cpu.pc += 2;
}

pub fn opcodeB(cpu: &mut Cpu, nnn: u16) {
    // Jump to location nnn + V0.
    println!("B");
    cpu.pc = cpu.vx[0] as u16 + nnn;
}

pub fn opcodeC(cpu: &mut Cpu, x: u8, lo: u8) {
    // Set Vx = random byte AND kk.
    println!("C");
    let random_num: u8 = rand::random();
    cpu.vx[x as usize] = random_num & lo;
    cpu.pc += 2;
}

pub fn opcodeD(cpu: &mut Cpu, display: &mut Display, memory: &mut Memory, n: u8, x: u8, y: u8) {
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    println!("D");
    let x_pos = cpu.vx[x as usize] % 64;
    let y_pos = cpu.vx[y as usize] % 32;
    let mut sprite: Vec<u8> = Vec::new();
    for index in 0..n {
        sprite.push(memory.read((cpu.i + index as u16).into()))
    }
    let collision = display.draw_sprite(&sprite, x_pos, y_pos);

    if collision {
        cpu.vx[0xF] = 1
    } else {
        cpu.vx[0xF] = 0
    }

    cpu.pc += 2;
}

pub fn opcodeE(cpu: &mut Cpu, n: u8, x: u8) {
    match n {
        // Skip next instruction if key with the value of Vx is pressed.
        0xE => {
            println!("EE");
            if cpu.keys[cpu.vx[x as usize] as usize] {
                cpu.pc += 2
            }
            cpu.pc += 2;
        }
        // Skip next instruction if key with the value of Vx is not pressed.
        0x1 => {
            println!("E1");
            if !cpu.keys[cpu.vx[x as usize] as usize] {
                cpu.pc += 2
            }
            cpu.pc += 2;
        }
        _ => {}
    }
}

pub fn opcodeF(cpu: &mut Cpu, memory: &mut Memory, x: u8, lo: u8) {
    match lo {
        // Set Vx = delay timer value.
        0x07 => {
            println!("F07");
            cpu.vx[x as usize] = cpu.delay;
            cpu.pc += 2;
        }
        // Wait for a key press, store the value of the key in Vx.
        0x0A => {
            println!("F0A");
            for i in 0..cpu.keys.len() {
                if cpu.keys[i] {
                    cpu.vx[x as usize] = i as u8;
                    cpu.pc += 2;
                    break;
                }
            }
        }
        // Set delay timer = Vx.
        0x15 => {
            println!("F15");
            cpu.delay = cpu.vx[x as usize];
            cpu.pc += 2;
        }
        // Set sound timer = Vx.
        0x18 => {
            println!("F18");
            cpu.sound = cpu.vx[x as usize];
            cpu.pc += 2;
        }
        // Set I = I + Vx.
        0x1E => {
            println!("F1E");
            cpu.i = cpu.i + cpu.vx[x as usize] as u16;
            cpu.pc += 2;
        }
        // Set I = location of sprite for digit Vx.
        0x29 => {
            println!("F29");
            cpu.i = (cpu.vx[x as usize] * 5) as u16;
            cpu.pc += 2
        }
        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
        0x33 => {
            println!("F33");
            let num = cpu.vx[x as usize];
            println!(
                "{} {} {} {}",
                num,
                num / 100,
                (num % 100) / 10,
                (num % 100) % 10
            );
            memory.write_to_ram(cpu.i.into(), num / 100);
            memory.write_to_ram((cpu.i + 1).into(), (num % 100) / 10);
            memory.write_to_ram((cpu.i + 2).into(), (num % 100) % 10);
            cpu.pc += 2
        }
        // Store registers V0 through Vx in memory starting at location I.
        0x55 => {
            println!("F55");
            for index in 0..(x + 1) {
                memory.write_to_ram((cpu.i + index as u16).into(), cpu.vx[index as usize])
            }
            cpu.pc += 2;
        }
        // Read registers V0 through Vx from memory starting at location I.
        0x65 => {
            println!("F65");
            for index in 0..(x + 1) {
                cpu.vx[index as usize] = memory.read((cpu.i + index as u16).into());
            }
            cpu.pc += 2;
        }
        _ => {}
    }
}
