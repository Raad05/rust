pub fn numbers() {
    // reassigning variable data types
    let z = 10_u8 as u32; // or let z = 10_u8

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 +  255 = 1597
    assert!(v == 1597);
}
