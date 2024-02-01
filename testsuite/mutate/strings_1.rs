pub fn entry() -> () {
    let mut s: String = String::from("hello");
    callee(&mut s);
    assert_eq!(s, "hello, world");
}

fn callee(s: &mut String) -> () {
    s.push_str(", world");
}
