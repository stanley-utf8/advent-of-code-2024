use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("./inputs/day-03.txt");

    let res_01: i32 = part_01(path)?;
    println!("total; {res_01}");

    let res_02 = part_02(path)?;

    println!("part_02 total: {res_02}");

    Ok(())
}

fn part_01(path: &Path) -> std::io::Result<i32> {
    let f = File::open(&path)?;

    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut total: i32 = 0;

    for l in lines {
        let l = l?;

        let l_bytes = l.as_bytes();

        for (i, window) in l_bytes.windows(3).enumerate() {
            if window == b"mul" {
                // now check if next is '('
                // index out of bounds check
                if i + 3 < l_bytes.len() {
                    let next = l_bytes[i + 3];
                    if next == b'(' {
                        // found mul(
                        // collect until , for first number

                        let start_idx = i + 4;
                        let mut num_str: String = String::new();
                        let mut x: Option<i32> = None;
                        let mut y: Option<i32> = None;

                        // Start collecting until ')'

                        let mut j = start_idx;
                        while j < l_bytes.len() {
                            let c = l_bytes[j] as char;
                            if c == ')' {
                                if !num_str.is_empty() {
                                    if x.is_none() {
                                        break;
                                    } else {
                                        y = num_str.parse().ok();
                                    }
                                }
                                break;
                            } else if c == ',' {
                                if x.is_none() {
                                    // process x
                                    x = num_str.parse().ok();
                                    num_str.clear();
                                }
                            } else if c.is_ascii_digit() {
                                num_str.push(c);
                            } else if c.is_whitespace() {
                                break;
                            } else {
                                break;
                            }

                            j += 1
                        }

                        if let (Some(x_val), Some(y_val)) = (x, y) {
                            println!("found {x_val}, {y_val}");
                            total += x_val * y_val;
                        }
                    }
                }
            }
        }
    }

    Ok(total)
}

fn part_02(path: &Path) -> std::io::Result<i32> {
    let f = File::open(&path)?;

    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut total: i32 = 0;

    let mut search: bool = true;

    for l in lines {
        let l = l?;

        let l_bytes = l.as_bytes();

        // if window size don't() only look for do, when do(), look for correct mul and don't()
        // process line sequentially?
        // needs some way to continue from where I stopped

        // should I iterate through a window that can encompass all possibilities?

        for (i, window) in l_bytes.windows(7).enumerate() {
            if window == b"don't()" {
                println!("Found don't() at position {}", i); // Debug print
                search = false;
                continue;
            }

            if window.starts_with(b"do()") {
                println!("Found do() at position {}", i); // Debug print
                search = true;
                continue;
            }

            if search == true && window.starts_with(b"mul") {
                // now check if next is '('
                // index out of bounds check
                if i + 3 < l_bytes.len() {
                    let next = l_bytes[i + 3];
                    if next == b'(' {
                        // found mul(
                        // collect until , for first number

                        let start_idx = i + 4;
                        let mut num_str: String = String::new();
                        let mut x: Option<i32> = None;
                        let mut y: Option<i32> = None;

                        // Start collecting until ')'

                        let mut j = start_idx;
                        while j < l_bytes.len() {
                            let c = l_bytes[j] as char;
                            if c == ')' {
                                if !num_str.is_empty() {
                                    if x.is_none() {
                                        break;
                                    } else {
                                        y = num_str.parse().ok();
                                    }
                                }
                                break;
                            } else if c == ',' {
                                if x.is_none() {
                                    // process x
                                    x = num_str.parse().ok();
                                    num_str.clear();
                                }
                            } else if c.is_ascii_digit() {
                                num_str.push(c);
                            } else if c.is_whitespace() {
                                break;
                            } else {
                                break;
                            }

                            j += 1
                        }

                        if let (Some(x_val), Some(y_val)) = (x, y) {
                            println!("found {x_val}, {y_val}");
                            total += x_val * y_val;
                        }
                    }
                }
            }
        }
    }

    Ok(total)
}
