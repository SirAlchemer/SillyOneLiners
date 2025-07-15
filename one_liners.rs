
/// Gathers unique characters from two strings, sorts them lexicographically, and then returns them as a whole string. 
fn longest(a1: &str, a2: &str) -> String {
    return a1.chars() // 1
            .chain(a2.chars()) // 2
            .collect::<HashSet<char>>() // 3
            .into_iter() // 4
            .map(|c| c.to_string()) // 5
            .sorted() // 6
            .join(""); // 7
}

/// Checks to see if a valid triangle can be made from 3 numbers of type i64
fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    [a, b, c].iter().all(|&x| x > 0) && a + b > c && a + c > b && b + c > a
}
