use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

struct Turn { 
    opponent: String,
    my_move: String,
}

fn clean_string(mv: String) -> String { 
    if (mv == "A") || (mv == "X") {return "Rock".to_string();}
    else if (mv == "B") || (mv == "Y") { return "Paper".to_string();}
    else if (mv == "C") || (mv == "Z") { return "Scissors".to_string();}
    else {return "".to_string();}
}

fn check(turn: Turn) -> i32 { 
    let rock: String = "Rock".to_string();
    let paper: String = "paper".to_string();
    let scissor: String = "scissor".to_string();
    let mut sum = 0;
    if turn.my_move == rock { 
        sum = sum + 1;
        if turn.opponent == paper {return sum;}
        else if turn.opponent == scissor {return sum+6;}
        else if turn.opponent == rock {return sum+3;}
        return -1;
    }
    else if turn.my_move == paper { 
        sum = sum + 2;
        if turn.opponent == paper {return sum+3;}
        else if turn.opponent == scissor {return sum;}
        else if turn.opponent == rock {return sum+6;}
        else {return -1;}
    }
    else if turn.my_move == scissor { 
        sum = sum + 3;
        if turn.opponent == paper {return sum+6;}
        else if turn.opponent == scissor {return sum+3;}
        else if turn.opponent == rock {return sum;}
        else {return -1;}
    }
    else {return -1;}

}

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

    let mut sum = 0;

    let reader = BufReader::new(file);

    for line_result in reader.lines()  { 
        match line_result { 
            Ok(line_content) => {
                if line_content.is_empty() { 
                    continue;
                }
                else { 
                    let splitted_string: Vec<&str> = line_content.split(" ").collect();
                    let turn = Turn { 
                        opponent: clean_string(splitted_string[0].to_string()),
                        my_move: clean_string(splitted_string[1].to_string()),
                    };
                                
                    sum = sum + check(turn);
                }
            }
            Err(_error) => { 
                return;
            }

        }

    }

    println!("{}", sum);
}
