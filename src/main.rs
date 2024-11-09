/*
*
* Author: Matthew Jacobs
* Created: July 2024
* Updated: November 2nd, 2024
* Copyright: Matthew Jacobs
* Version: 0.1.0
*
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests;

/// * if a value is negative then that type of cell does not exist in that location
/// * if the value is not negative, it is the index to the Parent or Children within the parents_and_children tuple
#[derive(Debug)]
struct GridCell {
  vert: i32,
  horz: i32,
  child: i32 
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Parents {
  children: Vec<usize>,
  sum: u8,
  value_size: String,
  combinations: Vec<Vec<u8>>
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Children {
  parents: (usize, usize),
  siblings: Vec<usize>,
  value: u8,
  possible_values: Vec<u8>
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
///
/// Description:
/// - Gathers the needed sum combinations from the parents, retrieves them from the precomputed list
///      of combinations, and adds it to the list of possible combinations for that parent
fn set_possible_combinations(parents_and_children: &mut (Vec<Parents>, Vec<Children>)) {
  // Creates a file object and buffer reader
  let file = File::open("combinations\\Kakuro_combinations.txt");
  let reader = BufReader::new(file.unwrap());
  let mut combinations: HashMap<String, Vec<String>> = HashMap::new();
  let mut list_of_combinations: HashSet<String> = HashSet::new();

  // creates a list of needed sum combinations from the parents 
  for parent in &parents_and_children.0 {
    list_of_combinations.insert(parent.value_size.as_str().to_string());
  }

  // loop through each line of the file
  for lines in reader.lines() {
    let line = lines.unwrap();

    // Splits the line into multiple segments
    let mut elements = line.as_str().split(" ");

    // extract the first item from elements i.e. 11-2
    let data: String = elements.next().unwrap().to_string();

    // Check if the data is in the list of needed sum combinations
    if !list_of_combinations.contains(&data) {
      continue;
    }

    // extract the second item from elements i.e. [3, 8]
    let values  = elements.next().unwrap().to_string();

    // either modify an existing entry by adding another combination to the Vector
    //    or create a new entry depending on if the key exists
    if combinations.contains_key(&data) {
      combinations.entry(data).and_modify(|combos| combos.push(values));
    } else {
      combinations.insert(data, vec![values]);
    }
  }

  // loop through all of the parents
  for parent in &mut parents_and_children.0 {
    let value_size = &parent.value_size;

    let combos = combinations.get(value_size).unwrap();

    // loop through all the possible combinations and add it to the parents combinations after it is in the proper form
    for combo in combos {
      let option = (*combo).as_str().to_string();

      // this takes a string array and turns it into a Vector of values whose type is u8
      let values: Vec<u8> = option // gets the string array of the combination i.e. [1, 2, 3]
        .trim_matches(&['[', ']'][..]) // Remove the brackets
        .split(',') // split at the commas
        .filter_map(|s| s.trim().parse::<u8>().ok()) // map through each element and parse it
        .collect(); // put all the parsed elements into a collection

      parent.combinations.push(values);
    }

   // println!("{:?}, {:?}", parent.value_size, parent.combinations);
  }


  //for combo in combinations {
    //println!("{:?}, {:?}", combo.0, combo.1);
  //}
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - puzzle_file: A string which is the path to the file that contains the puzzle to be read in and solved
///
/// Description:
/// - This function reads in the puzzle from a file and establishes a grid which acts like a scaffold 
///      which allows the function to connect the parents to their children
fn insert_puzzle_and_connect_parents_and_children(parents_and_children: &mut (Vec<Parents>, Vec<Children>), puzzle_file: String) {
  // Creates a file object and buffer reader
  let file = File::open(puzzle_file).expect("Failed to open file");
  let reader = BufReader::new(file);
  let mut grid: Vec<Vec<GridCell>> = Vec::new();

  // Loop through each line of the file
  for line in reader.lines() {
    let line = line.expect("Failed to read line");
    grid.push(Vec::new());
        
    // Split the line into multiple segments
    let mut elements = line.split_whitespace(); // Use `split_whitespace` to split by spaces
        
    // Extract the first item (the board) from elements
    let board = elements.next().unwrap();

    // loop through each character in the board
    for c in board.chars() {
      match c {
        '-' => {
          // if - push a nothing cell onto the end of the grid
          grid.last_mut().unwrap().push(GridCell { vert: -1, horz: -1, child: -1 });
        },
        '\\' => {
          // if \
          // grab the next string from element and split it at the \, then grab both strings individually
          let mut values = elements.next().unwrap().split('\\');
          let vert = values.next().unwrap_or("-").to_string();
          let horz = values.next().unwrap_or("-").to_string();
          let mut cell = GridCell { vert: -1, horz: -1, child: -1 };

          // loop through both of the strings collected above
          for (index, relation) in [vert, horz].iter().enumerate() {
            if relation != "-" {
              // if there is a value then set the proper GridCell attribute with the positional index
              let length = parents_and_children.0.len() as i32;
              if index == 0 { cell.vert = length; } else { cell.horz = length; }

              // split the string and parse out the size component, then add a new Parent to parents_and_children
              let sum_value: u8 = relation.parse().unwrap(); 
              parents_and_children.0.push(Parents { children: Vec::new(), sum: sum_value, value_size: "".to_string().to_string(), combinations: Vec::new() });
            }
          }

          grid.last_mut().unwrap().push(cell);
        },
        'x' => {
          // if x then add a child cell to the end of the grid and add a new Child to parents_and_children
          grid.last_mut().unwrap().push(GridCell { vert: -1, horz: -1, child: parents_and_children.1.len() as i32 });
          parents_and_children.1.push(Children { parents: (0, 0), siblings: Vec::new(), value: 0, possible_values: Vec::new() });
        },
        _ => panic!("Invalid character"),
      }
    }
  }

  // now that the grid is completely built loop through each row and column
  for (current_row_num, row) in grid.iter().enumerate() {
    for (current_col_num, col) in row.iter().enumerate() {

      // check that either horz or vert is set
      if col.horz == -1 && col.vert == -1 {
        continue;
      }

      // loop through the following twice, first as vert then as horz
      for relation in ["vert", "horz"] {
        // get the max index of and the Parent index of the vertical or horizontal
        let max_pos = if relation == "vert" { grid.len() } else { grid[0].len() };
        let relation_index = if relation == "vert" { col.vert } else { col.horz };
        let parent_cell = &grid[current_row_num][current_col_num];

        if relation_index != -1 {
          // if this Parent exists then get the current position on that axis as well as the parent position
          let mut pos_num = if relation == "vert" { current_row_num + 1 } else { current_col_num + 1 };
          let parent_position = if relation == "vert" { parent_cell.vert as usize } else { parent_cell.horz as usize };
  
          while pos_num < max_pos {
            // while still on the grid get the child index from the current grid position 
            let child_position = if relation == "vert" { grid[pos_num][current_col_num].child } else { grid[current_row_num][pos_num].child };
  
            // if the GridCell is not that of a child then break from the loop
            if child_position == -1 {
              break;
            }

            // grab the Child from parents_and_children whose index was just found
            let child = &mut parents_and_children.1[child_position as usize];

            // add the child to its Parents list of children
            parents_and_children.0[parent_position].children.push(child_position as usize);
  
            // this ensures that both parents are properly assigned and that one is not overwritten by the other on accident
            if relation == "vert" {
              child.parents.0 = parent_position;
            } else {
              child.parents.1 = parent_position;
            }

            pos_num += 1;
          }

          // calculates and set the Parents value_size 
          let parent = &mut parents_and_children.0[parent_position];
          parent.value_size = format!("{}-{}", parent.sum, parent.children.len());

        }
      }
    }
  }

  //for parent in &parents_and_children.0 {
    //println!("{:?}", parent);
  //}
  //println!("{}", parents_and_children.0.len());
  //for child in &parents_and_children.1 {
    //println!("{:?}", child);
  //}
  //println!("{}", parents_and_children.1.len());
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
///
/// Description:
/// - This function finds and assigns both the siblings and the possible values for each Child
fn set_siblings_and_possible_values(parents_and_children: &mut (Vec<Parents>, Vec<Children>)) {
  // loop through all of the Children
  for (index, child) in parents_and_children.1.iter_mut().enumerate() {
    // Get both parents of the selected child
    let parent_1 = &parents_and_children.0[child.parents.0];
    let parent_2 = &parents_and_children.0[child.parents.1];

    // sets the siblings of the child to be its parents' children that are not itself
    child.siblings.append(&mut parent_1.children.clone());
    child.siblings.append(&mut parent_2.children.clone());
    child.siblings.sort_unstable();
    child.siblings.retain(|e| e != &index);

    //println!("{:?} + {:?} = {:?}", parent_1.children, parent_2.children, child.siblings);

    // Flattening the combinations into HashSets of unique values
    let parent_1_values: HashSet<u8> = parent_1.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
    let parent_2_values: HashSet<u8> = parent_2.combinations.iter().flat_map(|v| v.iter()).cloned().collect();

    // Find intersection and collect into a Vec<u8>
    let intersection_values: Vec<u8> = parent_1_values.intersection(&parent_2_values).cloned().collect();

    // Append intersection values to child.possible_values
    child.possible_values.extend(intersection_values);
    child.possible_values.sort_unstable();
  }

  //for child in &parents_and_children.1 {
    //println!("{}", child);
  //}
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - puzzle_file: A string which is the path to the file that contains the puzzle to be read in and solved
///
/// Description:
/// - This function calls other functions which together set up the parents_and_children variable
///     to the point where the puzzle can be solved
fn puzzle_setup(parents_and_children: &mut (Vec<Parents>, Vec<Children>), puzzle_file: String) {
  insert_puzzle_and_connect_parents_and_children(parents_and_children, puzzle_file);
  set_possible_combinations(parents_and_children);
  set_siblings_and_possible_values(parents_and_children);
}

fn main() {
  let mut parents_and_children: (Vec<Parents>, Vec<Children>) = (Vec::new(), Vec::new());
  let puzzle_file = "puzzles\\Kakuro_input.txt".to_string();
  
  puzzle_setup(&mut parents_and_children, puzzle_file);

  println!("Success");
}