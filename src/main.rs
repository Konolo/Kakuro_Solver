/*
*
* Author: Matthew Jacobs
* Created: July 2024
* Updated: March 26th, 2025
* Copyright: 2024 Matthew Jacobs
* Version: 0.1.0
*
*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::Colorize;

const CELL_WIDTH: usize = 5;

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
#[derive(Clone)]
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
  }
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - puzzle_file: A string which is the path to the file that contains the puzzle to be read in and solved
///
/// Description:
/// - This function reads in the puzzle from a file and establishes a grid which acts like a scaffold 
///      which allows the function to connect the parents to their children
fn insert_puzzle_and_connect_parents_and_children(parents_and_children: &mut (Vec<Parents>, Vec<Children>), puzzle_file: String) -> Vec<String> {
  // Creates a file object and buffer reader
  let file = File::open(puzzle_file).expect("Failed to open file");
  let reader = BufReader::new(file);
  let mut grid: Vec<Vec<GridCell>> = Vec::new();
  let mut board_pattern: Vec<String> = Vec::new();

  // Loop through each line of the file
  for line in reader.lines() {
    let line = line.expect("Failed to read line");
    board_pattern.push(line.clone());
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

  return board_pattern;
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

    // Flattening the combinations into HashSets of unique values
    let parent_1_values: HashSet<u8> = parent_1.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
    let parent_2_values: HashSet<u8> = parent_2.combinations.iter().flat_map(|v| v.iter()).cloned().collect();

    // Find intersection and collect into a Vec<u8>
    let intersection_values: Vec<u8> = parent_1_values.intersection(&parent_2_values).cloned().collect();

    if intersection_values.len() == 0 {
      panic!("Invalid puzzle");
    }

    // Append intersection values to child.possible_values
    child.possible_values.extend(intersection_values);
    child.possible_values.sort_unstable();

    if child.possible_values.len() > 1 { continue; }

    child.value = child.possible_values[0];
  }
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - puzzle_file: A string which is the path to the file that contains the puzzle to be read in and solved
///
/// Description:
/// - This function calls other functions which together set up the parents_and_children variable
///     to the point where the puzzle can be solved
fn puzzle_setup(parents_and_children: &mut (Vec<Parents>, Vec<Children>), puzzle_file: String) -> Vec<String> {
  let board_pattern = insert_puzzle_and_connect_parents_and_children(parents_and_children, puzzle_file);
  set_possible_combinations(parents_and_children);
  set_siblings_and_possible_values(parents_and_children);

  return board_pattern;
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - discovered_values: A mutable reference to a deque which contains the indexs of child cells that
///     can only be one value and that have not yet notified their parents and siblings
/// 
/// Description:
///  - This function examines all the children and adds them to the discovered_values deque if it can only be one value
fn starting_discovered_values(parents_and_children: &mut (Vec<Parents>, Vec<Children>), discovered_values: &mut VecDeque<usize>) {
  for (index, child) in parents_and_children.1.iter().enumerate() {
    if child.value == 0 { continue; }

    discovered_values.push_back(index);
  }
}


/*
/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - parents: A tuple which contains the indexes of both parent cells
/// - value: The value discovered by a child cell
/// 
/// Description:
/// - 
fn notify_parents(parents_and_children: &mut (Vec<Parents>, Vec<Children>), parents: &(usize, usize), value: &u8) {
  let (parents_vec, _) = parents_and_children;

  let (parent1, parent2) = {
    let (left, right) = parents_vec.split_at_mut(parents.1);
    (&mut left[parents.0], &mut right[0])
  };

  //println!("{:?}", parent1.combinations);
  //println!("{:?}", parent2.combinations);
  //println!("{}", value);

  parent1.combinations.retain(|combination| combination.contains(&value));
  parent2.combinations.retain(|combination| combination.contains(&value));

  let remaining_parent1_values: HashSet<u8> = parent1.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
  let remaining_parent2_values: HashSet<u8> = parent2.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
 
  for parent_info in [(parent1, remaining_parent1_values), (parent2, remaining_parent2_values)] {
    for child_index in &parent_info.0.children {
      let child_possible_values = &mut parents_and_children.1[*child_index].possible_values;

      //println!("{:?}", parent_info.1);
      //println!("{:?}", child_possible_values);

      child_possible_values.retain(|v| parent_info.1.contains(v));

      //println!("{:?}\n", child_possible_values);
    }
  }
  //println!("{:?}", remaining_parent1_combinations);
  //println!("{:?}\n", remaining_parent2_combinations);
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - discovered_values: A mutable reference to a deque which contains the indexs of child cells that
///     can only be one value and that have not yet notified their parents and siblings
/// - siblings: A list of indexes to all of the sibling cells
/// - value: The value discovered by a child cell
/// 
/// Description:
/// - 
fn notify_siblings(parents_and_children: &mut (Vec<Parents>, Vec<Children>), discovered_values: &mut VecDeque<usize>, siblings: &Vec<usize>, value: &u8) {
  for sibling in siblings {
    let child = &mut parents_and_children.1[*sibling];

    if child.value > 0 { continue; }

    //println!("{}", value);
    //println!("{:?}", child.possible_values);
    child.possible_values.retain(|v| v != value);
    //println!("{:?}\n", child.possible_values);

    if child.possible_values.len() > 1 { continue; }

    child.value = child.possible_values[0];
    discovered_values.push_back(*sibling);
  }
}

/// Parameters:
/// - parents_and_children: A mutable reference to a tuple containing a list of Parents and Children
///      - This variable contains all prevalent information for solving the puzzle
/// - discovered_values: A mutable reference to a deque which contains the indexs of child cells that
///     can only be one value and that have not yet notified their parents and siblings
/// 
/// Description:
/// - 
fn notify_parents_and_siblings(parents_and_children: &mut (Vec<Parents>, Vec<Children>), discovered_values: &mut VecDeque<usize>) {
  while !discovered_values.is_empty() {
    let child_index = discovered_values.pop_front().unwrap();
    let child = &parents_and_children.1[child_index].clone();

    notify_parents(parents_and_children, &child.parents, &child.value);
    notify_siblings(parents_and_children, discovered_values, &child.siblings, &child.value);
  }
}
*/


