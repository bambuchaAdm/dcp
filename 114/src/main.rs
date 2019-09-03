
fn reverseSplit(s: &str, delimiters: Vec<char>) -> String {
    let parts = s.rsplit(|c: char| delimiters.contains(&c));
    let mut splits = s.chars().filter(|c: &char| delimiters.contains(c));

    let mut buffer: String = String::new();

    for word in parts {
        print!("{}", word);
        buffer.push_str(word);
        splits.next().map(|c: char| buffer.push(c));
    }
    return buffer;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(reverseSplit("hello/world:here", ['/', ':'].to_vec()), "here/world:hello")
    }

    #[test]
    fn second_example() {
        assert_eq!(reverseSplit("hello/world:here/",['/', ':'].to_vec()), "/here:world/hello")
    }

    #[test]
    fn third_example() {
        assert_eq!(reverseSplit("hello//world:here", ['/', ':'].to_vec()), "here/world/:hello")
    }
}