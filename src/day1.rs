use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("/home/pavel/work/rust/advent/src/data/input_1.txt").unwrap();
    // let file = File::open("/home/pavel/work/rust/advent/src/data/test_1.txt").unwrap();
    let mut current = 50;
    let mut result = 0;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let left = line.chars().next().unwrap() == 'L';
        let mut shift = line[1..line.len()].parse::<i32>().unwrap();
        result += shift / 100;
        shift = shift % 100;
        let prev = current;
        current = if left {
            current - shift
        } else {
            current + shift
        };
        if (current < 0 || current > 99) && prev != 0 {
            result += 1;
        }
        if current == 0 && prev != 0 {
            result += 1;
        }
        current = (current + 100) % 100;
        println!("{},{},{}", line, current, result);
    }
    println!("{}", result);
}
