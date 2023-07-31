// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::env;

struct Item {
    ascii: u32,
    priority: i32,
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

    compartement_2.sort();
    compartement_1.sort();

    println!("{:?}", compartement_1);
    println!("{:?}", compartement_2);
    

    // for item in string.chars() { 
    //     let encoding = item as u32; 
    //     println!("{}", encoding);
    // }

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
