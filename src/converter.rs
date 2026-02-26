use std::{collections::HashMap, fmt::format, str::FromStr};

use anyhow::{Ok, Result, anyhow};

use crate::command::{Command, Segment};

pub trait Converter {
    fn convert(self: &mut Self, command: Command) -> Result<String>;
}

pub struct HackConverter {
    type_count: HashMap<Command, u64>,
}

fn get_segment_label(segment: &Segment) -> String {
    match segment {
        Segment::Argument => "ARG".to_string(),
        Segment::Local => "LCL".to_string(),
        Segment::Static => todo!(),
        Segment::Constant => todo!(),
        Segment::This => "THIS".to_string(),
        Segment::That => "THAT".to_string(),
        Segment::Pointer => String::new(),
        Segment::Temp => String::new(),
    }
}

impl HackConverter {
    pub fn new() -> Self {
        Self {
            type_count: HashMap::new(),
        }
    }

    fn convert_push(self: &Self, segment: Segment, idx: u16) -> Result<String> {
        match segment {
            Segment::Constant => Ok(convert_push_constant(idx)),
            Segment::Temp => Ok(convert_push_temp(idx)),
            _ => {
                let label = get_segment_label(&segment);

                return Ok(format!(
                    "//push {segment} {idx}\n\
                    @{label}\n\
                    D=M\n\
                    @{idx}\n\
                    D=D+A\n\
                    A=D\n\
                    D=M\n\
                    @SP\n\
                    A=M\n\
                    M=D\n\
                    @SP\n\
                    M=M+1\n\
                    ",
                    segment = segment.to_string(),
                    idx = idx
                ));
            }
        }
    }

    fn convert_pop(self: &mut Self, segment: Segment, idx: u16) -> Result<String> {
        match segment {
            Segment::Constant => Err(anyhow!("Cannot pop constant")),
            Segment::Temp => Ok(convert_pop_temp(idx)),
            _ => {
                let command = Command::Pop {
                    segment,
                    index: idx,
                };
                let type_count = self.type_count.get(&command).unwrap_or(&1);
                let label = get_segment_label(&segment);

                let result = format!(
                    "// pop {segment} {idx}
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.{count}
                        M=D
                        @{label}
                        D=M
                        @{idx}
                        D=D+A
                        @tmp2.{count}
                        M=D
                        @tmp.{count}
                        D=M
                        @tmp2.{count}
                        A=M
                        M=D
                        ",
                    count = type_count,
                    label = label,
                    idx = idx,
                    segment = segment.to_string()
                );

                self.type_count.insert(command, type_count + 1);
                Ok(result)
            }
        }
    }

    fn convert_add(self: &Self) -> Result<String> {
        Ok("//add\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        D=M\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        M=D+M\n\
        @SP\n\
        M=M+1"
            .to_string())
    }

    fn convert_sub(self: &Self) -> Result<String> {
        Ok("//sub\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        D=M\n\
        @SP\n\
        M=M-1\n\
        A=M\n\
        M=M-D\n\
        @SP\n\
        M=M+1"
            .to_string())
    }

    fn convert_neg(self: &Self) -> Result<String> {
        Ok("//neg\n\
    	@SP\n\
    	M=M-1\n\
    	A=M\n\
    	M=-M\n\
    	@SP\n\
    	M=M+1"
            .to_string())
    }

    fn convert_and(self: &Self) -> Result<String> {
        Ok("//and\n\
       @SP\n\
       M=M-1\n\
       A=M\n\
       D=M\n\
       @SP\n\
       M=M-1\n\
       A=M\n\
       M=D&M\n\
       @SP\n\
       M=M+1"
            .to_string())
    }

    fn convert_or(self: &Self) -> Result<String> {
        Ok("//or\n\
       @SP\n\
       M=M-1\n\
       A=M\n\
       D=M\n\
       @SP\n\
       M=M-1\n\
       A=M\n\
       M=D|M\n\
       @SP\n\
       M=M+1"
            .to_string())
    }

    fn convert_not(self: &Self) -> Result<String> {
        Ok("//not\n\
           	@SP\n\
           	M=M-1\n\
           	A=M\n\
           	M=!M\n\
           	@SP\n\
           	M=M+1"
            .to_string())
    }

