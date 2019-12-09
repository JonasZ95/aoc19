
#![allow(dead_code)]

use crate::*;
use std::str::FromStr;
use std::collections::VecDeque;
use std::iter::{once};


const MAX_STEPS: usize = 500_000;

#[derive(Clone)]
pub struct Data ( pub Vec<isize> );

impl FromStr for Data {
    type Err = AocErr;
    fn from_str(s: &str) -> AocResult<Data> {
        let mut v = Vec::new();
        for s in s.split(',') {
            v.push(s.parse()?);
        }
        Ok(Data(v))
    }
}


#[derive(Debug)]
enum Value {
    Position(usize),
    Immediate(isize),
    Relative(isize)
}

#[derive(Debug)]
enum Opcode {
    Add(Value, Value, Value),
    Mul(Value, Value, Value),
    In(Value),
    Out(Value),
    JumpTrue(Value, Value),
    JumpFalse(Value, Value),
    CmpLt(Value, Value, Value),
    CmpEq(Value, Value, Value),
    Halt,
    SetBase(Value)
}

pub struct Context {
    data: Vec<isize>,
    input: VecDeque<isize>,
    output: Vec<isize>,
    pc: usize,
    halted: bool,
    base: isize
}

fn decode_val(value: isize, mode: u8) -> Value {
    match mode {
        0 => Value::Position(value as usize),
        1 => Value::Immediate(value),
        2 => Value::Relative(value),
        _ => unreachable!()
    }
}

fn decode(data: &[isize]) -> AocResult<(Opcode, usize)> {
    let mut op = data[0];

    let third_mode = (op/10_000) as u8;
    op %= 10_000;

    let second_mode = (op/1_000) as u8;
    op %= 1_000;

    let first_mode = (op/100) as u8;
    op %= 100;


    Ok(match op {
        1 => (Opcode::Add(decode_val(data[1], first_mode), decode_val(data[2], second_mode), decode_val(data[3], third_mode)), 4),
        2 => (Opcode::Mul(decode_val(data[1], first_mode), decode_val(data[2], second_mode), decode_val(data[3], third_mode)), 4),
        3 => (Opcode::In(decode_val(data[1], first_mode)), 2),
        4 => (Opcode::Out(decode_val(data[1], first_mode)), 2),
        5 => (Opcode::JumpTrue(decode_val(data[1], first_mode), decode_val(data[2], second_mode)), 3),
        6 => (Opcode::JumpFalse(decode_val(data[1], first_mode), decode_val(data[2], second_mode)), 3),
        7 => (Opcode::CmpLt(decode_val(data[1], first_mode), decode_val(data[2], second_mode), decode_val(data[3], third_mode)), 4),
        8 => (Opcode::CmpEq(decode_val(data[1], first_mode), decode_val(data[2], second_mode), decode_val(data[3], third_mode)), 4),
        9 => (Opcode::SetBase(decode_val(data[1], first_mode)), 2),
        99 => (Opcode::Halt, 1),
        _ => return Err(AocErr::Custom(format!("Invalid Opcode {}", data[0])))
    })
}

impl Context {
    pub fn from_data(data: Data, inputs: &[isize]) -> Context {
        let input = inputs.iter()
            .cloned()
            .collect();

        Context{
            data: data.0,
            input,
            output: Vec::new(),
            pc: 0,
            halted: false,
            base: 0
        }
    }

    pub fn from_data_fill_up(mut data: Data, inputs: &[isize]) -> Context {
        let fill = 16_000 - data.0.len();
        data.0.extend(once(0).cycle().take(fill));
        Self::from_data(data, inputs)
    }

    pub fn data(&self) -> &[isize] {
        &self.data
    }

    pub fn read(&self, ix: usize) -> isize {
        self.data[ix]
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn output(&self) -> Option<isize> {
        self.output.last().cloned()
    }

    pub fn outputs(&self) -> &[isize] {
        &self.output
    }

    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    fn bool_to_num(b: bool)  -> isize{
        if b {
            1
        } else {
            0
        }
    }

    fn jump_if(&mut self, cond: bool, dst: &Value) {
        if cond {
            self.pc = self.read_val(dst) as usize
        }
    }

    fn read_val(&self, val: & Value) -> isize {
        match val {
            Value::Immediate(val) => *val,
            Value::Position(ix) => self.data[*ix],
            Value::Relative(off) => self.data[(self.base + *off) as usize]
        }
    }


    fn write_val(&mut self, val: &Value, value: isize) {
        match val {
            Value::Position(val) => self.data[*val] = value,
            Value::Relative(off) => self.data[(self.base + *off) as usize] = value,
            _ => unreachable!()
        }
    }


    pub fn resume(&mut self) -> AocResult<usize> {
        for cycle in 0..MAX_STEPS {
            let (op,ln) = decode(&self.data[self.pc..])?;
            self.pc += ln;

            match op {
                Opcode::Halt => {
                    self.halted = true;
                    return Ok(cycle);
                },
                Opcode::Add(a, b, c) => {
                    let value = self.read_val(&a) + self.read_val(&b);
                    self.write_val(&c, value);
                },
                Opcode::Mul(a, b, c) => {
                    let value = self.read_val(&a) * self.read_val(&b);
                    self.write_val(&c, value);
                },
                Opcode::In(a) => {
                    let value = self.input.pop_front().ok_or_else(|| custom_err("Not enough inputs"))?;
                    self.write_val(&a, value);
                },
                Opcode::Out(a) => {
                    self.output.push(self.read_val(&a));
                    return Ok(cycle);
                },
                Opcode::JumpTrue(a, dst) => self.jump_if(self.read_val(&a) != 0, &dst),
                Opcode::JumpFalse(a, dst)  => self.jump_if(self.read_val(&a) == 0, &dst),
                Opcode::CmpLt(a, b, dst) => {
                        let value = Self::bool_to_num(self.read_val(&a) < self.read_val(&b));
                        self.write_val(&dst, value);
                },
                Opcode::CmpEq(a, b, dst) => {
                    let value = Self::bool_to_num(self.read_val(&a) == self.read_val(&b));
                    self.write_val(&dst, value);
                },
                Opcode::SetBase(a) => self.base += self.read_val(&a)
            }
        }

        Err(AocErr::Custom("Exceeded max steps".to_string()))
    }


    pub fn exec(&mut self) -> AocResult<isize> {
        let mut cycle = 0;
        while cycle <= MAX_STEPS {
            cycle += self.resume()?;
            if self.halted() {
                return Ok(self.output().unwrap());
            }
        }

        Err(AocErr::Custom("Exceeded max steps".to_string()))
    }
}


fn run(data: Data, input: isize) -> AocResult<isize> {
    let mut ctx = Context::from_data(data, &[input]);

    ctx.exec()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_jump() -> AocResult<()> {
        let data: Data = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".parse()?;


        assert_eq!(999, run(data.clone(), 7)?);
        assert_eq!(1000, run(data.clone(), 8)?);
        assert_eq!(1001, run(data.clone(), 9)?);

        Ok(())
    }

 
    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, 5, 1)?;
        assert_eq!(13_547_311, run(data, 1)?);

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, 5, 1)?;
        assert_eq!(236_453, run(data, 5)?);

        Ok(())
    }

}