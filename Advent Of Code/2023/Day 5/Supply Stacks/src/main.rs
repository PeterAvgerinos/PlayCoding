use std::io::{BufRead,BufReader};
use std::fs::File;
use std::env;

fn is_item(item: char) -> bool { 
    if ((item as u32) <= 90) && ((item as u32) >= 65) { return true;}
    else { 
        return false;
    }
}

fn play_move(amount_of_pops: i32, stacks: &mut Vec<Vec<char>>, source: i32, destination: i32) -> () {
    for _ in 0..amount_of_pops {
        let item_to_move = match stacks[(source-1) as usize].pop() {
            Some(v) => v,
            None => return
        };
        stacks[(destination-1) as usize].push(item_to_move);
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

    for i in (0..=8).rev() {
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

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in &contents_of_stacks_as_lists { 
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }

    for item in contents_of_stacks_as_lists { 
        for (index,character) in item.iter().enumerate() { 
            if is_item(*character) { 
                for i in 0..indices.len() { 
                    if index == indices[i] { 
                        stacks[i].push(*character);
                    }
                }
            }
        }
    }
    // dbg!(stacks);



    for i in 10..results.len() { 
        let mv = match &results[i] { 
            Ok(content) => content,
            Err(_error) => return
        };

        let mut numbers: Vec<i32> = Vec::new();


        let mut current_number = String::new();
    
        for c in mv.chars() {
            if c.is_digit(10) {
                current_number.push(c);
            } else if !current_number.is_empty() {
                numbers.push(current_number.parse::<i32>().unwrap());
                current_number.clear();
            }
        }

        if !current_number.is_empty() {
            numbers.push(current_number.parse::<i32>().unwrap());
        }
        
        let amount_of_pops = numbers[0];
        let source = numbers[1];
        let destination = numbers[2];

        play_move(amount_of_pops, &mut stacks, source, destination);





    }

    let mut on_top: Vec<String> = vec![];

    for item in stacks { 
        let top = match item.last() {
            Some(content) => content,
            None => return
        };

        on_top.push(top.to_string());
    }


    for item in on_top {
        print!("{}", item);
    }

    println!();
}
