pub fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
    // TODO: Return a tuple of 4 values: (sum, difference, multiply, divide)
    let sum: i32 = a + b;
    let difference: i32 = a - b;
    let multiply: i32 = a * b;
    let divide: i32 = a / b;

    (sum, difference, multiply, divide)
}
