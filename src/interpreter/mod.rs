use std::{
    io::stdin,
    ops::{AddAssign, SubAssign},
};

use anyhow::Result;
use cell::Cell;
use token::Instruction;

mod cell;
pub mod token;

pub struct Interpreter<T> {
    cells: Vec<Cell<T>>,
    ptr: usize,
}

impl<T> Interpreter<T>
where
    T: Default + AddAssign + SubAssign + From<u8>,
{
    pub fn new() -> Self {
        Interpreter {
            cells: vec![Cell::default()],
            ptr: 0,
        }
    }
    pub fn inc_ptr(&mut self) {
        self.ptr += 1;
        if self.ptr >= self.cells.len() {
            self.cells.push(Cell::default());
        }
    }
    pub fn dec_ptr(&mut self) {
        if self.ptr == 0 {
            self.cells.insert(0, Cell::default());
        } else {
            self.ptr -= 1;
        }
    }
}

#[allow(dead_code)]
impl Interpreter<u8> {
    pub fn run(&mut self, code: Vec<Instruction>) -> Result<()> {
        use Instruction::*;
        for ins in code {
            match ins {
                Loop(body) => self.run(body)?,
                IncPtr => self.inc_ptr(),
                DecPtr => self.dec_ptr(),
                Inc => self.cells[self.ptr].inc(),
                Dec => self.cells[self.ptr].dec(),
                Output => print!("{}", self.cells[self.ptr].to_char()),
                Input => {
                    let mut buf = String::new();
                    stdin().read_line(&mut buf)?;
                    let val = buf.trim().parse()?;
                    self.cells[self.ptr].set_value(val);
                }
            };
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl Interpreter<u32> {
    pub fn run(&mut self, code: Vec<Instruction>) -> Result<()> {
        use Instruction::*;
        for ins in code {
            match ins {
                Loop(body) => self.run(body)?,
                IncPtr => self.inc_ptr(),
                DecPtr => self.dec_ptr(),
                Inc => self.cells[self.ptr].inc(),
                Dec => self.cells[self.ptr].dec(),
                Output => print!("{}", self.cells[self.ptr].to_char()),
                Input => {
                    let mut buf = String::new();
                    stdin().read_line(&mut buf)?;
                    let val = buf.trim().parse()?;
                    self.cells[self.ptr].set_value(val);
                }
            }
        }
        Ok(())
    }
}
