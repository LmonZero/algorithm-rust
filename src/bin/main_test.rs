fn main() {
    let c: char = '9';
    let mut t: usize = 0;
    match c {
        '0'..='9' => t = 10 * t + c as usize - 48,
        _ => {}
    }
    println!("{},{:#X}", t, 48);
}
