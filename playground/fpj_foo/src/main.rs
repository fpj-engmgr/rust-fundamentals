fn main() {
    let mut idx = 1;
    // continue looping until idx > 5
    loop {
        println!(" idx is {}", idx);
        idx += 1;
        if idx > 5 {
            break;
        }
    }
}
