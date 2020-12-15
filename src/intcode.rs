use std::convert::TryFrom;

type Memory = Vec<i64>;
type Address = usize;

const DEBUG_ON: bool = false;

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(Address),
    Immediate(i64),
}

#[derive(Debug, PartialEq)]
enum Op {
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
        src: Parameter,
    },
    JumpIfTrue {
        cmp: Parameter,
        dest: Parameter,
    },
    JumpIfFalse {
        cmp: Parameter,
        dest: Parameter,
    },
    LessThan {
        s1: Parameter,
        s2: Parameter,
        dest: Address,
    },
    Equals {
        s1: Parameter,
        s2: Parameter,
        dest: Address,
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

#[derive(Debug, PartialEq)]
enum Error {
    UnknownOpcode { spec: InstructionSpec },
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

fn build_op(spec: InstructionSpec, slice: &[i64]) -> Result<Op, Error> {
    use Op::*;

    let build_param = |n| {
        if spec.param_is_immediate[n] {
            Parameter::Immediate(slice[n])
        } else {
            Parameter::Position(
                Address::try_from(slice[n])
                    .expect("Positions must be usize convertible"),
            )
        }
    };

    let address_from = |n: usize| {
        Address::try_from(slice[n])
            .expect(&format!("{} must be an Address", slice[n]))
    };

    let op = match spec.opcode {
        1 => Add {
            s1: build_param(0),
            s2: build_param(1),
            dest: address_from(2),
        },
        2 => Mul {
            s1: build_param(0),
            s2: build_param(1),
            dest: address_from(2),
        },
        3 => Input {
            dest: address_from(0),
        },
        4 => Output {
            src: build_param(0),
        },
        5 => JumpIfTrue {
            cmp: build_param(0),
            dest: build_param(1),
        },
        6 => JumpIfFalse {
            cmp: build_param(0),
            dest: build_param(1),
        },
        7 => LessThan {
            s1: build_param(0),
            s2: build_param(1),
            dest: address_from(2),
        },
        8 => Equals {
            s1: build_param(0),
            s2: build_param(1),
            dest: address_from(2),
        },
        99 => Halt,
        _ => return Err(Error::UnknownOpcode { spec }),
    };

    Ok(op)
}

fn next_op(mem: &[i64]) -> Result<Op, Error> {
    let instruction = u16::try_from(mem[0])
        .expect(&format!("Instruction too large: {}", mem[0]));

    build_op(parse_instruction_spec(instruction), &mem[1..])
}

fn jump_if(dest: Address, f: impl Fn() -> bool) -> Jump {
    if f() {
        Jump::Absolute(dest)
    } else {
        Jump::Relative(3)
    }
}

fn apply_op<'a>(
    op: &Op,
    mem: &mut Memory,
    inputs: &mut impl Iterator<Item = &'a i64>,
    mut output: impl std::io::Write,
) -> Jump {
    use Op::*;

    let value_of = |p: &Parameter| match p {
        Parameter::Position(pos) => mem[*pos],
        Parameter::Immediate(value) => *value,
    };

    match op {
        Halt => (),
        Add { s1, s2, dest } => mem[*dest] = value_of(s1) + value_of(s2),
        Mul { s1, s2, dest } => mem[*dest] = value_of(s1) * value_of(s2),
        Input { dest } => {
            mem[*dest] = inputs.next().copied().expect("Expected an input")
        }
        Output { src } => {
            writeln!(output, "Output: {}", value_of(src)).unwrap();
        }
        JumpIfTrue { cmp, dest } => {
            return jump_if(usize::try_from(value_of(dest)).unwrap(), || {
                value_of(cmp) != 0
            })
        }
        JumpIfFalse { cmp, dest } => {
            return jump_if(usize::try_from(value_of(dest)).unwrap(), || {
                value_of(cmp) == 0
            })
        }
        LessThan { s1, s2, dest } => {
            mem[*dest] = if value_of(s1) < value_of(s2) { 1 } else { 0 }
        }
        Equals { s1, s2, dest } => {
            mem[*dest] = if value_of(s1) == value_of(s2) { 1 } else { 0 }
        }
    }

    match &op {
        Add { .. } | Mul { .. } | LessThan { .. } | Equals { .. } => {
            Jump::Relative(4)
        }
        Input { .. } | Output { .. } => Jump::Relative(2),
        Halt => Jump::Halt,
        _ => panic!("Unknown op: {:?}", op),
    }
}

pub fn execute<'a>(
    mem: &mut Memory,
    inputs: &mut impl Iterator<Item = &'a i64>,
    mut output: impl std::io::Write,
) {
    let mut ticks = 0;
    let mut ip = 0;

    while ip < mem.len() {
        let o = next_op(&mem[ip..]);

        if DEBUG_ON {
            writeln!(output, "Mem:").unwrap();

            let mut offset = 0;
            for line in (*mem).chunks(5).map(|c| {
                c.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("\t")
            }) {
                writeln!(output, "{}|\t{}", offset, line).unwrap();

                offset += 5
            }

            writeln!(output, "Next op: {:?}", o).unwrap();
            writeln!(output, "").unwrap();
            writeln!(output, "").unwrap();
        }

        match o {
            Ok(op) => {
                ip = match apply_op(&op, mem, inputs, &mut output) {
                    Jump::Relative(offset) => ip + offset,
                    Jump::Absolute(address) => address,
                    Jump::Halt => break,
                }
            }
            Err(e) => panic!(
                "Failure at ip: {} ticks: {}\nFailure: {:?}\nMemory: {:?}",
                ip, ticks, e, mem
            ),
        }

        ticks += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execute_to_stdout<'a>(
        mem: &mut Memory,
        inputs: &mut impl Iterator<Item = &'a i64>,
    ) {
        execute(mem, inputs, &mut std::io::stdout())
    }

    fn apply_to_stdout<'a>(
        op: &Op,
        mem: &mut Memory,
        inputs: &mut impl Iterator<Item = &'a i64>,
    ) -> Jump {
        apply_op(op, mem, inputs, &mut std::io::stdout())
    }

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
        assert_eq!(Ok(Op::Halt), next_op(&[99i64]));
    }

    #[test]
    fn building_add() {
        assert_eq!(
            Ok(Op::Add {
                s1: Parameter::Position(9),
                s2: Parameter::Position(10),
                dest: 3
            }),
            next_op(&[1, 9, 10, 3])
        );
    }

    #[test]
    fn building_mul() {
        assert_eq!(
            Ok(Op::Mul {
                s1: Parameter::Position(3),
                s2: Parameter::Position(11),
                dest: 0
            }),
            next_op(&[2, 3, 11, 0])
        )
    }

    #[test]
    fn building_jump_if_true() {
        assert_eq!(
            Ok(Op::JumpIfTrue {
                cmp: Parameter::Immediate(0),
                dest: Parameter::Immediate(1)
            }),
            next_op(&[1105, 0, 1])
        )
    }

    #[test]
    fn building_jump_if_false() {
        assert_eq!(
            Ok(Op::JumpIfFalse {
                cmp: Parameter::Immediate(0),
                dest: Parameter::Immediate(1)
            }),
            next_op(&[1106, 0, 1])
        )
    }

    #[test]
    fn building_less_than() {
        assert_eq!(
            Ok(Op::LessThan {
                s1: Parameter::Immediate(3),
                s2: Parameter::Immediate(4),
                dest: 0
            }),
            next_op(&[1107, 3, 4, 0])
        )
    }

    #[test]
    fn building_equals() {
        assert_eq!(
            Ok(Op::Equals {
                s1: Parameter::Immediate(3),
                s2: Parameter::Immediate(4),
                dest: 0
            }),
            next_op(&[1108, 3, 4, 0])
        )
    }

    #[test]
    fn apply_add() {
        let mut mem: Memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(4),
            apply_to_stdout(
                &Op::Add {
                    s1: Parameter::Position(9),
                    s2: Parameter::Position(10),
                    dest: 3,
                },
                &mut mem,
                &mut inputs.iter(),
            )
        );

        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], mem);
    }

    #[test]
    fn apply_mul() {
        let mut mem: Memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(4),
            apply_to_stdout(
                &Op::Mul {
                    s1: Parameter::Position(3),
                    s2: Parameter::Position(11),
                    dest: 0,
                },
                &mut mem,
                &mut inputs.iter(),
            )
        );

        assert_eq!(vec![150, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], mem);
    }

    #[test]
    fn apply_halt() {
        let mut mem: Memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let inputs = vec![];

        assert_eq!(
            Jump::Halt,
            apply_to_stdout(&Op::Halt, &mut mem, &mut inputs.iter())
        );

        assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], mem);
    }

    #[test]
    fn apply_jump_if_true() {
        let mut mem = vec![5, 0, 40];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(3),
            apply_to_stdout(
                &Op::JumpIfTrue {
                    cmp: Parameter::Immediate(0),
                    dest: Parameter::Immediate(40)
                },
                &mut mem,
                &mut inputs.iter()
            )
        );

        assert_eq!(
            Jump::Absolute(40),
            apply_to_stdout(
                &Op::JumpIfTrue {
                    cmp: Parameter::Immediate(1),
                    dest: Parameter::Immediate(40)
                },
                &mut mem,
                &mut inputs.iter()
            )
        )
    }

    #[test]
    fn apply_jump_if_false() {
        let mut mem = vec![6, 0, 40];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(3),
            apply_to_stdout(
                &Op::JumpIfFalse {
                    cmp: Parameter::Immediate(1),
                    dest: Parameter::Immediate(40)
                },
                &mut mem,
                &mut inputs.iter()
            )
        );

        assert_eq!(
            Jump::Absolute(40),
            apply_to_stdout(
                &Op::JumpIfFalse {
                    cmp: Parameter::Immediate(0),
                    dest: Parameter::Immediate(40)
                },
                &mut mem,
                &mut inputs.iter()
            )
        )
    }

    #[test]
    fn apply_less_than() {
        let mut mem = vec![7];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(4),
            apply_to_stdout(
                &Op::LessThan {
                    s1: Parameter::Immediate(3),
                    s2: Parameter::Immediate(4),
                    dest: 0
                },
                &mut mem,
                &mut inputs.iter()
            )
        );

        assert_eq!(vec![1], mem);

        assert_eq!(
            Jump::Relative(4),
            apply_to_stdout(
                &Op::LessThan {
                    s1: Parameter::Immediate(3),
                    s2: Parameter::Immediate(2),
                    dest: 0
                },
                &mut mem,
                &mut inputs.iter()
            )
        );

        assert_eq!(vec![0], mem)
    }

    #[test]
    fn apply_equals() {
        let mut mem = vec![7];
        let inputs = vec![];

        assert_eq!(
            Jump::Relative(4),
            apply_to_stdout(
                &Op::Equals {
                    s1: Parameter::Immediate(3),
                    s2: Parameter::Immediate(4),
                    dest: 0
                },
                &mut mem,
                &mut inputs.iter()
            )
        );

        assert_eq!(vec![0], mem);

        assert_eq!(
            Jump::Relative(4),
            apply_to_stdout(
                &Op::Equals {
                    s1: Parameter::Immediate(3),
                    s2: Parameter::Immediate(3),
                    dest: 0
                },
                &mut mem,
                &mut inputs.iter()
            )
        );

        assert_eq!(vec![1], mem)
    }

    #[test]
    fn execute_add() {
        let mut mem = vec![1, 0, 0, 0, 99];
        let input = vec![];

        execute_to_stdout(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn execute_mul() {
        let mut mem = vec![2, 3, 0, 3, 99];
        let input = vec![];

        execute_to_stdout(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn execute_with_trailing_data() {
        let mut mem = vec![2, 4, 4, 5, 99, 0];
        let input = vec![];

        execute_to_stdout(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn execute_instructions_modified() {
        let mut mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let input = vec![];
        execute_to_stdout(&mut mem, &mut input.iter());
        assert_eq!(mem, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
