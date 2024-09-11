pub fn variables() {
    // integar variables
    let x: i32 = 5;

    // variable scopes
    {
        let y: i32 = 10;

        // printing values
        println!("x = {} || y = {}", x, y);
    }

    // checks if they are equal
    assert_eq!(x, 5);

    // string variables
    let my_string: &str = "Rust";

    println!("I am {}!", my_string);

    // mutables
    let mut a: i32 = 20;
    a += 5;
    println!("{}", a);

    // shadowing
    let y: i32 = 10;
    // if datatype is different, we can reassign
    let y: &str = "I am Y";

    println!("Y = {}", y);
    println!("Y = {}", y);

    // destructuring
    let (mut x, y) = (2, 3);
    x += 1;

    assert_eq!(x, 3);
    assert_eq!(y, 3);
}
