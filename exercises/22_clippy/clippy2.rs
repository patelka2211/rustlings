fn main() {
    let mut res = 42;
    let option = Some(12);

    if let Some(value) = option {
        res += value;
    }

    println!("{res}");
}
