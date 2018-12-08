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
use std::collections::HashMap;

fn main() {
    let mut twos = 0;
    let mut threes = 0;
    let mut strings: Vec<String> = vec![];

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                strings.push(input.clone());

                let letters_count = count_letters(input);
                let mut twos_counted = false;
                let mut threes_counted = false;
                for (_key, val) in letters_count.iter() {
                    let val = val.to_owned();
                    if !twos_counted && val == 2 {
                        twos += 1;
                        twos_counted = true;
                    }
                    if !threes_counted && val == 3 {
                        threes += 1;
                        threes_counted = true;
                    }
                }
            }
            Err(_) => break
        }
    }

    println!("TWOS {} THREES {} CHECKSUM {}", twos, threes, twos * threes);

    strings.sort();

    let mut i = 0;
    loop {
        if i + 1 >= strings.len() {
            println!("NO MATCH!");
            break;
        }

        let mut a = strings[i].split("").collect::<Vec<&str>>();
        let mut b = strings[i + 1].split("").collect::<Vec<&str>>();
        let mut c = vec![];

        let mut j = 0;
        loop {
            if j >= a.len() || j >= b.len() {
                break;
            }

            if a[j] == b[j] {
                c.push(a[j]);
            }

            j += 1;
        }

        if a.len() - c.len() == 1 {
            println!(
                "MATCH!\n{:?}\n{:?}\nCOMMON {:?}",
                a.into_iter().collect::<String>(),
                b.into_iter().collect::<String>(),
                c.into_iter().collect::<String>()
            );
            break;
        }

        i += 1;
    }
}

fn count_letters(input: String) -> HashMap<char, i64> {
    let mut store = HashMap::new();
    let mut input = input.clone();
    input.pop();

    loop {
        let l = input.pop();

        if l.is_none() {
            break;
        }

        let l = l.unwrap();

        let c = match store.get(&l) {
            Some(i) => i + 1,
            _ => 1
        };

        store.insert(l, c);
    }

    store
}
