use std::fmt::Display;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    IncPtr,
    DecPtr,
    Inc,
    Dec,
    Output,
    Input,
    Loop(Vec<Instruction>),
}

impl Instruction {
    pub fn from<T: Display>(value: T) -> Option<Self> {
        use self::Instruction::*;

        match value.to_string().as_str() {
            "IPT" => Some(IncPtr),
            "DPT" => Some(DecPtr),
            "IVL" => Some(Inc),
            "DVL" => Some(Dec),
            "OUT" => Some(Output),
            "INP" => Some(Input),
            _ => None,
        }
    }

    pub fn into_cmd<T: Display>(code: Vec<T>) -> Result<Vec<Self>> {
        use self::Instruction::*;
        let mut instructions = Vec::new();
        let mut i = 0;

        let code = code.iter().map(|c| c.to_string()).collect::<Vec<String>>();
        while i < code.len() {
            if code[i].as_str() == "CLS" {
                let mut loop_cnt = 1;
                let mut j: usize = i + 1;
                while j < code.len() {
                    match code[j].as_str() {
                        "CLS" => loop_cnt += 1,
                        "CLE" => loop_cnt -= 1,
                        _ => (),
                    }
                    if loop_cnt == 0 {
                        break;
                    }
                    j += 1;
                }
                if loop_cnt != 0 {
                    return Err(anyhow!("Unmatched loop."));
                }
                instructions.push(Loop(Self::into_cmd(code[i + 1..j].to_vec())?));
                i = j;
            } else if let Some(ins) = Instruction::from(code[i].as_str()) {
                instructions.push(ins);
            }

            i += 1;
        }

        Ok(instructions)
    }
}
