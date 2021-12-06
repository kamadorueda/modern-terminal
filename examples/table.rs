extern crate modern_terminal;

use modern_terminal::{
    components::{
        table::{Size, Table},
        text::{Text, TextAlignment},
    },
    core::{console::Console, style::Style},
};

fn main() -> std::io::Result<()> {
    let mut writer = std::io::stdout();
    let mut console = Console::from_fd(&mut writer);

    let component = Table {
        column_sizes: vec![Size::Weight(1.0), Size::Cells(13)],
        rows:         vec![
            vec![header("Movie"), header("IMDb Rating")],
            vec![movie("The Shawshank Redemption (1994)"), rating("9.2")],
            vec![movie("The Godfather (1972)"), rating("9.1")],
            vec![movie("The Godfather: Part II (1974)"), rating("9.0")],
            vec![movie("The Dark Knight (2008)"), rating("9.0")],
            vec![movie("12 Angry Men (2008)"), rating("8.9")],
        ],
    };

    console.render(&component)?;

    Ok(())
}

fn header(text: &str) -> Box<Text> {
    Box::new(Text {
        align:  TextAlignment::Center,
        text:   String::from(text),
        styles: vec![Style::Bold, Style::Foreground("yellow".to_string())],
    })
}

fn movie(text: &str) -> Box<Text> {
    Box::new(Text {
        align:  TextAlignment::Center,
        text:   String::from(text),
        styles: vec![Style::Foreground("cyan".to_string())],
    })
}

fn rating(text: &str) -> Box<Text> {
    Box::new(Text {
        align:  TextAlignment::Center,
        text:   String::from(text),
        styles: vec![Style::Foreground("bright_green".to_string())],
    })
}
