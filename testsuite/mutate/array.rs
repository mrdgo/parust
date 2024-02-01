pub fn entry() {
    let mut arr: [i32; 3] = [1, 2, 3];

    callee(&mut arr);

    assert_eq!(arr, [1, 4, 9]);
}

fn callee(arr: &mut [i32; 3]) -> () {
    arr[0] *= arr[0];
    arr[1] *= arr[1];
    arr[2] *= arr[2];
}
