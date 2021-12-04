extern crate modern_terminal;

use modern_terminal::{
    components::color_palette::ColorPalette,
    core::console::Console,
};

fn main() -> std::io::Result<()> {
    let mut writer = std::io::stdout();
    let mut console = Console::from_fd(&mut writer);

    let component = ColorPalette::new();

    console.render(&component)?;

    Ok(())
}
