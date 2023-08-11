use std::io::{BufRead,BufReader};
use std::fs::File;
use std::env;

fn is_item(item: char) -> bool { 
    if ((item as u32) <= 90) && ((item as u32) >= 65) { 
        return true;
    }
    else { 
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].is_empty() {
        return;
    }

    let file_path = &args[1];

    let file = match File::open(file_path) { 
        Ok(file) => file,
        Err(_error) => {
            println!("Ooopsies\n");
            return;
        }
    };

    let reader = BufReader::new(file);

    let results: Vec<_> = reader.lines().collect();
    let mut contents_of_stacks_as_lists: Vec<Vec<char>> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();

    for i in (0..8).rev() {
        let line = match &results[i] {
            Ok(p) => p,
            Err(_error) => return
        };

        let line_as_list: Vec<char> = line.chars().collect();


        if i == 7 { 
            for i in 0..(line_as_list.len() as usize) { 
                if is_item(line_as_list[i]){ 
                    indices.push(i);
                } 
            }
        }
        contents_of_stacks_as_lists.push(line.chars().collect());
    }

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(indices.len());

    for item in contents_of_stacks_as_lists { 
        for i in 0..indices.len() {
            if is_item(item[i]) { 
                stacks[i].push(item[i]);

            }
        }
    }

    dbg!(stacks);
}
