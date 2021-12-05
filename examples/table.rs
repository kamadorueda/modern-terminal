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
        column_sizes: vec![Size::Cells(25), Size::Cells(7)],
        rows:         vec![
            vec![
                Box::new(Text {
                    align:  TextAlignment::Left,
                    text:   String::from("Movie"),
                    styles: vec![
                        Style::Bold,
                        Style::Foreground("yellow".to_string()),
                    ],
                }),
                Box::new(Text {
                    align:  TextAlignment::Center,
                    text:   String::from("Score"),
                    styles: vec![
                        Style::Bold,
                        Style::Foreground("yellow".to_string()),
                    ],
                }),
            ],
            vec![
                Box::new(Text {
                    align:  TextAlignment::Left,
                    text:   String::from("The Shawshank Redemption"),
                    styles: vec![],
                }),
                Box::new(Text {
                    align:  TextAlignment::Center,
                    text:   String::from("9.2"),
                    styles: vec![Style::Foreground("bright_green".to_string())],
                }),
            ],
            vec![
                Box::new(Text {
                    align:  TextAlignment::Left,
                    text:   String::from("The Godfather"),
                    styles: vec![],
                }),
                Box::new(Text {
                    align:  TextAlignment::Center,
                    text:   String::from("9.1"),
                    styles: vec![Style::Foreground("bright_green".to_string())],
                }),
            ],
            vec![
                Box::new(Text {
                    align:  TextAlignment::Left,
                    text:   String::from("The Godfather: Part II"),
                    styles: vec![],
                }),
                Box::new(Text {
                    align:  TextAlignment::Center,
                    text:   String::from("9.0"),
                    styles: vec![Style::Foreground("bright_green".to_string())],
                }),
            ],
        ],
    };

    console.render(&component)?;

    Ok(())
}
