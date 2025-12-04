use memmap2::Mmap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

fn solve_first_half(lines: &Vec<Vec<char>>) -> (usize, Vec<(usize, usize)>) {
    let width = lines[0].len();
    let height = lines.len();
    let mut res = 0;
    let mut remove = vec![];

    for i in 0..lines.len() {
        for j in 0..width {
            if lines[i][j] == '@' {
                let mut cnt = 0;
                for k in 0..3 {
                    for l in 0..3 {
                        if k + i > 0
                            && k + i - 1 < height
                            && l + j > 0
                            && l + j - 1 < width
                            && lines[k + i - 1][l + j - 1] == '@'
                        {
                            cnt += 1
                        }
                    }
                }
                if cnt <= 4 {
                    // println!("{i} {j} {cnt}");
                    res += 1;
                    remove.push((i, j));
                }
            }
        }
    }
    // println!("res: {}", res);
    (res, remove)
}

fn solve_second_half(lines: &mut Vec<Vec<char>>) -> usize {
    let mut result = 0;

    let (mut removed, mut idx) = solve_first_half(lines);
    while removed > 0 {
        result += removed;
        for (i, j) in &idx {
            lines[*i][*j] = '.';
        }
        (removed, idx) = solve_first_half(lines);
    }
    println!("total: {result}");
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/data/input_4.txt")?;
    // let file = File::open("src/data/test_4.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let mut lines = mmap
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    // solve_first_half(&lines);
    solve_second_half(&mut lines);
    Ok(())
}
