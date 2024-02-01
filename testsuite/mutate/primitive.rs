pub fn entry() {
    let mut n: i32 = 17;

    callee(&mut n);

    assert_eq!(n, 13);
}

fn callee(n: &mut i32) -> () {
    *n = 13;
}
