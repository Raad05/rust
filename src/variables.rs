pub fn variables() {
    let x: i32 = 5;

    // variable scopes
    {
        let y: i32 = 10;

        // printing values
        println!("x = {} || y = {}", x, y);
    }

    // checks if they are equal
    assert_eq!(x, 5);
}
