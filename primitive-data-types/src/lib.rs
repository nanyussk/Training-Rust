pub fn data_types() -> (u8, f64, bool, char) {
    // 1. Define variable of type `u8` and value `42`
    let type1: u8 = 42;
    // 2. Define variable of type `f64` and value `3.14`
    let type2: f64 = 3.14;
    // 3. Define variable of type `bool` and value `false`
    let type3: bool = false;
    // 4. Define variable of type `char` and value `a`
    let type4: char = 'a';
    // 5. Return a tuple with the variables in the order they were defined
    return (type1, type2, type3, type4);
}
