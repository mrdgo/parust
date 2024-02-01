pub fn entry() {
    assert_eq!(callee(), "hello");
}

fn callee() -> String {
    String::from("hello")
}
