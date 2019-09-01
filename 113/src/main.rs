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
