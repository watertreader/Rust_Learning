fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* YOUR CODE GOES HERE */
    let af: f32 = a as f32;               // a as f32 is explictly casting it
    let average = ( af + b + c ) / 3.0;



    assert_eq!(average, 45.1);
    println!("Test passed!");
}
