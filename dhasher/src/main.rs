use dhash;

fn main() {
    let h1 = dhash::from_file("test.jpg");
    let h2 = dhash::from_file("test-emoji.jpg");

    println!("{:b} - {:b}", h1, h2);
    let d = dhash::calculate_distance(h1, h2);
    println!("d: {}", d);
}
