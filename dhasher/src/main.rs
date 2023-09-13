use dhash;

fn main() {
    let h1 = dhash::from_file("images/test.jpg");
    let h2 = dhash::from_file("images/test-emoji.jpg");

    println!("{:x} x {:x}", h1, h2);
    let d = dhash::calculate_distance(h1, h2);
    println!("distance: {}", d);
}
