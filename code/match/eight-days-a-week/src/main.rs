fn name_of_day(n: i64) -> &'static str {

    match n {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => ""
    }
}

fn main() {

    for n in 0..8 {
        println!("{} => \"{}\"", n, name_of_day(n));
    }
}
