
fn unsafe_operation() {

    panic!("予期しないエラーです。");
}

fn main() {

    unsafe_operation();

    println!("Ok.");
}
