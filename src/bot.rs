pub mod bot {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Lines};

    enum OpCodeEnum {
        Step,
        TurnLeft,
        TurnRight,
        LoopStart,
        LoopEnd,
        If,
        EndIf
    }

    #[derive(Debug)]
    enum CommandEnum {
        Step,
        TurnLeft,
        TurnRight,
        GoTo(i32),
        GoToNE(i32),
    }

    #[derive(PartialEq)]
    enum ParserStateEnum {
        Loop,
        If,
        Root
    }
    pub enum BotActionEnum {
        Step,
        TurnLeft,
        TurnRight,
        Nop
    }


    pub struct Bot {
        program: Vec<CommandEnum>,
        command_ptr: i32,
    }

    impl Bot {
        pub fn new() -> Self {
            Self {program: vec![], command_ptr: -1}
        }
        pub fn load_and_parse(&mut self, path_to_file: &str) -> Result<(), String>{
            let file = File::open(path_to_file).map_err(|d| d.to_string())?;
            let mut op_codes = self.lex(io::BufReader::new(file).lines())?;
            self.program = self.parse(&mut op_codes, 0, ParserStateEnum::Root)?;
            Ok(())
        }

        fn lex(&mut self, src: Lines<BufReader<File>>) -> Result<Vec<OpCodeEnum>,String> {
            src.map(|str| {
                match str.as_deref() {
                    Ok("if") => Ok(OpCodeEnum::If),
                    Ok("endIf") => Ok(OpCodeEnum::EndIf),
                    Ok("step") => Ok(OpCodeEnum::Step),
                    Ok("left") => Ok(OpCodeEnum::TurnLeft),
                    Ok("right")=> Ok(OpCodeEnum::TurnRight),
                    Ok("loop") => Ok(OpCodeEnum::LoopStart),
                    Ok("endLoop") => Ok(OpCodeEnum::LoopEnd),
                    Ok(other) => Err(format!("Нет такой комманды: {}", other )),
                    Err(e) => Err(format!("что то случилось{}", e)),
                }
            }).collect()
        }

        fn parse(&mut self, src: &mut Vec<OpCodeEnum>, mut ptr: i32, state: ParserStateEnum) -> Result<Vec<CommandEnum>, String> {
            let mut result = Vec::new();
            while let Some(code) = src.pop() {
                match code {
                    OpCodeEnum::TurnLeft => {
                        result.push(CommandEnum::TurnLeft);
                        ptr += 1;
                    },
                    OpCodeEnum::TurnRight => {
                        result.push(CommandEnum::TurnRight);
                        ptr += 1;
                    },
                    OpCodeEnum::Step => {
                        result.push(CommandEnum::Step);
                        ptr += 1;
                    },
                    OpCodeEnum::If => {
                        ptr += 1; //будет одна инструкция gotoE
                        let mut part = self.parse(src, ptr, ParserStateEnum::If ).unwrap();
                        result.push(CommandEnum::GoToNE((ptr as usize + part.len()).try_into().unwrap()));
                        result.append(&mut part);
                    },
                    OpCodeEnum::EndIf => {
                        if state == ParserStateEnum::If {
                            return Ok(result);
                        }
                        return Err("Что то не так с циклами и ифами".to_string());
                    },
                    OpCodeEnum::LoopStart => {
                        ptr += 1; //будет одна инструкция gotoNE
                        let mut part = self.parse(src, ptr, ParserStateEnum::Loop ).unwrap();
                        result.push(CommandEnum::GoToNE((ptr as usize + part.len() + 1).try_into().unwrap()));  //+1 т.к. будет еще одна инструкция goto для цикла
                        result.append(&mut part);
                        result.push(CommandEnum::GoTo(ptr - 1));
                        ptr = (ptr as usize + part.len()).try_into().unwrap();
                    }
                    OpCodeEnum::LoopEnd => {
                        if state == ParserStateEnum::Loop {
                            return Ok(result);
                        }
                        return Err("Что то не так с циклами и ифами".to_string());
                    }
                }
            }
            if state != ParserStateEnum::Root {
                return Err("Похоже есть if или цикл не закрытый".to_string());
            }
            Ok(result)
        }

        pub fn do_step(&mut self, can_step: bool) -> Option<BotActionEnum> {
            self.command_ptr += 1;
            if self.command_ptr as usize == self.program.len() {
                return None;
            }

            let current_command = &self.program[self.command_ptr as usize];
            match current_command {
                CommandEnum::Step => Some(BotActionEnum::Step),
                CommandEnum::TurnLeft => Some(BotActionEnum::TurnLeft),
                CommandEnum::TurnRight => Some(BotActionEnum::TurnRight),
                CommandEnum::GoTo(new_ptr) => {
                    self.command_ptr = *new_ptr;
                    Some(BotActionEnum::Nop)
                },
                CommandEnum::GoToNE(new_ptr) => {
                    if can_step {
                        self.command_ptr = *new_ptr
                    }
                    Some(BotActionEnum::Nop)
                }
            }
        }
    }
}