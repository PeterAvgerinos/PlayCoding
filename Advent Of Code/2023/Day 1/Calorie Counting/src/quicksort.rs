pub fn quick_sort(vector: &mut Vec<i32>) { 
    let len = vector.len();
    _quick_sort(vector, 0, (len-1) as isize);
}

fn _quick_sort(vector: &mut Vec<i32>, low: isize, high: isize){ 
    if low < high { 
        let p = partition(vector, low, high);
        _quick_sort(vector, low, p-1);
        _quick_sort(vector, p+1, high);
    }
}

fn partition(vector: &mut Vec<i32>, low: isize, high: isize) -> isize { 
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
