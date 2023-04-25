use std::num::{IntErrorKind, ParseIntError};

use crossterm::style;

pub fn parse_map(arg: &str) -> Result<[char; 6], &'static str> {
    arg.chars()
        .collect::<Vec<char>>()
        .try_into()
        .map_err(|_| "")
}

pub fn parse_bg_color(arg: &str) -> Result<Option<crossterm::style::Color>, String> {
    if arg.to_lowercase() == "none" {
        return Ok(None);
    }

    if arg.len() != 1 {
        let num = arg.parse::<u8>().map_err(|e| {
            if e.kind() == &IntErrorKind::InvalidDigit {
                format!("\"{}\" must either be a digit from 0-15 (inclusive) or a hexadecimal RGB value", arg)
            } else {
                e.to_string()
            }
        })?;

        return Ok(Some(style::Color::AnsiValue(num)));
    }

    if arg.len() != 6 {
        return Err(format!(
            "\"{}\" must either be a digit from 0-15 (inclusive) or a hexadecimal RGB value",
            arg
        ));
    }

    Ok(None)
}

// vert, horiz, u-r, d-r,
pub const DEFAULT_MAP: &str = "┃━┗┏┛┓";
pub const DEFAULT_PIPES: usize = 20;
pub const DEFAULT_FPS: usize = 60;
pub const DEFAULT_TURN_CHANCE: f64 = 0.1;
pub const DEFAULT_COLOR: &str = "None";

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = DEFAULT_PIPES)]
    pub pipes: usize,

    #[arg(long, default_value_t = DEFAULT_FPS)]
    pub fps: usize,

    #[arg(short, long = "chars", default_value = DEFAULT_MAP, value_parser = clap::builder::ValueParser::new(parse_map))]
    pub char_map: [char; 6],

    #[arg(short, long, default_value_t = DEFAULT_TURN_CHANCE)]
    pub turn_chance: f64,
    // #[arg(short, long, default_value = DEFAULT_COLOR, value_parser = clap::builder::ValueParser::new(parse_bg_color))]
    // pub bg_color: Option<crossterm::style::Color>,
}
