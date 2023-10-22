fn main() {
    // Mutable
    let mut x = 5;
    println!("The value of mutable x is: {x}");
    x = 6;
    println!("The value of mutable x is: {x}");

    // Immutable
    let y = 13;
    println!("The value of immutable y is: {y}");

    // Constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Number of seconds in three hours: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let z = 3;
    let z = z + 2;

    {
        let z = z * 2;
        println!("The value of z in inner scop: {z}"); // 10
    }

    println!("The value of z: {z}"); // 5

    /* This example does not compile: mismatched types
    let mut spaces = "     ";
    println!("Original spaces value: {spaces}");
    spaces = spaces.len();
    println!("New shadowed spaces value: {spaces}");
    */
}
