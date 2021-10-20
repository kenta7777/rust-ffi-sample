extern crate libc;

extern {
    fn add(n1: libc::c_int, n2: libc::c_int) -> libc::c_int;
}

fn main() {
    let n1 = 4;
    let n2 = 5;
    let result = unsafe { add(n1, n2) };
    println!("{}", result);
}
