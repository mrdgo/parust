pub fn entry() {
    let mut fibs = (1, 1, 2, 3, 5);

    callee(&mut fibs);

    assert_eq!(fibs.0, 4);
}

fn callee(fibs: &mut (i32, i32, i32, i32, i32)) {
    fibs.0 += 3;
}
