fn main() {
    fizzbuzz(
        &vec![
            (3, String::from("Fizz")),
            (5, String::from("Buzz")),
            (7, String::from("Bazz")),
        ],
        (0, 100),
    );
}

fn fizzbuzz(map: &Vec<(i32, String)>, range: (i32, i32)) {
    // Main loop
    for i in (range.0)..(range.1) {
        let mut result = String::new();
        let mut found = false;
        // Looping through tuple
        for tup in map {
            if i % tup.0 == 0 {
                // If i is divisible by any of the first values in the tuple
                found = true; // Then signal that we found a word
                result += tup.1.as_str(); // And add the second value in that tuple to the result
            }
        }
        if !found {
            // If we did not find a word, add the number to the result
            result += i.to_string().as_str();
        }
        println!("{}", result);
    }
}
