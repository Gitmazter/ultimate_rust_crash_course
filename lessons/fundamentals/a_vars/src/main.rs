
// Constants are super fast and 100% immutable
// have to be written in "screaming snake case"
const _BUNNY_SIZE:i32 = 32;

fn main() {
    // Prints the string to terminal
    println!("Hello, world!");
    
    // Setting a mutable variable "bunnies"
    let mut _bunnies: i32 = 2;
    // updating the mutable variable
    _bunnies = 5; // error

    // destructuring variables from a tuple
    let (_dogs, _sausages) = (8, 50);
}
