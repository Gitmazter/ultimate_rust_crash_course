
const STARTING_LAZORS:i32 = 8;
const READY_COUNT:i32 = 2;

fn main() {
    // let mut lazors:i32 = STARTING_LAZORS;
    // let ready:i32 = READY_COUNT;
    let (lazors, ready):(i32, i32) = (STARTING_LAZORS, READY_COUNT);
    println!("IMMA FIRING {} OF MA {} LAZOOOOOOOOOORS...", ready, lazors);

    // lazors = lazors - ready;

    println!("{} LAZORS REMAIN", lazors - ready);

    let _casualties = "infinity";

    // cannot assign
    // READY_COUNT = 4;
}
