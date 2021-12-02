extern crate lipsum;
extern crate modern_terminal;

use lipsum::lipsum;
use modern_terminal::{
    base::{console::Console, style::Style},
    components::text::Text,
};

fn main() -> std::io::Result<()> {
    let mut writer = std::io::stdout();
    let mut console = Console::from_fd(&mut writer);

    for color in ["green", "yellow", "blue", "red"] {
        let style = Style::new().foreground(color).bold();
        let text = lipsum(16);

        let component = Text::new(text, style);

        console.render(&component)?;
    }

    Ok(())
}
