#![no_main]

use libfuzzer_sys::fuzz_target;
use check_elf::is_valid_elf;

fuzz_target!(|data: &[u8]| {
    is_valid_elf(data);
});
