use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Parameters:
// - list: A list of all the sums with their sizes which are found in the puzzle to solve
// Takes the list of needed combinations and retrieves them from the precomputed list of combinations
fn get_possible_sum_combinations(list: &HashSet<String>) {
    // Creates a file object and buffer reader
    let file = File::open("D:\\Code\\Kakuro_combinations.txt");
    let reader = BufReader::new(file.unwrap());

    //println!("{:?}", *list);

    // loop through each line of the file
    for lines in reader.lines() {
        let line = lines.unwrap();

        // Splits the line into multiple segments
        let mut elements = line.as_str().split(" ");

        // extract the first item from elements
        let mut data = "";
        data = elements.next().unwrap();

        // Check if the data is in the list of needed sum combinations
        if !(*list).contains(data) {
            continue;
        }

        // extract the second item from elements
        let mut values = "";
        values = elements.next().unwrap();

        println!("{}, {}", data, values);
    }
}

// Parameters:
// - list: A list of all the sums with their sizes which are found in the puzzle to solve
//
fn insert_puzzle(list: &mut HashSet<String>) {
    // Creates a file object and buffer reader
    let file = File::open("D:\\Code\\Kakuro_input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Loop through each line of the file
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        
        // Split the line into multiple segments
        let mut elements = line.split_whitespace(); // Use `split_whitespace` to split by spaces
        
        // Extract the first item from elements
        let _board = elements.next().unwrap(); // Use `_board` if needed

        for set in elements {
            let mut values = set.split('\\');
            let vert = values.next().unwrap_or("-").to_string();
            let horz = values.next().unwrap_or("-").to_string();

            if vert != "-" {
                list.insert(vert);
            }
            if horz != "-" {
                list.insert(horz);
            }
        }
    }
    println!("List of all the sums & sizes: {:?}", list);
}

fn main() {
    let mut list: HashSet<String> = HashSet::new();

    insert_puzzle(&mut list);
    get_possible_sum_combinations(&list);
    println!("Hello, world!");
}
