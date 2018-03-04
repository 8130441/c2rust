extern crate libc;

use sieve_of_eratosthenes::sieve_of_eratosthenes as rust_sieve_of_eratosthenes;
use self::libc::c_int;

extern "C" {
    #[no_mangle]
    fn sieve_of_eratosthenes(_: *mut c_int);
}

const BUFFER_SIZE: usize = 102; // [0, 101]

pub fn test_buffer() {
    let mut buffer = [0; BUFFER_SIZE];
    let mut rust_buffer = [0; BUFFER_SIZE];
    let expected_buffer = [
        0, 0, 1, 1, 0, 1, 0, 1, 0, 0,
        0, 1, 0, 1, 0, 0, 0, 1, 0, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 1, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 1, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1,
    ];

    unsafe {
        sieve_of_eratosthenes(buffer.as_mut_ptr());
        rust_sieve_of_eratosthenes(rust_buffer.as_mut_ptr());
    }

    for index in 0..BUFFER_SIZE {
        assert_eq!(buffer[index], rust_buffer[index]);
        assert_eq!(buffer[index], expected_buffer[index], "idx: {}", index);
    }
}
