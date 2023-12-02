use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};



fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


pub fn pt2() -> Result<(), Error> {
    let vec = read(File::open("./input")?)?;
    let mut sum: i64 = 0;
    let mut list_of_cal : Vec<i64> = Vec::new();
    for _item in vec{
        if _item == 0{
         list_of_cal.push(sum);
         sum = 0;
        }
        else{
            sum = sum + _item;

        }

    }
    list_of_cal.sort();
    
    let sum_of_three_top: i64 = list_of_cal.pop().expect("number?") + list_of_cal.pop().expect("number") + list_of_cal.pop().expect("number");

    print!("{:?}", sum_of_three_top);
    Ok(())
}
