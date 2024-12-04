use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // file struct keeps internal state, hence mut
    let path = Path::new("./inputs/day-01.txt");

    let res_01: i32 = part_01(path)?;

    let res_02: i32 = part_02(path)?;

    println!("part 01 result: {}", res_01);
    println!("part 02 result: {}", res_02);

    Ok(())
}

fn part_02(path: &Path) -> std::io::Result<i32> {
    let f = File::open(&path)?;

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
    }

    for i in 0..v1.len() {
        //let mut iter = v2.iter().filter(|&&x| x == v1[i]); // reference and then dereference to get x
        let occurences: i32 = v2
            .iter()
            .filter(|&x| *x == v1[i])
            .count()
            .try_into()
            .unwrap();
        let similarity_score: i32 = v1[i] * occurences;
        total += similarity_score;
    }

    Ok(total)
}

fn part_01(path: &Path) -> std::io::Result<i32> {
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
    }

    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        let id_1: i32 = v1[i];
        let id_2: i32 = v2[i];

        let diff: i32 = (id_1 - id_2).abs();

        total += diff;
    }

    Ok(total)
}
