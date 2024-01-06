fn main() {
    println!("Hello, world2!");
    let a = ("sdhgsajd", 12);
    let array = [1, 2, 3];
    println!("{}-{}-{}", a.0, a.1, array[1]);
    for item in array.iter() {
        print!("printing {}\n", item)
    }
}
