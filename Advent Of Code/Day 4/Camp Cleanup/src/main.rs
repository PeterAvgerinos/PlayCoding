use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn check(v: &Vec<i32>) -> bool { 

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
            println!("Oopsies!\n");
            return;
        }
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines() { 
        match line_result { 
            Ok(line_content) => { 
                let parts = line_content.split(",");
                let parts: Vec<_> = parts.collect();
                let mut numbers: Vec<i32> = vec![];
                for item in parts { 
                    let inner_parts = item.split("-");
                    let inner_parts: Vec<_> = inner_parts.collect();
                    for part in inner_parts { 
                        let number: i32 = part.parse().unwrap();
                        numbers.push(number);
                    }

                }

                println!("{:?}", numbers);

            }
            Err(_error) => {
                println!("Double Oopsies!\n");
            }
        }

    }
}
