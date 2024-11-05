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
  children: Vec<usize>,
  sum: u8,
  value_size: String,
  combinations: Vec<Vec<u8>>
}

#[derive(Debug)]
struct Children {
  parents: (usize, usize),
  siblings: Vec<usize>,
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
  let mut combinations: HashMap<String, Vec<String>> = HashMap::new();
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
    let values  = elements.next().unwrap().to_string();

    if combinations.contains_key(&data) {
      combinations.entry(data).and_modify(|combos| combos.push(values));
    } else {
      combinations.insert(data, vec![values]);
    }
  }

  for parent_index in 0..parents_and_children.0.len() {
    let value_size = &parents_and_children.0[parent_index].value_size;

    let combos = combinations.get(value_size).unwrap();

    for combo in combos {
      let option = (*combo).as_str().to_string();

      let values: Vec<u8> = option // gets the string array of the combination i.e. [1, 2, 3]
        .trim_matches(&['[', ']'][..]) // Remove the brackets
        .split(',') // split at the commas
        .filter_map(|s| s.trim().parse::<u8>().ok()) // map through each element and parse it
        .collect(); // put all the parsed elements into a collection

      parents_and_children.0[parent_index].combinations.push(values);
    }

    println!("{:?}, {:?}", &parents_and_children.0[parent_index].value_size, &parents_and_children.0[parent_index].combinations);
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

          let mut is_horz = false;
          for relation in [vert, horz] {
            if relation != "-" {
              let length = parents_and_children.0.len() as i8;
              if is_horz { cell.horz = length; } else { cell.vert = length; }

              let sum_value: u8 = relation.split("-").next().unwrap().parse().unwrap(); 
              parents_and_children.0.push(Parents { children: Vec::new(), sum: sum_value, value_size: relation, combinations: Vec::new() });
            }
            is_horz = true;
          }

          grid.last_mut().unwrap().push(cell);
        },
        'x' => {
          grid.last_mut().unwrap().push(GridCell { vert: -1, horz: -1, child: parents_and_children.1.len() as i8 });
          parents_and_children.1.push(Children { parents: (65535, 65535), siblings: Vec::new(), value: 0, possible_values: Vec::new() });
        },
        _ => println!("ERROR"),
      }
    }
  }

  for (current_row_num, row) in grid.iter().enumerate() {
    for (current_col_num, col) in row.iter().enumerate() {

      if col.horz == -1 && col.vert == -1 {
        continue;
      }

      for relation in ["vert", "horz"] {
        let max_pos = if relation == "vert" { grid.len() } else { grid[0].len() };
        let relation_index = if relation == "vert" { col.vert } else { col.horz };
        let parent_cell = &grid[current_row_num][current_col_num];

        if relation_index != -1 {
          let mut pos_num = if relation == "vert" { current_row_num + 1 } else { current_col_num + 1 };
          let parent_position = if relation == "vert" { parent_cell.vert as usize } else { parent_cell.horz as usize };
  
          while pos_num < max_pos {
            let child_position = if relation == "vert" { grid[pos_num][current_col_num].child } else { grid[current_row_num][pos_num].child };
  
            if child_position == -1 {
              break;
            }
  
            parents_and_children.0[parent_position].children.push(child_position as usize);
  
            pos_num += 1;
          }
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

fn connect_children_to_parents(parents_and_children: &mut (Vec<Parents>, Vec<Children>)) {
  for (index, parent) in parents_and_children.0.iter().enumerate() {
    for child_index in &parent.children {
      let child = &mut parents_and_children.1[*child_index];
      let parents_of_child = &mut child.parents;
      
      // 65535 is used to ensure the first parent is vertical
      if parents_of_child.0 == 65535 {
        parents_of_child.0 = index;
      } else {
        parents_of_child.1 = index;
      }

      // sets the siblings of the child to be its parents' children that are not itself
      child.siblings.append(&mut parent.children.clone());
      child.siblings.retain(|e| e != child_index);
    }
    println!("{:?}", parent);
  }

  for child in &mut parents_and_children.1 {
    // Get both parents of the selected child
    let parent_1 = &parents_and_children.0[child.parents.0];
    let parent_2 = &parents_and_children.0[child.parents.1];

    // Flattening the combinations into HashSets of unique values
    let parent_1_values: HashSet<u8> = parent_1.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
    let parent_2_values: HashSet<u8> = parent_2.combinations.iter().flat_map(|v| v.iter()).cloned().collect();

    // Find intersection and collect into a Vec<u8>
    let intersection_values: Vec<u8> = parent_1_values.intersection(&parent_2_values).cloned().collect();

    // Append intersection values to child.possible_values
    child.possible_values.extend(intersection_values);
  }

  for child in &parents_and_children.1 {
    println!("{:?}", child);
  }
}

fn main() {
  let mut parents_and_children: (Vec<Parents>, Vec<Children>) = (Vec::new(), Vec::new());

  insert_puzzle(&mut parents_and_children);
  get_possible_sum_combinations(&mut parents_and_children);
  connect_children_to_parents(&mut parents_and_children);
  println!("Hello, world!");
}
