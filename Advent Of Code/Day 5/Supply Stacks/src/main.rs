use std::io::{BufRead,BufReader};
use std::fs::File;
use std::env;

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

    let mut stacks: Vec<Vec<u32>> = Vec::new();

    match &results[8] {
        Ok(contents) => {
            let numbers: Vec<i32> = contents
                .trim() 
                .split_whitespace() 
                .map(|s| s.parse::<i32>().unwrap()) 
                .collect(); 
            if let Some(number_of_stacks) = numbers.last() {

                for _i in 0..*number_of_stacks { 
                    let small_stack: Vec<u32> = Vec::new();
                    stacks.push(small_stack);
                }

                for i in (0..8).rev() {
                    match &results[i] {
                        Ok(contents) => {
                            println!("{}", contents);
                        }
                        Err(_error) => {
                            println!("Oopsies\n");
                        }
                    }
                }


            }
            else { 
                println!("empty vector\n");
            }

        }
        Err(_error) => {
            println!("Oooopsies hehe\n");
            return;
        }
    }

    dbg!(stacks);


}
