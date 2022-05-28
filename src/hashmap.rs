use std::collections::HashMap;

pub fn hashmap_a() {
    let mut pairs = HashMap::new();

    // Store initial key-value pair.
    pairs.insert(("a", "a"), (1, 1));

    // Replace key's value.
    pairs.insert(("a", "a"), (2, 2));

    // Get value from key.
    let pair_a = pairs[&("a", "a")];
    let pair_b = pairs.get(&("a", "a"));
    let pair_c = pairs.get(&("a", "b"));

    println!("pair_a: {:?}", pair_a);
    println!("pair_b: {:?}", pair_b);
    println!("pair_c: {:?}", pair_c);

    // If key exists, don't replace value.
    // Otherwise, create new key-value pair.
    pairs.entry(("a", "a")).or_insert((2, 2));

    // Iterating over HashMap.
    for (key, value) in &pairs {
        // Key filter.
        if pairs.contains_key(&("a", "a")) {
            println!("key: {:?} : value: {:?}", key, value);
        }
    }

    // Iteration over hashmap and checking val.
    match pairs.get(&("a", "a")) {
        Some(val) => {
            println!("Val found: {:?}", val);
        },
        None => {},
    }
}