fn solver(parents_and_children: &mut (Vec<Parents>, Vec<Children>), discovered_values: &mut VecDeque<usize>) {
  while !discovered_values.is_empty() {
    let child_index = discovered_values.pop_front().unwrap();
    let child = &parents_and_children.1[child_index].clone();
    
    
    let (parents_vec, _) = parents_and_children;

    let (parent1, parent2) = {
      let (left, right) = parents_vec.split_at_mut(child.parents.1);
      (&mut left[child.parents.0], &mut right[0])
    };


    // 1) Takes the value of the discovered cell and makes sure that its parents possible combinations contain the value 
    //    from the discovered cell. 
    // 2) From there the parents determine all the possible values their children could have.
    // 3) Lastly, the parents inform all of their children of all the values they could be.

    // 1)
    parent1.combinations.retain(|combination| combination.contains(&child.value));
    parent2.combinations.retain(|combination| combination.contains(&child.value));
    
    // 2)
    let remaining_parent1_values: HashSet<u8> = parent1.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
    let remaining_parent2_values: HashSet<u8> = parent2.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
    
    // 3)
    for parent_info in [(parent1, remaining_parent1_values), (parent2, remaining_parent2_values)] {
      for index in &parent_info.0.children {
        let child_possible_values = &mut parents_and_children.1[*index].possible_values;
        
        child_possible_values.retain(|v| parent_info.1.contains(v));
      }
    }
    
    
    // 1) For each sibling cell of the discovered child cell, we make sure that the siblings value 
    //    cant be that of the discovered cell.
    // 2) For both parents of the sibling cell we compute its hashset then
    //    2a) We look at all of the combinations that the parent could be and only keep those that have at least one
    //        overlapping value with the possible values of its child (the sibling cell of the discovered cell).
    //    2b) After that we recompute the parents hashset, and if there is a difference, then we add the index of 
    //        that parent and their hashset to a list of parents who need to update their children
    // 3) Finally, if the sibling can only be one value, then their value is set and they are added to the list of discovered cells

    let mut parents_who_need_to_update_their_children: VecDeque<(usize, HashSet<u8>)> = VecDeque::new();
    // 1)
    for sibling_index in &child.siblings {
      let sibling = &mut parents_and_children.1[*sibling_index];
      
      if sibling.value > 0 { continue; }
      
      sibling.possible_values.retain(|v| v != &child.value);

      // 2)
      for parent_index in [&sibling.parents.0, &sibling.parents.1] {
        let parent = &mut parents_and_children.0[*parent_index];
        let parent_hashset_before: HashSet<u8> = parent.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
        // 2a)
        parent.combinations.retain(|combo| {
          for v in &sibling.possible_values {
            if combo.contains(v) {return true}
          }
          return false
        });
        // 2b)
        let parent_hashset_after: HashSet<u8> = parent.combinations.iter().flat_map(|v| v.iter()).cloned().collect();
        if parent_hashset_before.difference(&parent_hashset_after).count() > 0 {
          parents_who_need_to_update_their_children.push_back((*parent_index, parent_hashset_after.clone()));
        }
      }
        
      // 3)
      if sibling.possible_values.len() > 1 { continue; }
  
      sibling.value = sibling.possible_values[0];
      discovered_values.push_back(*sibling_index);
    }

    // For each parent who needs to update their children, they inform all of their children of all the values they could be.
    while !parents_who_need_to_update_their_children.is_empty() {
      let parent_index_and_hashset = parents_who_need_to_update_their_children.pop_front().unwrap();
      
      for child_index in &parents_and_children.0[parent_index_and_hashset.0].children {
        let child_possible_values = &mut parents_and_children.1[*child_index].possible_values;
        
        child_possible_values.retain(|v| parent_index_and_hashset.1.contains(v));
      }
    }




  }
}


