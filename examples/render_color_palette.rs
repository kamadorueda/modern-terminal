extern crate modern_terminal;

use modern_terminal::base::console::Console;
use modern_terminal::components::color_palette::ColorPalette;

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let mut console = Console::from_os(&mut stdout);

    console.render(&ColorPalette::new())?;

    Ok(())
}
