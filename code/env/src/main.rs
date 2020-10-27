fn main() {
    let path = std::env::var("USERPROFILE").unwrap_or(String::new());
    println!("PATH: [{:?}]", path);

    std::env::set_var("USERPROFILE", "X");

    let path = std::env::var("USERPROFILE").unwrap_or(String::new());
    println!("PATH: [{:?}]", path);
}
