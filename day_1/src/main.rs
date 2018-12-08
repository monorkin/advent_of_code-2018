//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.


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
