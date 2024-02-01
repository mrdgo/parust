pub fn entry() {
    let s: String = String::from("hello");
    callee(s);
}

fn callee(s: String) -> () {
    assert_eq!("hello", s)
}
