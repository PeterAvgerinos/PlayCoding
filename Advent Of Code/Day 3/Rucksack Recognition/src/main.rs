mod utils;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


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

    // let mut sum: i32 = 0;
    // let mut lost_and_found = vec![];

    let reader = BufReader::new(file);
    let mut sum = 0;

    for line_result in &reader.lines().chunks(3) { 
        let line_result: Vec<_> = line_result.collect();
        let mut contents: Vec<_> = vec![];
        for item in line_result { 
            match item { 
                Ok(content) => { 
                    contents.push(utils::convert_to_ascii(content));
                }
                Err(_error) => { 
                    println!("Oopsies\n");
                    return;
                }
            }

        }

        contents[1].sort();
        contents[2].sort();

        for item in contents[0].clone() { 
            let index = utils::binary_search(item, &contents[1], 0, (contents[1].len() as i32) - 1);
            if index != -1 {
                let index2 = utils::binary_search(item, &contents[2], 0, (contents[2].len() as i32) - 1);
                if index2 != -1 {
                    let index3 = utils::binary_search_item(item as u32, &v, 0, (v.len() as i32) - 1);
                        if index3 != -1 { 
                            sum = sum + v[index3 as usize].priority;
                            break;
                        }
                }
            }
        }
    }

    println!("{}", sum);

}
