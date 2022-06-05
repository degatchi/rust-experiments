#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[allow(dead_code)]

pub fn hashmap_a() {
    // let mut pairs = BTreeMap::new();
    let mut pairs = HashMap::new();

    // Store initial key-value pair.
    pairs.insert(("a", "a"), (1, 1));

    // Replace key's value.
    pairs.insert(("a", "a"), (2, 2));

    // Index key directly.
    // If no value at key, it panics.
    let coords = ("a", "a");
    let _pair = pairs[&coords];

    // Another way of indexing directly.
    let pair_a = pairs[&("a", "a")];

    // Get value from key, except it doesn't panic - returns Option.
    let pair_b = pairs.get(&("a", "a"));
    let pair_c = pairs.get(&("a", "b"));

    println!("pair_a: {:?}", pair_a);
    println!("pair_b: {:?}", pair_b);
    println!("pair_c: {:?}", pair_c);

    // If key exists, replace value.
    // Otherwise, insert new key-value pair.
    pairs.entry(("a", "a")).or_insert((2, 2));
    // Another way of replacing.
    *pairs.entry(("a", "a")).or_insert((2, 2)) = (1, 1);

    // Searches for key and removes if it's found. 
    pairs.insert(("b", "b"), (1, 1));
    pairs.remove(&("b", "b"));

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

pub fn hashset() {
    let mut primes = HashSet::new();

    primes.insert(2);
    primes.insert(3);
    primes.insert(5);
    
    for prime in &primes {
        println!("Prime: {}", prime);
    }
}

pub fn map_in_map() {
    let mut pairs: HashMap<(&str, &str), HashMap<i32, (i32, i32)>> = HashMap::new(); 
    let mut reserves: HashMap<i32, (i32, i32)> = HashMap::new();

    reserves.insert(0, (100, 150));
    reserves.insert(1, (100, 160));
    reserves.insert(2, (100, 170));

    pairs.insert(("a", "b"), reserves); 

    /*
        Pairs: {
            ("a", "b"): {
                "2": (100, 170), 
                "0": (100, 150), 
                "1": (100, 160)
            }
        }
     */
    println!("Pairs: {:?}", pairs);

    // Update inner mapping
    if let Some(group) = pairs.get(&("a", "b")) {
        // len() == 3
        println!("Group len: {:?}", group.len());

        
        /*
            pair 0: ("1", (100, 160))
            pair 1: ("0", (100, 150))
            pair 2: ("2", (100, 170))
        */
        for (index, pair) in group.iter().enumerate() {
            println!("pair {}: {:?}", index, pair);
        }
    }

    // Updating hashmap value located within hashmap.
    let val = pairs[&("a", "b")][&0];
    println!("val before: {:?}", val);
    if let Some(changing) = pairs.get_mut(&("a", "b")) {
        changing.insert(0, (110, 140));
        println!("val after: {:?}", pairs[&("a", "b")][&0]);
        println!("new pairs: {:?}", pairs.get(&("a", "b")));
    }

    
}