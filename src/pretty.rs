// Defining terminal colors
const COLOR_BLACK:  &str = "\u{001b}[30m"; // Terminals are black why output black ???
const COLOR_RED:    &str = "\u{001b}[31m";
const COLOR_GREEN:  &str = "\u{001b}[32m";
const COLOR_YELLOW: &str = "\u{001b}[33m";
const COLOR_BLUE:   &str = "\u{001b}[34m";
const COLOR_BOLD:   &str = "\x1B[1m";
const COLOR_RESET:  &str = "\u{001b}[0m";

pub fn output(color: &str, text: &str) {

    match color {
        "RED" => {
            let color: &str = COLOR_RED;
            print_text(color, &text);
        }
        "GREEN" => {
            let color: &str = COLOR_GREEN;
            print_text(color, &text);
        }
        "YELLOW" => {
            let color: &str = COLOR_YELLOW;
            print_text(color, &text);
        }
        "BLUE" => {
            let color: &str = COLOR_BLUE;
            print_text(color, &text);        
        }
        _ => {
            let color: &str = COLOR_BLACK;
            print_text(color, &text);
        }
    }

    fn print_text(color: &str, text: &str) {
        print!("{}{}{}{}", COLOR_BOLD, color, text, COLOR_RESET);

    }
}

pub fn pass(text: &str) {
    println!("{}{}{}! {}", COLOR_BOLD, COLOR_GREEN, text, COLOR_RESET);
    std::process::exit(0);
}

pub fn notice(text: &str) {
    println!("{}{}Notice: {}! {}", COLOR_BOLD, COLOR_BLUE, text, COLOR_RESET);
}

pub fn warn(text: &str) {
    println!("{}{}Warning: {}! {}", COLOR_BOLD, COLOR_YELLOW, text, COLOR_RESET);
}

pub fn halt(text: &str) {
    println!("{}{}Panic!: {}! {}", COLOR_BOLD, COLOR_RED, text, COLOR_RESET);
    std::process::exit(1);
}

// * for debugging only
pub fn _dump(text: &str) {
    println!("{}{}DUMPED: {}! {}", COLOR_BOLD, COLOR_YELLOW, text, COLOR_RESET);
    std::process::exit(13);
}