const NUM: i32 = 17;

pub fn entry() -> () {
    let price: i32 = 5;
    let total = callee(price);
    assert_eq!(85, total);
}

fn callee(price: i32) -> i32 {
    price * NUM
}
