use termcol::*;

fn main() {
    println!("===== COLORS =====\n");

    println!("{}black", color("black"));
    println!("{}red", color("red"));
    println!("{}green", color("green"));
    println!("{}gold", color("gold"));
    println!("{}blue", color("blue"));
    println!("{}pink", color("pink"));
    println!("{}cyan", color("cyan"));
    println!("{}gray", color("gray"));
    println!("{}light red", color("light red"));
    println!("{}light green", color("light green"));
    println!("{}yellow", color("yellow"));
    println!("{}purple", color("purple"));
    println!("{}light pink", color("light pink"));
    println!("{}light blue", color("light blue"));
    println!("{}white", color("white"));
    println!("{}reset\n", color("reset"));

    let mut colorful_string: String = "".to_string();
    colorful_string += &(color("black") + "! " + &color("red") + "c");
    colorful_string += &(color("green") + "o" + &color("gold") + "l");
    colorful_string += &(color("blue") + "o" + &color("pink") + "r");
    colorful_string += &(color("cyan") + "f" + &color("gray") + "u");
    colorful_string += &(color("light red") + "l" + &color("light green") + " s");
    colorful_string += &(color("yellow") + "t" + &color("purple") + "r");
    colorful_string += &(color("light pink") + "i" + &color("light blue") + "n");
    colorful_string += &(color("white") + "g" + &color("reset") + " !");
    println!("{}\n", colorful_string);

    let mut string_to_color: String = "Hello, World!".to_string();
    string_to_color = color_string(&string_to_color, "gold");
    println!("{}", string_to_color);

    let mut multi_color: String = "".to_string();
    multi_color += &(color_string("easy ", "red") + &color_string("multi ", "green"));
    multi_color += &(color_string("color ", "blue") + &color_string("text!", "white"));
    println!("{}\n", multi_color);

    println!("===== TRUE COLOR =====\n");

    println!(
        "{}{}True Color{} {}{}Support!{}",
        true_color(255, 240, 32, true),
        true_color(20, 20, 200, false),
        color("reset"),
        true_color_hex("0x1414C8", true),
        true_color_hex("0xfff020", false),
        color("reset")
    );

    println!(
        "{} {}",
        true_color_string("True Color", 255, 240, 32, 20, 20, 200),
        true_color_string_hex("Support!", "0x1414C8", "0xfff020")
    );

    println!();

    println!("===== FORMATING =====\n");

    println!("{}bold{}", format("bold"), format("reset"));
    println!("{}faint{}", format("faint"), format("reset"));
    println!("{}italic{}", format("italic"), format("reset"));
    println!("{}underline{}", format("underline"), format("reset"));
    println!("{}blink{}", format("blink"), format("reset"));
    println!("{}invert{}", format("invert"), format("reset"));
    println!("{}strike{}\n", format("strike"), format("reset"));

    let mut multi_format: String = "".to_string();
    multi_format +=
        &(format_string("easy", "italic") + " " + &format_string("multi", "underline") + " ");
    multi_format += &(format_string("format", "invert") + " " + &format_string("text!", "strike"));
    println!("{}\n", multi_format);

    println!("===== OPTIMIZING DISPLAYING TEXT =====\n");

    let mut string_to_optimize: String = "".to_string();
    string_to_optimize +=
        &(color_string("this text ", "reset") + &color_string("contains a lot", "reset"));
    string_to_optimize +=
        &(color_string(" of invisible ", "reset") + &color_string("characters.", "reset"));
    println!(
        "before: {}     length: {}",
        string_to_optimize,
        string_to_optimize.chars().count()
    );
    string_to_optimize = optimize(&string_to_optimize);
    println!(
        " after: {}     length: {}",
        string_to_optimize,
        string_to_optimize.chars().count()
    );

    // This is how to disable coloring/formating all together:
    clean_output(true);
}