    fn convert_eq(&mut self) -> Result<String> {
        let current_count = self.type_count.get(&Command::Eq).unwrap_or(&1);

        let result = format!(
            "//eq\n\n\
            // load the bottom value from stack\n\
            @SP\n\
            M=M-1\n\
            A=M\n\
            D=M\n\n\
            // load the second bottom value from stack\n\
            @SP\n\
            M=M-1\n\
            A=M\n\n\
            // subtract both numbers\n\
            D=M-D\n\
            @IsEqual.{count}\n\
            D;JEQ\n\
            @NotEqual.{count}\n\
            D;JNE\n\
            (IsEqual.{count})\n\
            @SP\n\
            A=M\n\
            M=-1\n\
            @EqEnd.{count}\n\
            0;JMP\n\
            (NotEqual.{count})\n\
            @SP\n\
            A=M\n\
            M=0\n\
            (EqEnd.{count})\n\
            @SP\n\
            M=M+1\n\
            ",
            count = current_count
        );

        self.type_count.insert(Command::Eq, current_count + 1);
        Ok(result)
    }

    fn convert_gt(&mut self) -> Result<String> {
        let current_count = self.type_count.get(&Command::Gt).unwrap_or(&1);

        let result = format!(
            "//gt\n\
            // load the bottom value from stack\n\
            @SP\n\
            M=M-1\n\
            A=M\n\
            D=M\n\
            // load the second bottom value from stack\n\
            @SP\n\
            M=M-1\n\
            A=M\n\
            // subtract both numbers\n\
            D=M-D\n\
            @IsGreaterThan.{count}\n\
            D;JGT\n\
            @NotGreaterThan.{count}\n\
            D;JLE\n\
            (IsGreaterThan.{count})\n\
            @SP\n\
            A=M\n\
            M=-1\n\
            @GtEnd.{count}\n\
            0;JMP\n\
            (NotGreaterThan.{count})\n\
            @SP\n\
            A=M\n\
            M=0\n\
            // increase stack pointer\n\
            (GtEnd.{count})\n\
            @SP\n\
            M=M+1\n\
            ",
            count = current_count
        );

        self.type_count.insert(Command::Gt, current_count + 1);
        Ok(result)
    }

    fn convert_lt(&mut self) -> std::result::Result<String, anyhow::Error> {
        let current_count = self.type_count.get(&Command::Lt).unwrap_or(&1);

        let result = format!(
            "// lt\n\
            // load the bottom value from stack\n\
            @SP\n\
            M=M-1\n\
            A=M\n\
            D=M\n\
            // load the second bottom value from stack\n\
            @SP\n\
            M=M-1\n\
            A=M\n\
            // subtract both numbers\n\
            D=M-D\n\
            @IsLessThan.{count}\n\
            D;JLT\n\
            @NotLessThan.{count}\n\
            D;JGE\n\
            (IsLessThan.{count})\n\
            @SP\n\
            A=M\n\
            M=-1\n\
            @LtEnd.{count}\n\
            0;JMP\n\
            (NotLessThan.{count})\n\
            @SP\n\
            A=M\n\
            M=0\n\
            // increase stack pointer\n\
            (LtEnd.{count})\n\
            @SP\n\
            M=M+1\n\
            ",
            count = current_count
        );

        self.type_count.insert(Command::Lt, current_count + 1);
        Ok(result)
    }
}

impl Converter for HackConverter {
    fn convert(self: &mut Self, command: Command) -> Result<String> {
        match command {
            Command::Push { segment, index } => self.convert_push(segment, index),
            Command::Pop { segment, index } => self.convert_pop(segment, index),
            Command::Add => self.convert_add(),
            Command::Sub => self.convert_sub(),
            Command::Neg => self.convert_neg(),
            Command::Eq => self.convert_eq(),
            Command::Gt => self.convert_gt(),
            Command::Lt => self.convert_lt(),
            Command::And => self.convert_and(),
            Command::Or => self.convert_or(),
            Command::Not => self.convert_not(),
        }
    }
}

fn convert_push_constant(idx: u16) -> String {
    format!(
        "//push constant {arg}\n\
        @{arg}\n\
        D=A\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n",
        arg = idx
    )
}

fn convert_push_temp(idx: u16) -> String {
    format!(
        "//push temp {arg}\n\
        @5\n\
        D=A\n\
        @{arg}\n\
        D=D+A\n\
        A=D\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n",
        arg = idx
    )
}

fn convert_pop_temp(idx: u16) -> String {
    format!(
        "// pop temp {idx}\n\
            @SP\n\
            M=M-1\n\
            A=M\n\
            D=M\n\
            @tmp\n\
            M=D\n\
            @5 // base address for temp\n\
            D=A\n\
            @{idx}\n\
            D=D+A\n\
            @tmp2\n\
            M=D\n\
            @tmp\n\
            D=M\n\
            @tmp2\n\
            A=M\n\
            M=D\n\
        ",
    )
}
