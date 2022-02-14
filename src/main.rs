use std::error::Error;

enum OpCode {
    Step,
    TurnLeft,
    TurnRight,
    Check,
    LoopStart,
    LoopEnd
}

enum Command {
    Step,
    TurnLeft,
    TurnRight,
    GoTo(u32),
    Check
}

struct robot {
   program: Vec<Command>,
   x: u32,
   y: u32
}

impl robot {

   fn parse(src: &str) -> Result<Vec<OpCode>,&str> {
       src.split(";").map(|str| {
           match str {
               "step" =>  Ok(OpCode::Step),
               "left" => Ok(OpCode::TurnLeft),
               "right"=> Ok(OpCode::TurnRight),
               "check"=> Ok(OpCode::Check),
               "loop" => Ok(OpCode::LoopStart),
               "endloop" =>  Ok(OpCode::LoopEnd),
               _ => Err(format!("!!{}", str))
           }
       }).collect()
   }


    fn parse1(src: &str) -> Result<Vec<OpCode>,&str> {
         src.split(";").map(|str| {
            match str {
                "check" => Ok(OpCode::Check),
                _ => Err("!!")
            }
        }).collect()
        //result
    }


}













fn main() {
    println!("Hello, world!");
}
