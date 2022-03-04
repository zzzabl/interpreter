
use crate::bot::{Bot, BotActionEnum};
use rand::rngs::ThreadRng;
use rand::Rng;
#[derive(Debug, PartialEq, Clone, Copy)]
enum DirectionEnum {
    Up,
    Right,
    Down,
    Left,
}

pub struct Field {
    width: u32,
    height: u32,
    walls: Vec<(u32, u32)>,
    bots: Vec<BotWrapper>,
}

impl Field {
    pub fn run(&mut self) {
        let bot_wraps = &mut self.bots;
        loop {
            for bot_wrap in &mut *bot_wraps {
                bot_wrap
                    .do_step(&self.walls, self.width, self.height)
                    .unwrap();
            }
        }
    }
}

struct BotWrapper {
    bot: Bot,
    x: u32,
    y: u32,
    direction: DirectionEnum,
}

impl BotWrapper {
    pub fn new(bot: Bot, x: u32, y: u32, direction: DirectionEnum) -> Self {
        BotWrapper {
            bot,
            x,
            y,
            direction,
        }
    }
    pub fn do_step(
        &mut self,
        _walls: &[(u32, u32)],
        field_width: u32,
        field_height: u32,
    ) -> Result<(), String> {
        let step_result = self
            .bot
            .do_step(self.calc_can_step(field_width, field_height))
            .ok_or("End")?;
        match step_result {
            BotActionEnum::Step => self.calc_next_position(field_width, field_height),
            BotActionEnum::TurnLeft => self.calc_rotate(&step_result),
            BotActionEnum::TurnRight => self.calc_rotate(&step_result),
            BotActionEnum::Nop => {}
        };
        if step_result != BotActionEnum::Nop {
            println!(
                "command: {:?}   x:{}  y:{}  direction:{:?}",
                step_result, self.x, self.y, self.direction
            );
        }
        Ok(())
    }

    fn calc_next_position(&mut self, field_width: u32, field_height: u32) {
        match self.direction {
            DirectionEnum::Up if self.y > 0 => self.y -= 1,
            DirectionEnum::Down if self.y < field_height - 2 => self.y += 1,
            DirectionEnum::Right if self.x < field_width - 2 => self.x += 1,
            DirectionEnum::Left if self.x > 0 => self.x -= 1,
            _ => {}
        };
    }

    fn calc_rotate(&mut self, turn_direction: &BotActionEnum) {
        let all_directions = [
            DirectionEnum::Up,
            DirectionEnum::Right,
            DirectionEnum::Down,
            DirectionEnum::Left,
        ];
        let current_idx = all_directions
            .iter()
            .position(|el| *el == self.direction)
            .unwrap();
        self.direction = match turn_direction {
            BotActionEnum::TurnLeft => {
                if self.direction == DirectionEnum::Up {
                    DirectionEnum::Left
                } else {
                    all_directions[current_idx - 1]
                }
            }
            BotActionEnum::TurnRight => {
                if self.direction == DirectionEnum::Left {
                    DirectionEnum::Up
                } else {
                    all_directions[current_idx + 1]
                }
            }
            _ => panic!("Быть такого не может!"),
        };
    }

    fn calc_can_step(&self, field_width: u32, field_height: u32) -> bool {
        match self.direction {
            DirectionEnum::Up => self.y > 0,
            DirectionEnum::Down => self.y < field_height - 2,
            DirectionEnum::Right => self.x < field_width - 2,
            DirectionEnum::Left => self.x > 0,
        }
    }
}

pub struct FieldBuilder {
    width: u32,
    height: u32,
    walls: Vec<(u32, u32)>,
    bots: Vec<BotWrapper>,
}

impl FieldBuilder {
    pub fn new() -> Self {
        FieldBuilder {
            width: 10,
            height: 10,
            walls: vec![],
            bots: vec![],
        }
    }

    pub fn set_width(mut self, value: u32) -> Self {
        self.width = value;
        self
    }

    pub fn set_height(mut self, value: u32) -> Self {
        self.height = value;
        self
    }

    pub fn add_wall(mut self, value: (u32, u32)) -> Self {
        self.walls.push(value);
        self
    }

    pub fn add_bot_from_file(mut self, path: &str) -> Self {
        let mut bot = Bot::new();
        bot.load_and_parse(path).unwrap();
        let mut rnd = rand::thread_rng();
        let x = rnd.gen_range(0..(self.width - 1));
        let y = rnd.gen_range(0..(self.height - 1));
        let direction = Self::get_random_direction(rnd);
        self.bots.push(BotWrapper::new(bot, x, y, direction));
        self
    }

    fn get_random_direction(mut rnd: ThreadRng) -> DirectionEnum {
        match rnd.gen_range(0..4) {
            0 => DirectionEnum::Up,
            1 => DirectionEnum::Left,
            2 => DirectionEnum::Down,
            3 => DirectionEnum::Right,
            _ => panic!("А без этого можно как то?"),
        }
    }

    pub fn build(self) -> Field {
        Field {
            width: self.width,
            height: self.height,
            bots: self.bots,
            walls: self.walls,
        }
    }
}
