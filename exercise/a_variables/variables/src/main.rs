
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let mut ready = READY_AMOUNT;
    println!("firing {} of my {} misslies...", ready, missiles);
    missiles = missiles -ready;
    println!("{} missiles left", missiles);


}   
