extern crate lipsum;
extern crate modern_terminal;

use lipsum::lipsum;
use modern_terminal::{
    components::text::{Text, TextAlignment},
    core::{console::Console, style::Style},
};

fn main() -> std::io::Result<()> {
    let mut writer = std::io::stdout();
    let mut console = Console::from_fd(&mut writer);

    for color in ["#FFFF00", "blue", "rgb(255, 0, 0)"] {
        let component = Text {
            align:  TextAlignment::Center,
            text:   lipsum(16),
            styles: vec![
                Style::Bold,
                Style::Foreground(color.to_string()),
                Style::Background("black".to_string()),
            ],
        };

        console.render(&component)?;
    }

    Ok(())
}
