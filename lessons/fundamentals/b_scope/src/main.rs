fn main() {
    println!("Hello, world!");

    // scope
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y); 
    }
    // println!("{}, {}", x, y); // ERROR! y is not accessible here as it is out of scope

    // shadowing 
    let z = 69;
    {
        let z = 420;
        // z = 420 here
        println!("{}", z);
    }
    // z = 69 here
    println!("{}", z);


    // shadowing in the same scope
    let mut xy = 5; // xy is mutable
    let xy = xy; // xy is not immutable


    // shadowing can even change the type of a variable
    let meme = "Looby";
    let meme = make_image(meme);

}
