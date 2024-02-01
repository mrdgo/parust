static mut MAD: i32 = 0;

pub fn entry() {
    // The only way to access a mutable global variable is unsafe code.
    unsafe {
        MAD = 1;
    }

    callee();

    unsafe {
        assert_eq!(MAD, 0xf00d);
    }
}

// we might even have to pass back (1, 0xf00d), in case MAD changed
// while callee() ran.
fn callee() {
    unsafe {
        MAD = 0xf00d;
    }
}
