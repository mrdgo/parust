pub fn entry() {
    assert_eq!(6765, callee(19))
}

fn callee(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        callee(n - 2) + callee(n - 1)
    }
}
