use std::io::prelude::*;
use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let desired = 19690720;
    let mut program: Vec<i64> = Vec::new();
    let mut noun: i64 = 0;
    let mut verb: i64 = 0;
    while program.len() == 0 || program[0] != desired {
        program.clear();
        load_program(&mut program)?;
        // restore program where it crashed
        restore_program(&mut program, noun, verb);

        let mut loc: usize = 0;
        while loc < program.len() {
            match process_opcode(loc, &mut program) {
                Ok(num) => if num == 99 {
                    break;
                },
                //Err(e) => return Err(e),
                Err(_e) => break
            };
            loc += 4;
        }

        verb += 1;
        if verb == 100 {
            verb = 0;
            noun += 1;
            if noun == 100 {
                return Err(Error::new(ErrorKind::Other, "No solution found"));
            }
        }
    }

    println!("{:?}", program);

    Ok(())
}

fn load_program(program: &mut Vec<i64>) -> Result<(), Error> {
    let mut file = File::open("input.txt")?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    for code in source.trim().split(",") {
        program.push(code.parse::<i64>().unwrap());
    }
    Ok(())
}

fn process_opcode(pos: usize, program: &mut Vec<i64>) -> Result<(i8), Error> {
    if program[pos] == 1 {
        // addition
        let dest = program[pos + 3] as usize;
        program[dest] = program[program[pos + 1] as usize] + program[program[pos + 2] as usize];
        Ok(0)
    }
    else if program[pos] == 2 {
        // multiplication
        let dest = program[pos + 3] as usize;
        program[dest] = program[program[pos + 1] as usize] * program[program[pos + 2] as usize];
        Ok(0)
    }
    else if program[pos] == 99 {
        // halt
        Ok(99)
    }
    else {
        // error
        Err(Error::new(ErrorKind::Other, "Unknown opcode occured"))
    }
}

fn restore_program(program: &mut Vec<i64>, noun: i64, verb: i64) {
    program[1] = noun;
    program[2] = verb;
}
