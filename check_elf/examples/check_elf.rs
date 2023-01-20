use std::env;
use check_elf::is_valid_elf_file;

fn main() {
    let file = env::args().nth(1).unwrap();
    if is_valid_elf_file(&file) {
        println!("{} is a valid ELF file", file.as_str());
    }
    else {
        println!("{} is not a valid ELF file", file.as_str());
    }
}