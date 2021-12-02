extern crate modern_terminal;

use modern_terminal::base::console::Console;
use modern_terminal::components::color_palette::ColorPalette;

fn main() -> std::io::Result<()> {
    let mut writer = std::io::stdout();
    let mut console = Console::from_fd(&mut writer);

    let component = ColorPalette::new();

    console.render(&component)?;

    Ok(())
}
