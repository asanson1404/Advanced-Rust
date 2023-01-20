use elf_rs::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn is_valid_elf(content: &[u8]) -> bool {
    let elf = Elf::from_bytes(content);
    match elf {
        Ok(_) => true,
        Err(_)=> false,
    }
}

pub fn is_valid_elf_file<P: AsRef<Path>>(filename: P) -> bool {

    let elf_file = File::open(filename);
    if elf_file.is_err() { return false;}

    let mut elf_buf: Vec<u8> = Vec::new();
    let try_read = elf_file.unwrap().read_to_end(&mut elf_buf);
    if try_read.is_err() { return false;}
    is_valid_elf(&elf_buf)
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
