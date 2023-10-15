pub fn compress(input: &str) -> (Vec<char>, Vec<usize>) {
    let mut indexes: Vec<usize> = Vec::new();
    let mut chars: Vec<char> = Vec::new();

    input.chars().for_each(|c| {
        if !chars.contains(&c) {
            chars.push(c);
        }

        let index = chars.iter().position(|&x| x == c).expect("Index not found");

        indexes.push(index);
    });

    return (chars, indexes);
}

#[cfg(test)]
mod tests {
    use crate::compress::*;

    #[test]
    fn test_compress() {
        let result = compress("abcab");

        assert_eq!(result.0, vec!['a', 'b', 'c']);
        assert_eq!(result.1, vec![0, 1, 2, 0, 1]);
    }
}
