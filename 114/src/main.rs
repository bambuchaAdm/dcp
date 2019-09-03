
fn reverse_split(s: &str, delimiters: Vec<char>) -> String {
    let parts = s.rsplit(|c: char| delimiters.contains(&c));
    let mut splits = s.chars().filter(|c: &char| delimiters.contains(c));

    let mut buffer: String = String::new();
    for word in parts {
        buffer.push_str(word);
        for delimiter in splits.next() {
            buffer.push(delimiter)
        }
    }
    return buffer;
}

fn main() {
    println!("{}", reverse_split("Hello, world!", [',', ' '].to_vec()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(reverse_split("hello/world:here", ['/', ':'].to_vec()), "here/world:hello")
    }

    #[test]
    fn second_example() {
        assert_eq!(reverse_split("hello/world:here/", ['/', ':'].to_vec()), "/here:world/hello")
    }

    #[test]
    fn third_example() {
        assert_eq!(reverse_split("hello//world:here", ['/', ':'].to_vec()), "here/world/:hello")
    }
}