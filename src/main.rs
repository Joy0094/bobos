#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(b"Hi");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
fn print(text: &[u8]) {
    let vb = 0xb8000 as *mut u8;
    for (i, &b) in text.iter().enumerate() {
        unsafe {
            *vb.offset(i as isize * 2) = b;
            *vb.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
