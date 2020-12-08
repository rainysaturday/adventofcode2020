use std::collections::HashSet;

#[derive(Clone)]
enum Insn {
    JMP(i32),
    ACC(i32),
    NOP(i32),
}

struct Global {
    accumulator: i32,
    pc: i32
}

fn execute(insns: &Vec<Insn>) -> Global {
    let mut global = Global { accumulator: 0, pc: 0 };
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        if visited.get(&global.pc).is_some() {
            break;
        }
        visited.insert(global.pc);
        if global.pc >= insns.len() as i32 || global.pc < 0 {
            println!("Reached outside of the program at {}", global.pc);
            break;
        }

        match insns[global.pc as usize] {
            Insn::ACC(val) => {
                global.accumulator = global.accumulator + val;
            },
            Insn::JMP(val) => {
                global.pc = global.pc + val;
                continue;
            },
            Insn::NOP(_) => {
            },
        }
        global.pc = global.pc + 1;
    }
    global
}

fn main() {
    let insns = include_str!("../../../input08").split("\n").map(|i| {
        let parts = i.split(" ").collect::<Vec<&str>>();
        let val = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "acc" => Insn::ACC(val),
            "jmp" => Insn::JMP(val),
            "nop" => Insn::NOP(val),
            _ => { panic!("Unrecognized instruction: {}", i)}
        }
    }).collect::<Vec<Insn>>();

    let p1global = execute(&insns);
    println!("Accumulator after executing program: {}", p1global.accumulator);

    // Try "Repairing" nops or jmps and test
    for i in 0..insns.len() {
        let mutated = match insns[i] {
            Insn::JMP(val) => {
                let mut mutated = insns.clone();
                mutated[i] = Insn::NOP(val);
                mutated
            },
            Insn::NOP(val) => {
                let mut mutated = insns.clone();
                mutated[i] = Insn::JMP(val);
                mutated
            },
            _ => insns.clone(),
        };

        let glob = execute(&mutated);
        if glob.pc == mutated.len() as i32 {
            println!("Found correct mutation, accumulator value: {}", glob.accumulator);
            break;
        }
    }
}
