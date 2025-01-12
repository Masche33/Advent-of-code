use std::io;

fn main() {

    let mut first_type:u32 = 0;
    let mut second_type:u32 = 0;
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).
            expect("Error in line reading");
        if line == "" {break;}
        if first_type_safe(&line) {first_type+=1;}
        if second_type_safe(&line) {second_type+=1;}
    }
    println!("First safe type: {}; Second safe type {}",first_type, second_type);
}


fn first_type_safe(report :&str) -> bool{
    let numbers: Vec<i32> = report
        .split_whitespace()        // Splits the string in words
        .filter_map(|x| x.parse::<i32>().ok()) //Parse into u32
        .collect();

    // Verify all incrementing or decrementing
    if !(numbers.windows(2).all(|pair| pair[0] <= pair[1]) || // all incrementing
    numbers.windows(2).all(|pair| pair[0] >= pair[1]))    // all decrementing
    {return false;}

    return is_monotonic(&numbers) && valid_differences(&numbers) 
}

fn second_type_safe(report :&str) -> bool{
    let numbers: Vec<i32> = report
        .split_whitespace()        // Splits the string in words
        .filter_map(|x| x.parse::<i32>().ok()) //Parse into u32
        .collect();
    
    return is_monotonic(&numbers) && valid_differences(&numbers) || can_be_made_safe(numbers)
}

// Check if the levels are monotonic (either all increasing or all decreasing)
fn is_monotonic(levels: &[i32]) -> bool {
    levels.windows(2).all(|pair| pair[0] <= pair[1]) || // Increasing
    levels.windows(2).all(|pair| pair[0] >= pair[1])    // Decreasing
}

// Check if the differences between adjacent levels are between 1 and 3
fn valid_differences(levels: &[i32]) -> bool {
    levels.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs(); // Absolute difference
        diff >= 1 && diff <= 3
    })
}

fn can_be_made_safe(numbers: Vec<i32>) -> bool {
    for i in 0..numbers.len() {
        let mut modified = numbers.clone();
        modified.remove(i); // Remove the i-th element

        // Check if the remaining report is valid
        if is_monotonic(&modified) && valid_differences(&modified) {
            return true; // If valid after removing one element
        }
    }
    false // No valid configuration after removing any single element
}