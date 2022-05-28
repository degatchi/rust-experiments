#[allow(dead_code)]

#[derive(Debug)]
pub struct Something {
    pub a: String,
    pub b: i8,
}

pub fn lifetime_loop() {
    let mut count = 0;

    'outer: loop {
        count += 1;

        loop {
            if count == 1 {
                // Break out of the 'outer loop.
                // Results in termination of loop.
                break 'outer
            }
        }
    }
}

pub fn for_loops() {
    // Loop ends on 9th iteration.
    for _i in 0..10 {
        // println!("loop 1: {}", i);
    }
    
    // Loop includes 10th iteration.
    for _i in 0..=10 {
        // println!("loop 2: {}", i);
    }

    let some: Vec<Something> = vec![];

    // Iterates over the vector elements.
    for _something in some.iter() {
        // println!("Something: {:?}", something)
    }

    // Iterates over the vector of elements while keeping track of the counter.
    for _something in some.iter().enumerate() {
        // println!("Something: {:?}", something)
    }

    // Iterates over the vector of elements while keeping track of the counter.
    for (_index, _something) in some.iter().enumerate() {
        // println!("Something: {:?}", something)
    }

    // Reverses the order of iteration.
    for _something in some.iter().rev() {
        // println!("Something: {:?}", something)
    }
}