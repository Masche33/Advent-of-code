use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {

    //Reads the input file
    let input_file = BufReader::new(File::open("../input.txt")
        .expect("Fail to read file"));

    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();

    for line in input_file.lines() {
        for (i, word) in line.unwrap().split_whitespace().enumerate() {
            match i {
                0 => a.push(word.parse().unwrap()),
                1 => b.push(word.parse().unwrap()),
                _ => continue,
            }
        }
    }

    a.sort();
    b.sort();

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    let mut i = 0;
    for a_number in a {
        sum1 += (a_number as f32 - b[i] as f32).abs() as u32;
        sum2 += a_number * b.clone().into_iter().filter(|x| *x == a_number).count() as u32;
        i += 1;
    }

    println!("Part 1 result: {sum1}");
    println!("Part 2 result: {sum2}");

}
