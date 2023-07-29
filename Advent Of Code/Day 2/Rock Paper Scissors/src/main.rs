use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

static ROCK: String = "Rock".to_string();
static PAPER: String = "Paper".to_string();
static SCISSOR: String = "Scissors".to_string();

struct Turn { 
    opponent: String,
    my_move: String,
    wld: i32,
}

fn find_move(opponent: String) -> String { 


}

fn check_wld(mv: String) -> i32 {
    if mv == "X" {return -1;}
    else if mv == "Y" {return 0;}
    else {return 1;}
}

fn clean_string(mv: String) -> String { 
    if (mv == "A") || (mv == "X") {return "Rock".to_string();}
    else if (mv == "B") || (mv == "Y") { return "Paper".to_string();}
    else if (mv == "C") || (mv == "Z") { return "Scissors".to_string();}
    else {return "".to_string();}
}

fn check2(turn: Turn) -> i32 { 
    let mut sum = 0;
    if turn.opponent == ROCK { 


    }

}

fn check(turn: Turn) -> i32 { 
    let mut sum = 0;
    if turn.my_move == ROCK { 
        sum = sum + 1;
        if turn.opponent == PAPER {return sum;}
        else if turn.opponent == SCISSOR {return sum+6;}
        else if turn.opponent == ROCK {return sum+3;}
        return -1;
    }
    else if turn.my_move == PAPER { 
        sum = sum + 2;
        if turn.opponent == PAPER {return sum+3;}
        else if turn.opponent == SCISSOR {return sum;}
        else if turn.opponent == ROCK {return sum+6;}
        else {return -1;}
    }
    else if turn.my_move == SCISSOR { 
        sum = sum + 3;
        if turn.opponent == PAPER {return sum+6;}
        else if turn.opponent == SCISSOR {return sum+3;}
        else if turn.opponent == ROCK {return sum;}
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
                    let opponent = clean_string(splitted_string[0].to_string());
                    let my_move = set_my_move(opponent, splitted_string[1].to_string());
                                
                }
            }
            Err(_error) => { 
                return;
            }

        }

    }

    println!("{}", sum);
}
