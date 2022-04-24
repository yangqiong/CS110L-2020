use grid::Grid; // For lcs()
use std::cmp;
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut lines = vec![];
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        lines.push(line_str);
    }
    Ok(lines)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let m = seq1.len();
    let n = seq2.len();

    let mut grid = Grid::new(m + 1, n + 1);

    for i in 0..m + 1 {
        grid.set(i, 0, 0);
    }

    for j in 0..n + 1 {
        grid.set(0, j, 0);
    }

    for i in 0..m {
        for j in 0..n {
            if seq1[i] == seq2[j] {
                let value = grid.get(i, j).unwrap();
                grid.set(i + 1, j + 1, value + 1);
            } else {
                let value1 = grid.get(i + 1, j).unwrap();
                let value2 = grid.get(i, j + 1).unwrap();
                grid.set(i + 1, j + 1, cmp::max(value1, value2));
            }
        }
    }

    grid
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
        println!("  {}", lines1[i - 1]);
    } else if j > 0 && (i == 0 || lcs_table.get(i, j - 1) >= lcs_table.get(i - 1, j)) {
        print_diff(lcs_table, lines1, lines2, i, j - 1);
        println!("> {}", lines2[j - 1]);
    } else if i > 0 && (j == 0 || lcs_table.get(i, j - 1) < lcs_table.get(i - 1, j)) {
        print_diff(lcs_table, lines1, lines2, i - 1, j);
        println!("< {}", lines1[i - 1]);
    } else {
        println!();
    }
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let file1_lines = read_file_lines(filename1).expect("read filename1 error!");
    let file2_lines = read_file_lines(filename2).expect("read filename2 error");

    let grid = lcs(&file1_lines, &file2_lines);
    print_diff(
        &grid,
        &file1_lines,
        &file2_lines,
        file1_lines.len(),
        file2_lines.len(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
