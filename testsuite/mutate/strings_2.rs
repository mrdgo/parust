pub fn entry() -> () {
    let mut s: String = String::from("hell");
    callee(&mut s);

    s.push_str(", world");

    assert_eq!(s, "hello, world");
}

fn callee(s0: &mut String) -> () {
    s0.push_str("o");
}
