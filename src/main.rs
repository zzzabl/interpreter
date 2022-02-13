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
   fn parse(src: &str) -> Result<Vec<OpCode>,String> {
       src.split(";").map(|str| {
           Some(OpCode::Check)
       }).try_for_each(|code|   )
   }
}













fn main() {
    println!("Hello, world!");
}
