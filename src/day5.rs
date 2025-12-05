use memmap2::Mmap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct IngRange {
    start: u64,
    end: u64,
}

impl PartialOrd for IngRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.start
                .cmp(&other.start)
                .then(self.end.cmp(&other.end).reverse()),
        )
    }
}

fn solve_first_half(mut lines: Vec<String>) {
    let mut ids: Vec<u64> = Vec::new();
    let mut ranges: Vec<IngRange> = Vec::new();

    while let Some(num) = lines.pop() {
        if num.is_empty() {
            break;
        }
        let num = num.parse::<u64>().unwrap();
        ids.push(num);
    }
    while let Some(range_str) = lines.pop() {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();
        ranges.push(IngRange { start, end });
    }

    ranges.sort();
    let mut merged_ranges: Vec<IngRange> = vec![ranges.first().unwrap().clone()];

    for range in ranges.iter().skip(1) {
        let last = merged_ranges.pop().unwrap();
        if range.start <= last.end {
            let new_end = std::cmp::max(last.end, range.end);
            merged_ranges.push(IngRange {
                start: last.start,
                end: new_end,
            });
        } else {
            merged_ranges.push(last);
            merged_ranges.push(range.clone());
        }
    }

    let mut res = 0;
    for id in ids {
        for range in &merged_ranges {
            if id >= range.start && id <= range.end {
                res += 1;
                break;
            }
        }
    }

    println!("fresh ids: {}", res);

    let mut fresh_ranges: u64 = 0;
    for range in &merged_ranges {
        fresh_ranges += range.end - range.start + 1;
    }

    println!("fresh ranges: {}", fresh_ranges);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/data/input_5.txt")?;
    // let file = File::open("src/data/test_5.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let lines = mmap.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    solve_first_half(lines);
    Ok(())
}
