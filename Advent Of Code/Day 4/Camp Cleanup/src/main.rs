use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};


// ..... 4 5 6 7 ......
// ... 3 4 5 6 ........

fn check_overlap(v: &Vec<i32>) -> bool { 

    if check_redundancy(v) {return true;}

    let first_of_first = v[0];
    let second_of_first = v[1];
    let first_of_second = v[2];
    let second_of_second = v[3];

    if (first_of_second <= first_of_first) && (first_of_first <= second_of_second) {
        return true;
    }
    else if (first_of_second <= second_of_first) && (second_of_first <= second_of_second) {
        return true;
    }
    else { 
        return false;
    }

}

fn check_redundancy(v: &Vec<i32>) -> bool { 
    let first_of_first = v[0];
    let second_of_first = v[1];
    let first_of_second = v[2];
    let second_of_second = v[3];

    if (first_of_first <= first_of_second) && (second_of_first >= second_of_second) {
        return true;
    }
    else if (first_of_second <= first_of_first) && (second_of_second >= second_of_first)   {
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
            println!("Oopsies!\n");
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

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
                if check_redundancy(&numbers) {sum1 = sum1 + 1;}
                if check_overlap(&numbers) {sum2 = sum2 + 1;}

            }
            Err(_error) => {
                println!("Double Oopsies!\n");
            }
        }

    }

    println!("{} {}", sum1, sum2);
}
