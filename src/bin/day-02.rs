use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("./inputs/day-02.txt");

    let res_01: i32 = part_01(path)?;

    println!("levels safe: {}", res_01);

    Ok(())
}

fn part_01(path: &Path) -> std::io::Result<i32> {
    let f = File::open(&path)?;

    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut safe_reports: i32 = 0;

    for l in lines {
        let string_l = l?;

        let mut level_safe: bool = true;
        let mut iter = string_l.split_whitespace();

        let mut levels: Vec<Result<i32, _>> = vec![];

        while let Some(level) = iter.next() {
            levels.push(level.parse());
        }

        println!("{}", string_l);
        let decreasing: bool = levels[0].as_ref().unwrap() - levels[1].as_ref().unwrap() > 0;

        for i in 0..levels.len() - 1 {
            let cur: i32 = *levels[i].as_ref().unwrap();
            let next: i32 = *levels[i + 1].as_ref().unwrap();

            let cur_decreasing: bool = cur - next > 0;

            println!("{}", cur - next);

            if cur_decreasing != decreasing || cur - next == 0 || (cur - next).abs() > 3 {
                level_safe = false;
                break;
            }
        }
        if level_safe {
            safe_reports += 1
        }
    }

    Ok(safe_reports)
}
