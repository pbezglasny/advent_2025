use memmap2::Mmap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

#[allow(dead_code)]
fn max_jolt_on_line(line: &str) -> i32 {
    let mut result = 0;
    let mut prev_max = 0;
    for c in line.chars() {
        let n = c.to_digit(10).unwrap() as i32;
        result = result.max(prev_max * 10 + n);
        prev_max = prev_max.max(n);
    }
    result
}

fn find_max_starting_from(line: &str, start: usize, end: usize) -> (usize, u32) {
    let mut result = start;
    let mut val = line.chars().nth(start).unwrap().to_digit(10).unwrap();
    for i in start..end {
        let n = line.chars().nth(i).unwrap().to_digit(10).unwrap();
        if n > val {
            val = n;
            result = i;
        }
    }
    (result, val)
}

fn max_twelve(line: &str) -> i64 {
    let mut result = 0_i64;
    let mut start = 0;
    for i in (0..12).rev() {
        let (pos, val) = find_max_starting_from(line, start, line.len() - i);
        result = result * 10 + val as i64;
        start = pos + 1;
    }
    result
}
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/data/input_3.txt")?;
    // let file = File::open("src/data/test_3.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let mut line_num = 0;
    let result = mmap
        .lines()
        .map(|l| {
            println!("processing line {}", line_num);
            let result = max_twelve(&l.unwrap());
            println!("solved line {}: {}", line_num, result);
            line_num += 1;
            result
        })
        .sum::<i64>();
    println!("{}", result);
    Ok(())
}
