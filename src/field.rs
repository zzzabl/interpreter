
pub mod field {
    use crate::bot::bot::Bot;

    enum DirectionEnum {
        Up,
        Right,
        Down,
        Left
    }

    pub struct Field {
        width: u32,
        height: u32,
        walls: Vec<(u32 , u32)>,
        bots: Vec<Bot>
    }

    impl Field {
        pub fn run(&self) {
            loop {
                for bot in self.bots {





                }

            }




        }
    }

    struct BotWrapper {
        bot: Bot,
        x: i32,
        y: i32,
        direction: DirectionEnum
    }
    impl BotWrapper {
        pub fn new(bot: Bot) -> Self {
            BotWrapper {bot, x:0, y:0, direction: DirectionEnum::Up}
        }
        pub fn Step(&mut self, field: Field) -> Result<(), String> {
            let step_result = self.bot.do_step(true);


        }








    }

    pub struct FieldBuilder {
       width: u32,
       height: u32,
       walls: Vec<(u32 , u32)>,
       bots: Vec<Bot>
    }
    impl FieldBuilder {
        pub fn new() -> Self {
            FieldBuilder {
                width: 10,
                height: 10,
                walls: vec![],
                bots: vec![]
            }
        }
        pub fn set_width(mut self, value: u32) -> Self {
            self.width = value;
            self
        }

        pub fn set_height(mut self, value: u32) -> Self {
            self.width = value;
            self
        }

        pub fn add_wall(mut self, value: (u32, u32)) -> Self {
            self.walls.push(value);
            self
        }

        pub fn add_bot(mut self, path: &str) -> Self {
            let mut bot = Bot::new();
            bot.load_and_parse(path)?;
            self.bots.push(bot);
            self
        }

        pub fn build(mut self) -> Field {
            Field {
                width: self.width,
                height: self.height,
                bots: self.bots,
                walls: self.walls
            }
        }

    }
}