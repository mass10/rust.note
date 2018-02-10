
struct A {

    name: String
}

fn dump(a: &A) {

    println!("{}", a.name);

    // CANNOT MODIFY FIELD VALUE OF BORROWED OBJECT.
    // a.name = "".to_string();
}

fn main() {

    let a = A{name: "Hahaha".to_string()};

    dump(&a);

    println!("Ok.");
}
