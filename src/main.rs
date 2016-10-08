/*
 * Group Split
 * 
 * This program splits a delimeted file into multiple files, based on
 * the value of the specified column. Rows that share the same value are
 * concatenated into the same file.
 * 
 * ---------------------------------------------------------------------
 * 
 * This file is part of Group Split.
 * 
 * Group Split is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Group Split is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
 * 
 */

use std::io::{self, BufRead};
use std::fs::File;
use std::io::Write;

fn write_group(group: &Vec<Vec<String>>, delimeter: &str, file: &std::fs::File) {
  let mut file = file;
  
  for split in group {
    let _ = file.write_all((split.join(delimeter) + "\n").as_bytes());
  }
}

fn main() {
  // For this initial version, we are going to define a static delimeter
  // character. This should be specified by the user via command line
  // argument in future versions.
  const DELIMETER: &'static str = "\t";
  
  // We are also going to assume the input file has a header.
  const HAS_HEADER: bool = true;
  
  // We are also going to assume the first column is the group
  // identifier.
  const GROUP_IDENTIFIER: usize = 0;
  
  // We are also going to assume the number of files to be created. It
  // is important to note that if this number ever exceeds the number of
  // groups in the file, we will have no choice but to use the maximum
  // number of groups that is possible.
  const NUM_OUTPUT_FILES: u64 = 20;
  
  // Set up our file write descriptors
  let mut file_descriptors: Vec<std::fs::File> = vec!();
  
  for i in 0..NUM_OUTPUT_FILES {
    let w = (NUM_OUTPUT_FILES as f64).ln().floor() as usize;
    let file = File::create(format!("{:0width$}", i, width = w).to_string()).unwrap();
    file_descriptors.push(file);
  }
  
  // With those program options out of the way, we can start reading the
  // file into our program. The file should be piped into stdin.
  
  let stdin = io::stdin();
  
  let mut header: Vec<String> = vec!();
  let mut current_group_id: String = "\0".to_string();
  let mut current_group: Vec<Vec<String>> = vec!();
  
  let mut groups_processed: u64 = 0;
  let mut file_index: u64 = 0;
  
  for line in stdin.lock().lines() {
    let unwrapped: &str = &line.unwrap();
    let split: Vec<&str> = unwrapped.split(DELIMETER).collect();
    
    // Skip blank lines
    if "" == unwrapped {
      continue;
    }
    
    // Grab the header if necessary
    if header.is_empty() && HAS_HEADER {
      header = split.iter().map(|&v| v.to_string()).collect::<Vec<_>>();
      continue;
    }
    
    // First group bootstrap procedure
    if "\0" == current_group_id {
      current_group_id = split[GROUP_IDENTIFIER].clone().to_string();
      current_group.push(header.clone());
    }
    
    // Does this line belong to the group we just processed? If it
    // doesn't, we're going to stop what we're doing and write out the
    // previous group. Otherwise, we'll just be adding this line to the
    // current group.
    if &current_group_id != &split[GROUP_IDENTIFIER].to_string() {
      write_group(&current_group, DELIMETER, &file_descriptors[file_index as usize]);
      
      groups_processed += 1;
      file_index = groups_processed % NUM_OUTPUT_FILES;
      
      // Clear out the old group, make the new one
      current_group_id = split[GROUP_IDENTIFIER].clone().to_string();
      current_group = vec!();
      
      // If this is the first group of this file's kind, it's going to
      // need a header.
      if groups_processed < NUM_OUTPUT_FILES {
        current_group.push(header.clone());
      }
    } else {
      let row_to_add = split.iter().map(|&x| x.to_string()).collect::<Vec<_>>();
      current_group.push(row_to_add);
    }
  }
  
  write_group(&current_group, DELIMETER, &file_descriptors[file_index as usize]);
}
