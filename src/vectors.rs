

#[allow(dead_code)]
pub fn vectors() {
    let _v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let _third: &i32 = &v[2];

    // Panics
    let _does_not_exist = &v[100]; 
    // Returns: None
    let _does_not_exist = v.get(100);

    match v.get(2) {
        Some(third_element) => println!("The third element is {}", third_element),
        None => println!("There is no third element"),
    }
}

#[allow(dead_code)]
pub fn vector_iteration() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        // Dereference + add 50 to position i;
        *i += 50;
    }

    println!("v: {:?}", v)
}