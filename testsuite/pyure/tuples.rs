pub fn entry() {
    let some_numbers = (1, 13, 19);

    callee(some_numbers);
}

fn callee(tup: (i32, i32, i32)) -> i32 {
    tup.0 + tup.1 + tup.2
}
