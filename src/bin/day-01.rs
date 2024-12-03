use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // file struct keeps internal state, hence mut
    let path = Path::new("./inputs/day-01.txt");

    let f = File::open(&path)?; // ? for error handling -- return early if error

    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut total: i32 = 0;

    let mut v1 = vec![];
    let mut v2 = vec![];

    for l in lines {
        let string_l = l?;

        let mut iter = string_l.split_whitespace();

        let id_1 = iter.next().unwrap();
        let id_2 = iter.next().unwrap();

        let id_int_1: i32 = id_1.parse().unwrap();
        let id_int_2: i32 = id_2.parse().unwrap();

        v1.push(id_int_1);
        v2.push(id_int_2);
        //println!("{}", id_sum);
    }

    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        let id_1: i32 = v1[i];
        let id_2: i32 = v2[i];

        let diff: i32;

        if id_1 > id_2 {
            diff = id_1 - id_2;
        } else {
            diff = id_2 - id_1;
        }

        total += diff;
    }

    println!("{}", total);

    Ok(())
}
