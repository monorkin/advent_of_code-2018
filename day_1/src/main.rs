use std::io;
use std::collections::HashSet;

fn main() {
    let mut numbers: Vec<i64> = vec![];
    let mut total: i64 = 0;
    let mut duplicate: Option<i64> = None;
    let mut totals: HashSet<i64> = HashSet::new();

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                input.pop();
                let v: i64 = input.parse().unwrap();
                numbers.push(v);
            }
            Err(_) => break
        }
    }

    let mut i = 0;
    let mut summed = false;
    let mut current_total: i64 = 0;

    loop {
        if !summed && i >= numbers.len() {
            summed = true;
            total = current_total;
        }
        if summed && duplicate.is_some() {
            break;
        }
        if i >= numbers.len() {
            i = 0;
        }

        let n = numbers[i];
        current_total += n;

        if duplicate.is_none() && totals.contains(&current_total) {
            duplicate = Some(current_total);
        }
        totals.insert(current_total);

        i += 1;
    }

    println!("TOTAL: {} DUPLICATE: {}", total, duplicate.unwrap());
}
