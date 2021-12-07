extern crate modern_terminal;

use modern_terminal::{
    components::text::{Text, TextAlignment},
    core::{console::Console, segment::SegmentPortion, style::Style},
};

fn main() -> std::io::Result<()> {
    let mut writer = std::io::stdout();
    let mut console = Console::from_fd(&mut writer);

    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Style(Style::Foreground("yellow".to_string())),
            SegmentPortion::Style(Style::Underline),
            SegmentPortion::Text("TextAlignment::Left".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::Background("cyan".to_string())),
            SegmentPortion::Style(Style::Foreground("black".to_string())),
            SegmentPortion::Text("Background cyan".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Text("Bold".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("None".to_string()),
        ],
    })?;

    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Style(Style::Foreground("yellow".to_string())),
            SegmentPortion::Style(Style::Underline),
            SegmentPortion::Text("TextAlignment::Center".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Style(Style::Foreground("cyan".to_string())),
            SegmentPortion::Text("Foreground cyan".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Foreground("#00FFFF".to_string())),
            SegmentPortion::Text("Foreground #00FFFFF".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Foreground(
                "rgb(0,255,255)".to_string(),
            )),
            SegmentPortion::Text("Foreground rgb(0,255,255)".to_string()),
        ],
    })?;

    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Style(Style::Foreground("yellow".to_string())),
            SegmentPortion::Style(Style::Underline),
            SegmentPortion::Text("TextAlignment::Right".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Reverse),
            SegmentPortion::Text("Reverse".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Underline),
            SegmentPortion::Text("Underline".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
        ],
    })?;

    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Style(Style::Foreground("yellow".to_string())),
            SegmentPortion::Style(Style::Underline),
            SegmentPortion::Text("Supported in some terminals".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Blink),
            SegmentPortion::Text("Blink".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::BlinkFast),
            SegmentPortion::Text("BlinkFast".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::Conceal),
            SegmentPortion::Text("Conceal".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::Dim),
            SegmentPortion::Text("Dim".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::Encircle),
            SegmentPortion::Text("Encircle".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Frame),
            SegmentPortion::Text("Frame".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::Italic),
            SegmentPortion::Text("Italic".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::Overline),
            SegmentPortion::Text("Overline".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::Strike),
            SegmentPortion::Text("Strike".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text(" ".to_string()),
            SegmentPortion::Style(Style::UnderlineMore),
            SegmentPortion::Text("UnderlineMore".to_string()),
        ],
    })?;

    Ok(())
}
