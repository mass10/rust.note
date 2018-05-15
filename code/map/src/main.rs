fn main() {

    let v = vec![111, 222, 333, 444, 555];
    for e in v.into_iter().map(|x| x * 2) {
    	println!("{:?}", e);
    }
}
