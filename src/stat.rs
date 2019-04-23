use std::collections::HashMap;

pub fn mean(entries : &mut Vec<i32>) -> i32 {
    let mut sum = 0 as i32;
    let len = entries.len() as i32;
    for entry in entries {
        sum += *entry;
    }
    sum / len
}

pub fn median(entries : &mut Vec<i32>) -> i32 {
    let med_index = entries.len() / 2;
    entries.sort();
    entries[med_index]
}

pub fn mode (entries: &mut Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for e in entries {
        let val = *e;
        let entry = map.entry(val).or_insert(0);
        *entry += 1;
    }
    map
}

fn main() {
    let mut my_vec = vec![1,1,1,1,2,1,1,1,1,1];
    let m = mean(&mut my_vec);
    let med = median(&mut my_vec);
    let mode = mode(&mut my_vec);
    println!("Mean is {}", m);
    println!("Median is {}", med);
    println!("Mode is {:?}", mode);
}
