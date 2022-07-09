#[warn(dead_code)]
use std::io::{stdin, self};

pub fn read_console_line()->String {
    let mut buffer = String::new();
    let _res = stdin().read_line(&mut buffer);
    buffer= String::from(buffer.trim());
    return buffer
}


pub fn read_int()->i32{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    match s.trim_end().parse::<i32>() {
        Ok(i) => return i,
        Err(_) => return -1,
    }

}