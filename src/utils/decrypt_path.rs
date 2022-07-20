pub fn decrypt_path(path: String) -> Option<[u64; 2]> {
    let char_list: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();
    let mut index = 0;
    let mut returns = [0 as u64; 2];
    for x in 0..2 {
        let mut num: u64 = 0;
        for y in 0..11 {
            let char = path.chars().nth(index)?;
            num += char_list.iter().position(|c| *c == char)? as u64 * (62_u64.pow(y));
            index += 1;
        }
        returns[x as usize] = num;
    }
    Some(returns)
}
