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
            eprintln!("Error opening file: {}", error);
            return;
        }
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines()  { 
        match line_result { 
            Ok(line_content) => {
                if line_content.is_empty() { 
                    continue;
                }
                else { 
                    let mut p = Vec::new();
                    for item in line_content.split_whitespace() { 
                        p.push(item);
                    }
                    println!("{}, {}", p[0], p[1]);
                }
            }
            Err(_error) => { 
                return;
            }

        }


    }
}
