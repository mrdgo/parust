pub fn entry() -> () {
    let s: String = String::from("hello, world");

    let hello: &str = callee(&s);

    assert_eq!(hello, "hello");
}

// WARN: this might be our first actual limitation
fn callee(s: &String) -> &str {
    &s[..5]
}
