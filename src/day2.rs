use memmap2::Mmap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

fn is_match(n: i64) -> bool {
    let s = n.to_string();
    s[0..s.len() / 2] == s[s.len() / 2..s.len()]
}

fn is_match_2(n: i64) -> bool {
    let s = n.to_string();
    if s.len() < 2 {
        return false;
    }
    if is_match(n) {
        return true;
    }
    for i in 1..=s.len() / 2 {
        let mut j = 2;
        if s.len() % i != 0 {
            continue;
        }
        while j * i <= s.len() {
            if s[i * (j - 2)..i * (j - 1)] != s[i * (j - 1)..i * (j)] {
                break;
            }
            if j * i == s.len() {
                return true;
            }
            j += 1;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/home/pavel/work/rust/advent/src/data/input_2.txt")?;
    // let file = File::open("/home/pavel/work/rust/advent/src/data/test_2.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    // for line in mmap.lines() {
    //     println!("{}", line?);
    // }
    let line = mmap.lines().next().unwrap()?;
    let ranges = line.split(',');
    let mut result: i64 = 0;
    for range in ranges {
        let range = range.split('-').collect::<Vec<_>>();
        let left = range[0].parse::<i64>()?;
        let right = range[1].parse::<i64>()?;
        for x in left..=right {
            if is_match_2(x) {
                result += x;
            }
        }
    }
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_match_2() {
        println!("{}", is_match_2(2));
        println!("{}", is_match_2(11));
        println!("{}", is_match_2(12341234));
        println!("{}", is_match_2(123123123));
        println!("{}", is_match_2(1212121212));
        println!("{}", is_match_2(1111111));
    }
}
