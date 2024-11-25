fn main() {
    println!("Hello, world!");
    // set uninitialized variable enigma
    let enigma:i32;

    // Error
    // println!("{}", enigma); // Error, enigma is not initialized yet!

    // Error
    // if true {
    //     enigma = 42;
    // }
    // println!("{}", enigma); //error, value of true is unknown at compile

    // No Error
    if true {
        enigma = 42;
    } else {
        enigma =7;
    }

    // prints 42 without error because enigma will be guaranteed to be initialized here
    println!("{}", enigma);
}
