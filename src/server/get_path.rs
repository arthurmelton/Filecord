pub fn get_path(s: String) -> Option<String> {
    Some(s
        .split(" ")
        .nth(1)?
        .chars()
        .collect::<Vec<char>>()[1..]
        .into_iter()
        .collect::<String>()
        .trim()
        .to_string())
}