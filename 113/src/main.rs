fn reverse2(s: &str) -> String {
    let x: String = String::from(s);
    fn inner_loop(index: usize) -> String {
        let mut next = index + 1;
        match x.find(' ') {
            Some(i) =>
              return x[index..i]
            None =>
              return String::new()
        }
    }
    return inner_loop(0);
}

fn reverse1(s: &str) -> String {
    let mut o = String::new();
    let splited = s.rsplit(" ");
    splited.for_each(|word| {
        o.push_str(word);
        o.push(' ');
    });
    return o;
}

fn main() {
    println!("{}", reverse1("Hello world here!"));
}
