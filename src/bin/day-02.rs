use std::fs::File;
use std::i32;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let path = Path::new("./inputs/day-02.txt");

    let res_01: i32 = part_01(path)?;

    println!("levels safe: {}", res_01);

    let start = Instant::now();
    let correct_results = part_02(path);
    let duration = start.elapsed();

    println!("Original solution time: {:?}", duration);

    let start = Instant::now();
    let opt_results = part_02_opt(path);
    let duration = start.elapsed();

    println!("Optimized solution time: {:?}", duration);

    println!("Sequences found in correct but not in optimized:");

    for sequence in correct_results.iter() {
        if !opt_results.contains(sequence) {
            println!("{}", sequence);
        }
    }

    println!("\nCorrect count: {}", correct_results.len());
    println!("Optimized count: {}", opt_results.len());

    //Original solution time: 7.128167ms
    //Optimized solution time: 5.388083ms

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

fn check_pair(n_0: u32, n_1: u32, direction: bool) -> bool {
    (n_0 <= n_1) == direction && n_0.abs_diff(n_1) <= 3 && n_0 != n_1
}

fn check_line(nums: &[u32], direction: bool) -> bool {
    // for 'all' windows [n0, n1] in nums of size '2', check if they satisfy check_pair
    nums.windows(2).all(|n| check_pair(n[0], n[1], direction))
}

fn part_02(path: &Path) -> Vec<String> {
    let f = File::open(&path).expect("failed to open file");

    let reader = BufReader::new(f);
    reader
        .lines()
        .filter_map(|line| {
            //predicate + &pattern matches on the reference to line
            if let Ok(line) = line {
                let nums = line
                    .split_whitespace()
                    .map(|str_num| str_num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let direction = nums[0] <= nums[1];

                if check_line(&nums, direction) {
                    return Some(line);
                } else {
                    // edge case: remove index 0
                    let direction_0 = nums[1] <= nums[2];

                    if check_line(&nums[1..], direction_0) {
                        return Some(line);
                    }

                    // edge case: remove index 1
                    let direction_1 = nums[0] <= nums[2];
                    if nums[0] != nums[2]
                        && nums[0].abs_diff(nums[2]) <= 3
                        && check_line(&nums[2..], direction_1)
                    {
                        return Some(line);
                    }

                    // edge case: remove last index
                    if check_line(&nums[..nums.len() - 1], direction) {
                        return Some(line);
                    }

                    // rest of cases:
                    for removed in 2..nums.len() - 1 {
                        if check_line(&nums[..removed], direction)
                            && check_pair(nums[removed - 1], nums[removed + 1], direction)
                            && check_line(&nums[removed + 1..], direction)
                        {
                            return Some(line);
                        }
                    }
                    return None;
                }
            } else {
                return None;
            }
        })
        .collect()
}

fn part_02_opt(path: &Path) -> Vec<String> {
    let f = File::open(&path).expect("failed to open file");

    let reader = BufReader::new(f);
    reader
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                let nums = line
                    .split_whitespace()
                    .map(|str_num| str_num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let direction = nums[0] <= nums[1];

                if check_line(&nums, direction) {
                    return Some(line);
                }
                if let Some((problem_idx, _)) = nums
                    .windows(2)
                    .enumerate()
                    .find(|(_idx, pair)| !check_pair(pair[0], pair[1], direction))
                {
                    if problem_idx == 1 {
                        let direction_without_first = nums[1] <= nums[2];
                        if check_line(&nums[1..], direction_without_first) {
                            return Some(line);
                        }
                    }
                    let mut nums_without_first = nums.clone();
                    nums_without_first.remove(problem_idx);
                    let direction_first = nums_without_first[0] <= nums_without_first[1];
                    if check_line(&nums_without_first, direction_first) {
                        return Some(line);
                    }

                    let mut nums_without_second = nums.clone();
                    nums_without_second.remove(problem_idx + 1);
                    let direction_second = nums_without_second[0] <= nums_without_second[1];
                    if check_line(&nums_without_second, direction_second) {
                        return Some(line);
                    }
                }
                None
            } else {
                None
            }
        })
        .collect()
}