fn print_kakuro_board(board_pattern: Vec<String>, parents_and_children: &(Vec<Parents>, Vec<Children>)) {
  let mut parent_index = 0;
  let mut child_index = 0;
  let mut board_width: usize = 0;
    
  for line in board_pattern {
    // Split the line into board pattern and clue segments
    let mut elements = line.split_whitespace();
    let board = elements.next().expect("Board pattern missing");
    board_width = (board.len() * (CELL_WIDTH + 1)) + 1;

    if parent_index == 0 {
      print_horizontal_border(board_width, true);
    } else {
      print_horizontal_border(board_width - 2, false);
    }

    for cell_type in board.chars() {
      match cell_type {
        '\\' => {
          let clue_data = elements.next().expect("Parent cell data missing");
          let mut values = clue_data.split('\\');
          let vert = values.next().unwrap_or("-").to_string();
          let horz = values.next().unwrap_or("-").to_string();
                
          print_parent_cell(
            &mut parent_index,
            &parents_and_children.0,
            vert,
            horz
          );
        },
        '-' => print_empty_cell(),
        'x' => print_child_cell(&mut child_index, &parents_and_children.1),
        _ => {
          eprintln!("Warning: Invalid character '{}' in board pattern", cell_type);
           print_empty_cell(); // Fallback to empty cell instead of panicking
        }
      }
    }
    println!("|");
  }  
  print_horizontal_border(board_width, true);
}

fn print_horizontal_border(width: usize, external_horizontal_border: bool) {
  if external_horizontal_border { println!("{}", "-".repeat(width)); }
  else { println!("|{}|", "-".repeat(width)); }
}

fn print_empty_cell() {
  print!("|{}", "     ".on_black());
}

fn print_parent_cell(parent_index: &mut usize, parents: &Vec<Parents>, vert_clue: String, horz_clue: String) {
  print!("|");
  
  let mut cell_content = String::with_capacity(CELL_WIDTH);
  
  // Handle vertical clue
  if vert_clue != "-" {
    let parent_val = format_parent_value(&parents[*parent_index].sum);
    cell_content.push_str(&parent_val);
    cell_content.push('\\');
    *parent_index += 1;
  } else {
    cell_content.push_str("  \\");
  }
  
  // Handle horizontal clue
  if horz_clue != "-" {
    let parent_val = format_parent_value(&parents[*parent_index].sum);
    cell_content.push_str(&parent_val);
    *parent_index += 1;
  } else {
    cell_content.push_str("  ");
  }
  
  print!("{}", cell_content.black().on_yellow());
}

fn format_parent_value(value: &u8) -> String {
  // Ensure two-digit format
  let value_str = value.to_string();
  if value_str.len() == 1 {
    format!("0{}", value_str)
  } else {
    value_str
  }
}

fn print_child_cell(child_index: &mut usize, children: &Vec<Children>) {
  let child_val = &children[*child_index].value;
  let cell_content = format!("  {}  ", child_val);
    
  print!("|{}", cell_content.black().on_white());
  *child_index += 1;
}

fn main() {
  let mut parents_and_children: (Vec<Parents>, Vec<Children>) = (Vec::new(), Vec::new());
  let puzzle_file = "puzzles\\Kakuro_input.txt".to_string();
  let mut discovered_values: VecDeque<usize>  = VecDeque::new(); // use push_back() & pop_front()

  let board_pattern = puzzle_setup(&mut parents_and_children, puzzle_file);
  starting_discovered_values(&mut parents_and_children, &mut discovered_values);

  if discovered_values.is_empty() {
    panic!("This puzzle currently can't be solved because of the complexity");
  }

  solver(&mut parents_and_children, &mut discovered_values);

  let mut count = 0;
  for child in &parents_and_children.1 {
    if child.value == 0 {
      count += 1;
      println!("{:?}", child);
    }
  }

  println!("{}", count);
  println!("Success");

  print_kakuro_board(board_pattern, &parents_and_children);
}