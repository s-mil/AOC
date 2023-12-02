use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use enum_map::{enum_map, Enum, EnumMap};

enum Rps {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

let mut map = enum_map! {
    Rps::A => 1,
    Rps::B => 1,
    Rps::C => 1,
    Rps::X => 1,
    Rps::Y => 1,
    Rps::Z => 1,
    
    
}

struct Play {
    Mine: 

}


fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}



fn main() {
    println!("Hello, world!");
}
