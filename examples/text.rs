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
            SegmentPortion::Style(Style::Background("bright_blue".to_string())),
            SegmentPortion::Style(Style::Foreground("black".to_string())),
            SegmentPortion::Text("Background bright_blue".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::Blink),
            SegmentPortion::Text("Blink".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::BlinkFast),
            SegmentPortion::Text("BlinkFast".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
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
            SegmentPortion::Text("Conceal".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::Dim),
            SegmentPortion::Text("Dim".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Encircle),
            SegmentPortion::Text("Encircle".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Frame),
            SegmentPortion::Text("Frame".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Foreground("yellow".to_string())),
            SegmentPortion::Text("Foreground yellow".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Foreground("#FFFF00".to_string())),
            SegmentPortion::Text("Foreground #FFFF00".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Foreground(
                "rgb(255,255,0)".to_string(),
            )),
            SegmentPortion::Text("Foreground rgb(255,255,0)".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::Italic),
            SegmentPortion::Text("Italic".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Center,
        portions: vec![
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("None".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Overline),
            SegmentPortion::Text("Overline".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Reverse),
            SegmentPortion::Text("Reverse".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Strike),
            SegmentPortion::Text("Strike".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::Underline),
            SegmentPortion::Text("Underline".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Right,
        portions: vec![
            SegmentPortion::Style(Style::UnderlineMore),
            SegmentPortion::Text("UnderlineMore".to_string()),
            SegmentPortion::Style(Style::None),
            SegmentPortion::Text("¹".to_string()),
        ],
    })?;
    console.render(&Text {
        align:    TextAlignment::Left,
        portions: vec![
            SegmentPortion::Style(Style::Bold),
            SegmentPortion::Style(Style::Foreground(
                "bright_green".to_string(),
            )),
            SegmentPortion::Text(
                "¹ Only supported in some terminals".to_string(),
            ),
        ],
    })?;

    Ok(())
}
