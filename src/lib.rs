/// Helper function returning the value of DISALBE_COLORING global variable
pub fn get_disable_coloring_state() -> bool {
    unsafe { DISABLE_COLORING }
}

/// Function providing a way to disable any form of coloring/formating
pub fn clean_output(state: bool) {
    unsafe {
        DISABLE_COLORING = state;
    }
}

/// Helper function providing easy way of finding every occurrence of string in a string
pub fn find_every_occurance(in_string: &str, find_me: &str) -> Vec<usize> {
    let mut occurances: Vec<usize> = vec![];

    for (i, _c) in in_string.chars().enumerate() {
        let sub_string: String = in_string[i..].to_string();
        if sub_string.starts_with(find_me) {
            occurances.push(i);
        }
    }

    occurances
}

static mut DISABLE_COLORING: bool = false;
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const GOLD: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PINK: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const GRAY: &str = "\x1b[90m";
const LIGHT_RED: &str = "\x1b[91m";
const LIGHT_GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const PURPLE: &str = "\x1b[94m";
const LIGHT_PINK: &str = "\x1b[95m";
const LIGHT_BLUE: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[00m";
const BOLD: &str = "\x1b[1m";
const FAINT: &str = "\x1b[2m";
const ITALIC: &str = "\x1b[3m";
const UNDERLINE: &str = "\x1b[4m";
const BLINK: &str = "\x1b[5m";
const INVERT: &str = "\x1b[7m";
const STRIKE: &str = "\x1b[9m";

/// Function providing a way to color your cli output
pub fn color(color: &str) -> String {
    if get_disable_coloring_state() {
        return String::new();
    }
    match color {
        "black" => BLACK.to_string(),
        "red" => RED.to_string(),
        "green" => GREEN.to_string(),
        "gold" => GOLD.to_string(),
        "blue" => BLUE.to_string(),
        "pink" => PINK.to_string(),
        "cyan" => CYAN.to_string(),
        "gray" => GRAY.to_string(),
        "light red" => LIGHT_RED.to_string(),
        "light green" => LIGHT_GREEN.to_string(),
        "yellow" => YELLOW.to_string(),
        "purple" => PURPLE.to_string(),
        "light pink" => LIGHT_PINK.to_string(),
        "light blue" => LIGHT_BLUE.to_string(),
        "white" => WHITE.to_string(),
        "reset" => RESET.to_string(),
        _ => String::new(),
    }
}

/// Function providing a way to color your cli output
pub fn color_string(str_to_color: &str, color: &str) -> String {
    if get_disable_coloring_state() {
        return str_to_color.to_string();
    }
    // Creating and returning the final string using the format macro
    format!(
        "{}{}{}",
        match color {
            "black" => BLACK,
            "red" => RED,
            "green" => GREEN,
            "gold" => GOLD,
            "blue" => BLUE,
            "pink" => PINK,
            "cyan" => CYAN,
            "gray" => GRAY,
            "light red" => LIGHT_RED,
            "light green" => LIGHT_GREEN,
            "yellow" => YELLOW,
            "purple" => PURPLE,
            "light pink" => LIGHT_PINK,
            "light blue" => LIGHT_BLUE,
            "white" => WHITE,
            "reset" => RESET,
            _ => "",
        },
        str_to_color,
        RESET
    )
}

/// Returns true color escape codes for given u8 values
/// background_or_foreground true => change background
/// background_or_foreground false => change foreground
pub fn true_color(red: u8, green: u8, blue: u8, background_or_foreground: bool) -> String {
    if get_disable_coloring_state() {
        return String::new();
    }
    if background_or_foreground {
        // Background
        format!("\x1b[48;2;{};{};{}m", red, green, blue)
    } else {
        // Foreground
        format!("\x1b[38;2;{};{};{}m", red, green, blue)
    }
}

/// Returns given string altered with true color escape codes from given u8 values
pub fn true_color_string(
    str_to_color: &str,
    fg_red: u8,
    fg_green: u8,
    fg_blue: u8,
    bg_red: u8,
    bg_green: u8,
    bg_blue: u8,
) -> String {
    if get_disable_coloring_state() {
        return String::new();
    }
    format!(
        "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m{}{}",
        bg_red, bg_green, bg_blue, fg_red, fg_green, fg_blue, str_to_color, RESET
    )
}

/// Returns true color escape codes from given hex string
/// background_or_foreground true => change background
/// background_or_foreground false => change foreground
pub fn true_color_hex(hex: &str, background_or_foreground: bool) -> String {
    if get_disable_coloring_state() || hex.len() != 8 {
        return String::new();
    }
    if &hex[0..2] != "0x" {
        return String::new();
    }
    let red: u8 = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let green: u8 = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let blue: u8 = u8::from_str_radix(&hex[6..8], 16).unwrap();
    if background_or_foreground {
        // Background
        format!("\x1b[48;2;{};{};{}m", red, green, blue)
    } else {
        // Foreground
        format!("\x1b[38;2;{};{};{}m", red, green, blue)
    }
}

/// Returns given string altered with true color escape codes from given hex strings
pub fn true_color_string_hex(str_to_color: &str, fg: &str, bg: &str) -> String {
    if get_disable_coloring_state() || fg.len() != 8 || bg.len() != 8 {
        return String::new();
    }
    if &fg[0..2] != "0x" || &bg[0..2] != "0x" {
        return String::new();
    }
    let fg_red: u8 = u8::from_str_radix(&fg[2..4], 16).unwrap();
    let fg_green: u8 = u8::from_str_radix(&fg[4..6], 16).unwrap();
    let fg_blue: u8 = u8::from_str_radix(&fg[6..8], 16).unwrap();
    let bg_red: u8 = u8::from_str_radix(&bg[2..4], 16).unwrap();
    let bg_green: u8 = u8::from_str_radix(&bg[4..6], 16).unwrap();
    let bg_blue: u8 = u8::from_str_radix(&bg[6..8], 16).unwrap();
    format!(
        "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m{}{}",
        bg_red, bg_green, bg_blue, fg_red, fg_green, fg_blue, str_to_color, RESET
    )
}

/// Function providing a way to format you cli output
pub fn format(style: &str) -> String {
    if get_disable_coloring_state() {
        return String::new();
    }
    match style {
        "bold" => BOLD.to_string(),
        "faint" => FAINT.to_string(),
        "italic" => ITALIC.to_string(),
        "underline" => UNDERLINE.to_string(),
        "blink" => BLINK.to_string(),
        "invert" => INVERT.to_string(),
        "strike" => STRIKE.to_string(),
        "reset" => RESET.to_string(),
        _ => String::new(),
    }
}

/// Function providing a way to format you cli output
pub fn format_string(str_to_format: &str, style: &str) -> String {
    if get_disable_coloring_state() {
        return str_to_format.to_string();
    }
    // Creating and returning the final string using the format macro
    format!(
        "{}{}{}",
        match style {
            "bold" => BOLD,
            "faint" => FAINT,
            "italic" => ITALIC,
            "underline" => UNDERLINE,
            "blink" => BLINK,
            "invert" => INVERT,
            "strike" => STRIKE,
            "reset" => RESET,
            _ => "",
        },
        str_to_format,
        RESET
    )
}

/// Function which optimizes given string from unneded reset control codes
pub fn optimize(str_to_optimize: &str) -> String {
    let mut out_string: String = str_to_optimize.to_string();

    if get_disable_coloring_state() {
        return str_to_optimize.to_string();
    }

    let mut occurances = find_every_occurance(str_to_optimize, RESET);
    occurances.reverse();

    for i in 0..occurances.len() {
        let current_occurance = occurances[i] + (3 * (occurances.len() - (i + 1)));

        if i == 0 && occurances[0] + 3 == str_to_optimize.chars().count() {
            out_string = (&out_string[0..current_occurance]).to_string();
        } else {
            let temp_out_string: String = out_string;
            out_string = temp_out_string[0..occurances[i]].to_string();
            out_string += &temp_out_string[occurances[i] + 5..];
        }
    }
    out_string += RESET;
    out_string
}
