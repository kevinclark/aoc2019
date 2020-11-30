use std::convert::TryFrom;

type Memory = Vec<i64>;
type Address = usize;

#[derive(Debug, PartialEq)]
enum Parameter {
    Position { pos: Address },
    Immediate { value: i64 },
}

#[derive(Debug, PartialEq)]
enum Op {
    Unknown, // For memory with an invalid opcode
    Halt,
    Add {
        s1: Parameter,
        s2: Parameter,
        dest: Address,
    },
    Mul {
        s1: Parameter,
        s2: Parameter,
        dest: Address,
    },
    Input {
        dest: Address,
    },
    Output {
        src: Address,
    },
}

#[derive(Debug, PartialEq)]
struct InstructionSpec {
    opcode: u8,
    param_is_immediate: [bool; 3],
}

#[derive(Debug, PartialEq)]
enum Jump {
    Relative(usize),
    Absolute(usize),
    Halt,
}

pub fn load_program(text: &str) -> Memory {
    text.trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse: {:?}", s))
        })
        .collect()
}

fn parse_instruction_spec(int: u16) -> InstructionSpec {
    let mut remaining = int;
    let mut ds: [u8; 5] = [0; 5];

    let mut idx = 4;
    while remaining > 0 {
        ds[idx] = u8::try_from(remaining % 10)
            .expect("Mod 10 doesn't fit into u8. Something is very wrong.");
        remaining /= 10;
        idx -= 1;
    }

    InstructionSpec {
        opcode: u8::try_from((ds[3] * 10) + ds[4])
            .expect("Op code is > 99, which makes little sense"),
        param_is_immediate: [ds[2] == 1, ds[1] == 1, ds[0] == 1],
    }
}

fn build_op(spec: InstructionSpec, slice: &[i64]) -> Op {
    let build_param = |n| {
        if spec.param_is_immediate[n] {
            Parameter::Immediate { value: slice[n] }
        } else {
            Parameter::Position {
                pos: Address::try_from(slice[n])
                    .expect("Positions must be usize convertible"),
            }
        }
    };

    let address_from =
        |n: usize| Address::try_from(slice[n]).expect("{} must be an Address");

    match spec.opcode {
        1 => Op::Add {
            s1: build_param(0),
            s2: build_param(1),
            dest: address_from(2),
        },
        2 => Op::Mul {
            s1: build_param(0),
            s2: build_param(1),
            dest: address_from(2),
        },
        3 => Op::Input {
            dest: address_from(0),
        },
        4 => Op::Output {
            src: address_from(0),
        },
        99 => Op::Halt,
        _ => Op::Unknown,
    }
}

fn next_op(mem: &[i64]) -> Op {
    let instruction = u16::try_from(mem[0])
        .expect(&format!("Instruction too large: {}", mem[0]));

    build_op(parse_instruction_spec(instruction), &mem[1..])
}

fn apply_op<'a>(
    op: &Op,
    mem: &mut Memory,
    inputs: &mut impl Iterator<Item = &'a i64>,
) -> Jump {
    let value_of = |p: &Parameter| match p {
        Parameter::Position { pos } => mem[*pos],
        Parameter::Immediate { value } => *value,
    };

    match op {
        Op::Halt | Op::Unknown => (),
        Op::Add { s1, s2, dest } => mem[*dest] = value_of(s1) + value_of(s2),
        Op::Mul { s1, s2, dest } => mem[*dest] = value_of(s1) * value_of(s2),
        Op::Input { dest } => {
            mem[*dest] = inputs.next().copied().expect("Expected an input")
        }
        Op::Output { src } => println!("Output: {}", mem[*src]),
    }

    match &op {
        Op::Add { .. } | Op::Mul { .. } => Jump::Relative(4),
        Op::Input { .. } | Op::Output { .. } => Jump::Relative(2),
        Op::Halt => Jump::Halt,
        _ => panic!("Unknown op: {:?}", op),
    }
}

pub fn execute<'a>(
    mem: &mut Memory,
    inputs: &mut impl Iterator<Item = &'a i64>,
) {
    let mut ip = 0;

    while ip < mem.len() {
        ip = match apply_op(&next_op(&mem[ip..]), mem, inputs) {
            Jump::Relative(offset) => ip + offset,
            Jump::Absolute(address) => address,
            Jump::Halt => break,
        }
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
        let spec = parse_instruction_spec(99);
        assert_eq!(99, spec.opcode);
        assert_eq!([false, false, false], spec.param_is_immediate);
    }

    #[test]
    fn building_halt() {
        assert_eq!(Op::Halt, next_op(&[99i64]));
    }

    #[test]
    fn building_add() {
        assert_eq!(
            Op::Add {
                s1: Parameter::Position { pos: 9 },
                s2: Parameter::Position { pos: 10 },
                dest: 3
            },
            next_op(&[1, 9, 10, 3])
        );
    }

    #[test]
    fn building_mul() {
        assert_eq!(
            Op::Mul {
                s1: Parameter::Position { pos: 3 },
                s2: Parameter::Position { pos: 11 },
                dest: 0
            },
            next_op(&[2, 3, 11, 0])
        )
    }

    #[test]
    fn application() {
        let mut mem: Memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(4),
            apply_op(
                &Op::Add {
                    s1: Parameter::Position { pos: 9 },
                    s2: Parameter::Position { pos: 10 },
                    dest: 3,
                },
                &mut mem,
                &mut inputs.iter(),
            )
        );

        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);

        assert_eq!(
            Jump::Relative(4),
            apply_op(
                &Op::Mul {
                    s1: Parameter::Position { pos: 3 },
                    s2: Parameter::Position { pos: 11 },
                    dest: 0,
                },
                &mut mem,
                &mut inputs.iter(),
            )
        );

        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);

        assert_eq!(
            Jump::Halt,
            apply_op(&Op::Halt, &mut mem, &mut inputs.iter())
        );

        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);
    }

    #[test]
    fn execute_add() {
        let mut mem = vec![1, 0, 0, 0, 99];
        let input = vec![];
        execute(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn execute_mul() {
        let mut mem = vec![2, 3, 0, 3, 99];
        let input = vec![];
        execute(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn execute_with_trailing_data() {
        let mut mem = vec![2, 4, 4, 5, 99, 0];
        let input = vec![];
        execute(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn execute_instructions_modified() {
        let mut mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let input = vec![];
        execute(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
