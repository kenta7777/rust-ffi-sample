extern crate libc;

extern {
    fn add(n1: libc::c_int, n2: libc::c_int) -> libc::c_int;
    fn add_pointer(int_ptr1: *mut libc::c_int, int_ptr2: *mut libc::c_int) -> libc::c_int;
    fn add_pointer_with_no_result(int_ptr1: *mut libc::c_int, int_ptr2: *mut libc::c_int);
}

pub fn add_example() {
    let mut n1 = 4;
    let mut n2 = 5;
    let add_result = unsafe { add(n1, n2) };
    println!("add_result: {}", add_result);

    let add_pointer_result = unsafe { add_pointer( &mut n1, &mut n2) };
    println!("n1: {}", n1);
    println!("n2: {}", n2);
    println!("add_pointer_result: {}", add_pointer_result);

    unsafe {
        add_pointer_with_no_result(&mut n1, &mut n2);
    };
    println!("n1: {}", n1);
    println!("n2: {}", n2);
}
