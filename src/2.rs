// Function to generate a random number between 1 and 10
fn get_random_number() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=10) as u8
}

// Function to check if a number is prime
fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Driver function to test the above functions
fn main() {
    let number = get_random_number();
    println!("Generated random number: {}", number);
    let is_prime = is_prime(number as i32);
    println!("Is prime: {}", is_prime);
}
