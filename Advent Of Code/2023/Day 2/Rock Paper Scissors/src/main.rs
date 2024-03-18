use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

static ROCK: &str = "Rock";
static PAPER: &str = "Paper";
static SCISSOR: &str = "Scissors";
static EMPTY: &str = "";

struct Turn<'a> { 
    opponent: &'a str,
    my_move: &'a str,
}

fn clean_string<'a>(mv: &str) -> &'a str { 
    if (mv == "A") || (mv == "X") {return &ROCK}
    else if (mv == "B") || (mv == "Y") { return &PAPER}
    else if (mv == "C") || (mv == "Z") { return &SCISSOR}
    else {return &EMPTY}
}

fn set_my_move<'a>(opponent: &str, outcome: &str) -> &'a str { 
    if outcome == "X" { 
        if opponent == ROCK {return &SCISSOR;}
        else if opponent == PAPER {return &ROCK;}
        else if opponent == SCISSOR {return &PAPER;}
        else {return &EMPTY;}
    }
    else if outcome == "Y" { 
        if opponent == ROCK {return &ROCK;}
        else if opponent == PAPER {return &PAPER;}
        else if opponent == SCISSOR {return &SCISSOR;}
        else {return &EMPTY;}
    }
    else { 
        if opponent == ROCK {return &PAPER;}
        else if opponent == PAPER {return &SCISSOR;}
        else if opponent == SCISSOR {return &ROCK;}
        else {return &EMPTY;}
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
                    let opponent = clean_string(splitted_string[0]);
                    let my_move = set_my_move(&opponent, splitted_string[1]);
                    println!("{} {}", opponent, my_move);
                    let turn = Turn { 
                        opponent: &opponent,
                        my_move: &my_move,
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
