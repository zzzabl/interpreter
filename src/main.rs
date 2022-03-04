mod bot;
mod field;

use field::FieldBuilder;

fn main() {
    let mut field = FieldBuilder::new()
        .set_height(15)
        .set_width(15)
        .add_bot_from_file("c:\\temp\\src.txt")
        .add_wall((4, 4))
        .build();
    field.run();
}
