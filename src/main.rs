mod bot;
mod field;

use bot::bot::Bot;
use field::field::{FieldBuilder, Field};

fn main() {
    let field = FieldBuilder::new()
        .set_height(20)
        .set_width(20)
        .add_bot("c:\\temp\\test.txt")
        .build();
    field.run();
}
