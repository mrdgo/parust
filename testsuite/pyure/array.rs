pub fn entry() {
    let arr: [i32; 3] = [1, 2, 3];

    let sqarr: [i32; 3] = callee(arr);

    assert_eq!(sqarr, [1, 4, 9]);
}

fn callee(arr: [i32; 3]) -> [i32; 3] {
    [arr[0] * arr[0], arr[1] * arr[1], arr[2] * arr[2]]
}
