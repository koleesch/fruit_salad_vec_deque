/*
    This code starts with an initial VecDeque,
    Convert it to a Vec for shuffling, and then it converts it back to a VecDeque.
    After that, it pushes "Pomegranate" to the front of the VecDeque and "Fig" and "Cherry"
    to the back of the deque.
    Finally, it prints the VecDeque.

    VecDeque is a double-ended queue implemented with a growable ring buffer.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<&str> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert the fruit back to a VecDeque
    let mut fruit: VecDeque<&str> = fruit.into_iter().collect();

    // Add some more fruit
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    print_fruit_salad(&fruit);

    println!("Insert a fruit in front of the salad: ");
    // Insert a fruit from a user
    let mut fruit_input = String::new();
    std::io::stdin().read_line(&mut fruit_input).unwrap();
    fruit.push_front(&fruit_input);

    println!("Insert a fruit at the end from the salad: ");
    // Insert a fruit from a user
    let mut fruit_input = String::new();
    std::io::stdin().read_line(&mut fruit_input).unwrap();
    fruit.push_back(&fruit_input);

    print_fruit_salad(&fruit);

    let fruit_vec: Vec<&str> = fruit.clone().into_iter().collect();
    println!("A random fruit: {}", fruit_vec.choose(&mut rng).unwrap());

    println!("Shuffling again");
    let mut fruit: Vec<&str> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);
    
    // Convert the fruit back to a VecDeque
    let mut fruit: VecDeque<&str> = fruit.into_iter().collect();
    print_fruit_salad(&fruit);

    if let Some(popped_fruit) = fruit.pop_back() {
        println!("popped back fruit: {:?}", popped_fruit);
    }
    print_fruit_salad(&fruit);
}

fn print_fruit_salad(fruit_salad: &VecDeque<&str>) {
    println!("Fruit Salad:");
    // Print the fruit salad
    for (i, iten) in fruit_salad.iter().enumerate() {
        if i < fruit_salad.len() - 1 {
            print!("{}, ", iten);
        } else {
            println!("{}", iten);
        }
    }
}
