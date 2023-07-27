use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

pub fn quick_sort(vector: &Vec<i32>) { 
    let len = vector.len();
    _quick_sort(vector, 0, (len-1) as isize);
}

fn _quick_sort(vector: &Vec<i32>, low: isize, high: isize){ 
    if low < high { 
        let p = partition(vector, low, high);
        _quick_sort(vector, low, p-1);
        _quick_sort(vector, p+1, high);
    }
}

fn partition(vector: &Vec<i32>, low: isize, high: isize) -> isize { 
    let pivot = high as usize;
    let mut store_index = low-1;
    let mut last_index = high;

    loop { 
        store_index += 1;
        while vector[store_index as usize] < vector[pivot] { 
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && vector[last_index as usize] > vector[pivot] { 
            last_index -= 1;
        }
        if store_index >= last_index { 
            break;
        } else { 
            vector.swap(store_index as usize, last_index as usize);
        }
    }
    vector.swap(store_index as usize, pivot as usize);
    store_index
}
    
fn main() {
    // file path
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Attempt to open the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file: {}", error);
            return;
        }
    };

    let mut v = Vec::new();

    // Use a BufReader to read the file line by line
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Process each line in the file
    for line_result in reader.lines() {
        match line_result {
            Ok(line_content) => {
                if line_content.is_empty() {
                    v.push(sum);
                    sum = 0;
                } else {
                    match line_content.parse::<i32>() { 
                        Ok(number) => { 
                            sum = sum + number;
                        }
                        Err(error) => { 
                            eprintln!("Oopsies {}\n", error);
                            return;
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
            }
        }
    }
    
    quick_sort(&v);

    let a = v.pop().parse::<i32>();
    let b = v.pop().parse::<i32>();
    let c = v.pop().parse::<i32>();

    println!("{a}");
    println!("{b}");
    println!("{c}");

}
