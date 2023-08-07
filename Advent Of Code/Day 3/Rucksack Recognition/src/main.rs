// use std::fs::File;
// use std::io::{BufRead, BufReader};
mod utils;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args[1].is_empty() { 
        return;
    }

    let file_path = &args[1];

    let file = match File::open(file_path) { 
        Ok(file) => file,
        Err(error) => { 
            eprintln!("Error opening file {}", error);
            return;
        }
    };
    
    let mut v: Vec<utils::Item> = vec![];

    for (j,i) in (0x41..0x5B).enumerate() {
        let item = utils::Item {
            ascii:i,
            priority:(j as i32) + 27,
        };
        v.push(item);
    }

    for (j,i) in (0x61..0x7B).enumerate() {
        let item = utils::Item {
            ascii:i,
            priority:(j as i32) + 1,
        };

        v.push(item);
    }

    // println!("{:?}", v);

    let string = "ttgJtRGJQctTZtZT";
    let (mut compartement_1, mut compartement_2) = utils::split_to_compartements(string);

    // println!("{:?}", compartement_1);
    // println!("{:?}", compartement_2);

    utils::clean_compartement(&mut compartement_1);
    utils::clean_compartement(&mut compartement_2);
    // compartement_2.sort();

    // println!("{:?}", compartement_1);
    // println!("{:?}", compartement_2);
    
    let mut lost_and_found = vec![];
    let mut index: i32;

    for item in compartement_1 { 
        index = utils::binary_search(item, compartement_2.clone(), 0, (compartement_2.len() as i32) - 1);
        if index != -1 {
            // println!("Found");
            println!("item {} was found at index {}", compartement_2[index as usize], index);
            lost_and_found.push(item);
        }
    }

    for item in lost_and_found {
        index = utils::binary_search_item(item as u32, &v, 0, (v.len() as i32) - 1);
        if index != -1 { 
            // println!("Found");
            println!("item {} has priority {}", v[index as usize].ascii, v[index as usize].priority);
        }
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
