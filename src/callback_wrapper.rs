extern fn callback(num: i32) {
    println!("called from C with num: {}", num);
}

extern "C" {
    fn register_callback(cb: extern fn(i32)) -> i32;
    fn trigger_callback();
}

pub fn callback_example() {
    unsafe {
        register_callback(callback);
        trigger_callback();
    }
}