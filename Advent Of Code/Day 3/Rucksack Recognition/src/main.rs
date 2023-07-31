// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::env;

struct Item {
    ascii: u32,
    priority: i32,
}

fn split_to_compartements(string: &str) -> (Vec<u32>,Vec<u32>) { 
    let mut compartement_1: Vec<u32> = vec![];
    let mut compartement_2: Vec<u32> = vec![];

    let length = string.len();

    for (i,item) in string.chars().enumerate() {
        if i<length/2 { 
            compartement_1.push(item as u32);
        }
        else {
            compartement_2.push(item as u32);
        }
    }

    return (compartement_1, compartement_2);
}

fn clean_compartement(compartement:&mut Vec<u32>) -> () {
    for (i,item) in compartement.clone().into_iter().enumerate() {
        for j in i+1..compartement.len() { 
            if item == compartement[j] { 
                compartement[j] = 0;
            }
            else { 
                continue;
            }
        }
    }

    compartement.retain(|x| *x!=0);
}

fn main() {

    // let args: Vec<String> = env::args().collect();
    // if args[1].is_empty() { 
    //     return;
    // }
    //
    // let file_path = &args[1];
    //
    // let file = match File::open(file_path) { 
    //     Ok(file) => file,
    //     Err(error) => { 
    //         eprintln!("Error opening file {}", error);
    //         return;
    //     }
    // };
    
    let string = "ttgJtRGJQctTZtZT";
    let (mut compartement_1, mut compartement_2) = split_to_compartements(string);

    println!("{:?}", compartement_1);
    println!("{:?}", compartement_2);

    clean_compartement(&mut compartement_1);
    clean_compartement(&mut compartement_2);

    println!("{:?}", compartement_1);
    println!("{:?}", compartement_2);

    let mut v: Vec<Item> = vec![];

    for (j,i) in (0x41..0x5B).enumerate() {
        let item = Item {
            ascii:i,
            priority:(j as i32) + 27,
        };
        println!("{} {}", item.ascii, item.priority);
        v.push(item);
    }

    for (j,i) in (0x61..0x7B).enumerate() {
        let item = Item {
            ascii:i,
            priority:(j as i32) + 1,
        };
        println!("{} {}", item.ascii, item.priority);

        v.push(item);
    }


    // let reader = BufReader::new(file);
    //
    // for line_result in reader.lines() { 
    //     match line_result { 
    //         Ok(line_content) => {
    //             if line_content.is_empty() { 
    //                 return;
    //             }
    //             else { 
    //                 continue;
    //             }
    //         }
    //         Err(error) => { 
    //             eprintln!("Error reading file {}", error);
    //             return;
    //         }
    //     }
    // }

}
