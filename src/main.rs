/*
*
* Author: Matthew Jacobs
* Created: July 2024
* Updated: November 2nd, 2024
* Version: 0.1.0
*
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct GridCell {
  vert: i8,
  horz: i8,
  child: i8 
}

#[derive(Debug)]
struct Parents {
  children: Vec<u16>,
  sum: u8,
  value_size: String,
  combinations: Vec<Vec<u8>>
}

#[derive(Debug)]
struct Children {
  parents: (u16, u16),
  neighbors: Vec<u16>,
  value: u8,
  possible_values: Vec<u8>
}

// Parameters:
// - list: A list of all the sums with their sizes which are found in the puzzle to solve
// Takes the list of needed combinations and retrieves them from the precomputed list of combinations
fn get_possible_sum_combinations(parents_and_children: &mut (Vec<Parents>, Vec<Children>)) {
  // Creates a file object and buffer reader
  let file = File::open("D:\\Code\\Kakuro_combinations.txt");
  let reader = BufReader::new(file.unwrap());
  let mut combinations: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
  let mut list_of_combinations: HashSet<String> = HashSet::new();

  for parent in &parents_and_children.0 {
    list_of_combinations.insert(parent.value_size.as_str().to_string());
  }

  // loop through each line of the file
  for lines in reader.lines() {
    let line = lines.unwrap();

    // Splits the line into multiple segments
    let mut elements = line.as_str().split(" ");

    // extract the first item from elements
    let data: String = elements.next().unwrap().to_string();

    // Check if the data is in the list of needed sum combinations
    if !list_of_combinations.contains(&data) {
      continue;
    }

    // extract the second item from elements
    let values: Vec<u8> = elements.next().unwrap().to_string() // gets the string array of the combination i.e. [1, 2, 3]
      .trim_matches(&['[', ']'][..]) // Remove the brackets
      .split(',') // split at the commas
      .filter_map(|s| s.trim().parse::<u8>().ok()) // map through each element and parse it
      .collect(); // put all the parsed elements into a collection

    if combinations.contains_key(&data) {
      combinations.entry(data).and_modify(|combos| combos.push(values));
    } else {
      combinations.insert(data, vec![values]);
    }
  }

  for parent in &parents_and_children.0 {
    todo!();
  }

  for combo in combinations {
    println!("{}, {:?}\n", combo.0, combo.1);
  }
}

// Parameters:
// - list: A list of all the sums with their sizes which are found in the puzzle to solve
//
fn insert_puzzle(parents_and_children: &mut (Vec<Parents>, Vec<Children>)) {
  // Creates a file object and buffer reader
  let file = File::open("D:\\Code\\Kakuro_input.txt").expect("Failed to open file");
  let reader = BufReader::new(file);
  let mut grid: Vec<Vec<GridCell>> = Vec::new();

  // Loop through each line of the file
  for line in reader.lines() {
    let line = line.expect("Failed to read line");
    grid.push(Vec::new());
    //println!("{}", grid.len().to_string());
        
    // Split the line into multiple segments
    let mut elements = line.split_whitespace(); // Use `split_whitespace` to split by spaces
        
    // Extract the first item (the board) from elements
    let board = elements.next().unwrap();

    for c in board.chars() {
      match c {
        '-' => {
          grid.last_mut().unwrap().push(GridCell { vert: -1, horz: -1, child: -1 });
        },
        '\\' => {
          let mut values = elements.next().unwrap().split('\\');
          let vert = values.next().unwrap_or("-").to_string();
          let horz = values.next().unwrap_or("-").to_string();
          let mut cell = GridCell { vert: -1, horz: -1, child: -1 };

          if vert != "-" {
            cell.vert = parents_and_children.0.len() as i8;
            let sum_value: u8 = vert.split("-").next().unwrap().parse().unwrap(); 
            parents_and_children.0.push(Parents { children: Vec::new(), sum: sum_value, value_size: vert, combinations: Vec::new() });
          }

          if horz != "-" {
            cell.horz = parents_and_children.0.len() as i8;
            let sum_value: u8 = horz.split("-").next().unwrap().parse().unwrap();
            parents_and_children.0.push(Parents { children: Vec::new(), sum: sum_value, value_size: horz, combinations: Vec::new() });
          }

          grid.last_mut().unwrap().push(cell);
        },
        'x' => {
          grid.last_mut().unwrap().push(GridCell { vert: -1, horz: -1, child: parents_and_children.1.len() as i8 });
          parents_and_children.1.push(Children { parents: (0, 0), neighbors: Vec::new(), value: 0, possible_values: Vec::new() });
        },
        _ => println!("ERROR"),
      }
    }
  }

  let max_rows = grid.len();
  let max_cols = grid[0].len();

  for (current_row_num, row) in grid.iter().enumerate() {
    for (current_col_num, col) in row.iter().enumerate() {

      if col.horz == -1 && col.vert == -1 {
        continue;
      }

      if col.horz != -1 {
        let mut col_num = current_col_num + 1;
        let parent_position = grid[current_row_num][current_col_num].horz as usize;

        while col_num < max_cols {
          let child_position = grid[current_row_num][col_num].child;

          if child_position == -1 {
            break;
          }

          parents_and_children.0[parent_position].children.push(child_position as u16);

          col_num += 1;
        }
      }

      if col.vert != -1 {
        let mut row_num = current_row_num + 1;
        let parent_position = grid[current_row_num][current_col_num].vert as usize;

        while row_num < max_rows {
          let child_position = grid[row_num][current_col_num].child;

          if child_position == -1 {
            break;
          }

          parents_and_children.0[parent_position].children.push(child_position as u16);

          row_num += 1;
        }
      }
    }
  }

  for parent in &parents_and_children.0 {
    println!("{:?}", parent);
  }
  println!("{}", parents_and_children.0.len());
  for child in &parents_and_children.1 {
    println!("{:?}", child);
  }
  println!("{}", parents_and_children.1.len());
}

fn main() {
  let mut list_of_combinations: HashSet<String> = HashSet::new();
  let mut parents_and_children: (Vec<Parents>, Vec<Children>) = (Vec::new(), Vec::new());

  insert_puzzle(&mut parents_and_children);
  get_possible_sum_combinations(&mut parents_and_children);
  println!("Hello, world!");
}
