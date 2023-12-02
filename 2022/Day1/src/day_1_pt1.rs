use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};



fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


pub fn pt1() -> Result<(), Error> {
    let vec = read(File::open("./input")?)?;
    let mut max: i64 = 0;
    let mut sum: i64 = 0;
    for _item in vec{
        if _item == 0{
            if sum > max{
                max = sum;
                sum = 0;
            }
            else{
                sum=0;
            }
        }
        else{
            sum = sum + _item;

        }

    }
    print!("{:?}",max);
    Ok(())
}
