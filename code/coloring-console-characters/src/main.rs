mod decorator {

    pub fn black(s: &str) -> String {
        return format!("\x1b[30m{}\x1b[0m", s);
    }

    pub fn red(s: &str) -> String {
        return format!("\x1b[31m{}\x1b[0m", s);
    }

    pub fn green(s: &str) -> String {
        return format!("\x1b[32m{}\x1b[0m", s);
    }

    pub fn yellow(s: &str) -> String {
        return format!("\x1b[33m{}\x1b[0m", s);
    }

    pub fn blue(s: &str) -> String {
        return format!("\x1b[34m{}\x1b[0m", s);
    }

    pub fn magenta(s: &str) -> String {
        return format!("\x1b[35m{}\x1b[0m", s);
    }

    pub fn cyan(s: &str) -> String {
        return format!("\x1b[36m{}\x1b[0m", s);
    }

    pub fn white(s: &str) -> String {
        return format!("\x1b[37m{}\x1b[0m", s);
    }

    pub fn bold(s: &str) -> String {
        return format!("\x1b[1m{}\x1b[0m", s);
    }
}


fn main() {

    println!("{}", decorator::black("COLORING EXAMPLE"));
    println!("{}", decorator::red("COLORING EXAMPLE"));
    println!("{}", decorator::green("COLORING EXAMPLE"));
    println!("{}", decorator::yellow("COLORING EXAMPLE"));
    println!("{}", decorator::blue("COLORING EXAMPLE"));
    println!("{}", decorator::magenta("COLORING EXAMPLE"));
    println!("{}", decorator::cyan("COLORING EXAMPLE"));
    println!("{}", decorator::white("COLORING EXAMPLE"));

    println!("{}", decorator::bold(&decorator::black("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::red("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::green("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::yellow("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::blue("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::magenta("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::cyan("COLORING EXAMPLE")));
    println!("{}", decorator::bold(&decorator::white("COLORING EXAMPLE")));
}
