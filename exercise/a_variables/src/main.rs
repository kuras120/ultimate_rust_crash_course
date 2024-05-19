fn main() {
    const READY_AMOUNT: i32 = 1;
    let (mut missiles, ready): (i32, i32) = (8, 2);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles -= ready;
    println!("{} missiles left", missiles);
}
