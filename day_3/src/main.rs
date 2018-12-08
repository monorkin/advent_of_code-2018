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

extern crate regex;
use std::collections::HashSet;

use regex::Regex;
use std::io;

/// The Claim struct represents each Elf's claim to a specific part of the
/// fabric
#[derive(PartialEq, Eq, Hash, Debug)]
struct Claim {
    id: i64,
    x: i64,
    y: i64,
    w: i64,
    h: i64
}

impl Claim {
    /// Builds and returns a fully formed Claim struct
    ///
    pub fn new(id: i64, x: i64, y: i64, w: i64, h: i64) -> Self {
        Self {
            id: id,
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    /// Returns the left bound of the claim
    ///
    pub fn left(&self) -> i64 {
        self.x
    }

    /// Returns the right bound of the claim
    ///
    pub fn right(&self) -> i64 {
        self.x + self.w
    }

    /// returns the top bound of the claim
    ///
    pub fn top(&self) -> i64 {
        self.y
    }

    /// returns the bottom bound of the claim
    ///
    pub fn bottom(&self) -> i64 {
        self.y + self.h
    }

    /// Returns true if the current and the given Claim have any overlap
    ///
    pub fn intersects_with(&self, other: &Self) -> bool {
        self.left() <= other.right()
        && self.right() >= other.left()
        && self.top() <= other.bottom()
        && self.bottom() >= other.top()
    }

    /// Returns the size of the overlap between the current and the given
    /// Claim
    ///
    pub fn intersecting_area(&self, other: &Self) -> i64 {
        if !self.intersects_with(other) {
            return 0;
        }

        let rect = self.intersecting_rect(other);
        rect.area()
    }

    /// Returns a new Claim that represents the intersecting rect of the current
    /// and the input claim with an ID of -1
    ///
    /// E.g.
    /// ............
    /// ...AAAA.....
    /// ...AAXXCC...
    /// ...AAXXCC...
    /// .....CCCC...
    /// ............
    /// In the above situation, for the current Claim A and the other Claim C,
    /// a new Claim representing X would be returned
    ///
    pub fn intersecting_rect(&self, other: &Self) -> Claim {
        let mut h = [
            self.left(), self.right(), other.left(), other.right()
        ];
        let mut v = [
            self.top(), self.bottom(), other.top(), other.bottom()
        ];
        h.sort();
        v.sort();

        let l = h[1];
        let r = h[2];
        let w = r - l;
        let t = v[1];
        let b = v[2];
        let h = b - t;

        Self::new(-1, l, t, w, h)
    }

    /// Returns the area of the claim - width * height
    ///
    pub fn area(&self) -> i64 {
        self.w * self.h
    }
}


fn main() {
    let re = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    let mut claims: Vec<Claim> = vec![];
    let mut overlap_area: i64 = 0;
    let mut visited: HashSet<(&Claim, &Claim)> = HashSet::new();
    let mut intersects: HashSet<Claim> = HashSet::new();

    // Parse the input to Claim structs
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                claims.push(parse_claim(&re, input).unwrap());
            }
            Err(_) => break
        }
    }

    // Find all intersecting structs and store their intersections
    for c1 in &claims {
        for c2 in &claims {
            if c1 != c2
                && c1.intersects_with(c2)
                && !(visited.contains(&(c1, c2)) || visited.contains(&(c2, c1)))
            {
                intersects.insert(c1.intersecting_rect(c2));
                visited.insert((c1, c2));
            }
        }
    }

    let mut visited: HashSet<(&Claim, &Claim)> = HashSet::new();
    let mut visited_nodes: HashSet<&Claim> = HashSet::new();
    let mut new_intersects: Vec<Claim> = vec![];

    // Find all intersections intersecting other intersections
    for c1 in &intersects {
        if visited_nodes.contains(c1) {
            continue;
        }
        for c2 in &intersects {
            if c1 != c2
                && c1.intersects_with(c2)
                && !(visited.contains(&(c1, c2)) || visited.contains(&(c2, c1)))
            {
                new_intersects.push(c1.intersecting_rect(c2));
                visited.insert((c1, c2));
                visited_nodes.insert(c1);
                visited_nodes.insert(c2);
            }
        }
    }

    // Sum all intersecting areas
    for c1 in &intersects {
        overlap_area += c1.area();
    }

    // Deduct all intersecting areas that are intersecting others
    for c2 in &new_intersects {
        overlap_area -= c2.area();
    }

    println!("AREA {}", overlap_area);
}

/// Parses an input string into a Claim struct by the given Regex
///
fn parse_claim(re: &Regex, input: String) -> Result<Claim, ()> {
    match re.captures(&input) {
        Some(cap) => {
            Ok(
                Claim::new(
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                    cap[4].parse().unwrap(),
                    cap[5].parse().unwrap()
                )
            )
        },
        _ => Err(())
    }
}
