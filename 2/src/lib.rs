use std::cmp::min;

type Memory = Vec<u32>;

#[derive(Debug, PartialEq)]
pub enum Op {
    Unknown, // For memory with an invalid opcode
    Halt,
    Add { s1: u32, s2: u32, dest: u32 },
    Mul { s1: u32, s2: u32, dest: u32 },
}

pub fn load_program(text: &str) -> Memory {
    text.trim()
        .split(",")
        .map(|s| {
            s.parse::<u32>()
                .expect(&format!("Failed to parse: {:?}", s))
        })
        .collect()
}

pub fn parse_instruction(slice: &[u32]) -> Op {
    match slice[0] {
        1 => Op::Add {
            s1: slice[1],
            s2: slice[2],
            dest: slice[3],
        },
        2 => Op::Mul {
            s1: slice[1],
            s2: slice[2],
            dest: slice[3],
        },
        99 => Op::Halt,
        _ => Op::Unknown,
    }
}

pub fn apply_op(op: &Op, mem: &mut Memory) {
    match op {
        Op::Halt | Op::Unknown => (),
        Op::Add { s1, s2, dest } => mem[*dest as usize] = mem[*s1 as usize] + mem[*s2 as usize],
        Op::Mul { s1, s2, dest } => mem[*dest as usize] = mem[*s1 as usize] * mem[*s2 as usize],
    }
}

pub fn execute(mem: &mut Memory) {
    let mut ip = 0;

    while ip < mem.len() {
        match parse_instruction(&mem[ip..min(ip + 4, mem.len())]) {
            Op::Halt => break,
            o => apply_op(&o, mem),
        }

        ip += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading() {
        assert_eq!(vec![1, 2, 3, 4], load_program("1,2,3,4"))
    }

    #[test]
    fn parsing() {
        assert_eq!(Op::Halt, parse_instruction(&[99]));
        assert_eq!(
            Op::Add {
                s1: 9,
                s2: 10,
                dest: 3
            },
            parse_instruction(&[1, 9, 10, 3])
        );
        assert_eq!(
            Op::Mul {
                s1: 3,
                s2: 11,
                dest: 0
            },
            parse_instruction(&[2, 3, 11, 0])
        )
    }

    #[test]
    fn application() {
        let mut mem: Memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        apply_op(
            &Op::Add {
                s1: 9,
                s2: 10,
                dest: 3,
            },
            &mut mem,
        );

        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);

        apply_op(
            &Op::Mul {
                s1: 3,
                s2: 11,
                dest: 0,
            },
            &mut mem,
        );

        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);

        apply_op(&Op::Halt, &mut mem);

        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);
    }

    #[test]
    fn execution() {
        let mut mem = vec![1, 0, 0, 0, 99];
        execute(&mut mem);
        assert_eq!(mem, vec![2, 0, 0, 0, 99]);

        let mut mem = vec![2, 3, 0, 3, 99];
        execute(&mut mem);
        assert_eq!(mem, vec![2, 3, 0, 6, 99]);

        let mut mem = vec![2, 4, 4, 5, 99, 0];
        execute(&mut mem);
        assert_eq!(mem, vec![2, 4, 4, 5, 99, 9801]);

        let mut mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut mem);
        assert_eq!(mem, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
