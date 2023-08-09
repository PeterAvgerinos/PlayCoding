#[derive(Clone)]
#[derive(Debug)]
pub struct Item {
    pub ascii: u32,
    pub priority: i32,
}

pub fn binary_search_item(item: u32, vec: &Vec<Item>, lower: i32, upper: i32) -> i32 {
    if lower <= upper { 
        let mid = (lower+upper)/2;
        // println!("{}", vec[mid as usize].ascii);
        if item == vec[mid as usize].ascii {
            return mid;
        } else if item < vec[mid as usize].ascii { 
            return binary_search_item(item, vec, lower, mid-1);
        } else {return binary_search_item(item, vec, mid+1, upper);}
    }

    return -1;
}

pub fn binary_search(item: u32, vec: &Vec<u32>, lower: i32, upper: i32) -> i32 {
    if lower <= upper { 
        let mid = (lower+upper)/2;
        if item == vec[mid as usize] {
            return mid;
        } else if item < vec[mid as usize] { 
            return binary_search(item, vec, lower, mid-1);
        } else {return binary_search(item, vec, mid+1, upper);}
    }

    return -1;
}

pub fn convert_to_ascii(string: String) -> Vec<u32> { 
    let mut v: Vec<u32> = vec![];
    
    for item in string.chars() { 
        v.push(item as u32);
    }

    return v;
}

pub fn split_to_compartements(string: String) -> (Vec<u32>,Vec<u32>) { 
    let mut compartement_1: Vec<u32> = vec![];
    let mut compartement_2: Vec<u32> = vec![];

    let length = string.len();

    for (i,item) in string.chars().enumerate() {
        if i<length/2 { 
            compartement_1.push(item as u32);
        }
        else {
            compartement_2.push(item as u32);
        }
    }

    return (compartement_1, compartement_2);
}

pub fn clean_compartement(compartement:&mut Vec<u32>) -> () {
    for (i,item) in compartement.clone().into_iter().enumerate() {
        for j in i+1..compartement.len() { 
            if item == compartement[j] { 
                compartement[j] = 0;
            }
            else { 
                continue;
            }
        }
    }

    compartement.retain(|x| *x!=0);
}
