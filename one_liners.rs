fn longest(a1: &str, a2: &str) -> String {
    return a1.chars() // 1
            .chain(a2.chars()) // 2
            .collect::<HashSet<char>>() // 3
            .into_iter() // 4
            .map(|c| c.to_string()) // 5
            .sorted() // 6
            .join(""); // 7
}
