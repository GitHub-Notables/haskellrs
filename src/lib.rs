use std::ffi::CString;

#[no_mangle]
pub extern fn double_input(x: i32) -> i32 {
    2 * x
}

#[no_mangle]
pub extern fn print_string(x: CString) {
    if let Ok(input) = x.into_string() {
        println!("We're printing strings from Rust!");
        println!("{}",input);
    } else {
        panic!("Unable to print input");
    }
}
