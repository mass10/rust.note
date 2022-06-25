mod decorator {

    pub fn make_black(s: &str) -> String {
        return format!("\x1b[30m{}\x1b[0m", s);
    }

    pub fn make_red(s: &str) -> String {
        return format!("\x1b[31m{}\x1b[0m", s);
    }

    pub fn make_green(s: &str) -> String {
        return format!("\x1b[32m{}\x1b[0m", s);
    }

    pub fn make_yellow(s: &str) -> String {
        return format!("\x1b[33m{}\x1b[0m", s);
    }

    pub fn make_blue(s: &str) -> String {
        return format!("\x1b[34m{}\x1b[0m", s);
    }

    pub fn make_magenta(s: &str) -> String {
        return format!("\x1b[35m{}\x1b[0m", s);
    }

    pub fn make_cyan(s: &str) -> String {
        return format!("\x1b[36m{}\x1b[0m", s);
    }

    pub fn make_white(s: &str) -> String {
        return format!("\x1b[37m{}\x1b[0m", s);
    }

    pub fn make_bold(s: &str) -> String {
        return format!("\x1b[1m{}\x1b[0m", s);
    }
}


fn main() {

    println!("{}", decorator::make_black("COLORING EXAMPLE"));
    println!("{}", decorator::make_red("COLORING EXAMPLE"));
    println!("{}", decorator::make_green("COLORING EXAMPLE"));
    println!("{}", decorator::make_yellow("COLORING EXAMPLE"));
    println!("{}", decorator::make_blue("COLORING EXAMPLE"));
    println!("{}", decorator::make_magenta("COLORING EXAMPLE"));
    println!("{}", decorator::make_cyan("COLORING EXAMPLE"));
    println!("{}", decorator::make_white("COLORING EXAMPLE"));

    println!("{}", decorator::make_bold(&decorator::make_black("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_red("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_green("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_yellow("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_blue("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_magenta("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_cyan("COLORING EXAMPLE")));
    println!("{}", decorator::make_bold(&decorator::make_white("COLORING EXAMPLE")));
}
