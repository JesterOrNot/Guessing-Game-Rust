use rand::Rng;
fn main() {
    println!("Guess A Number!");
    let secret_key = rand::thread_rng().gen_range(1, 101);
    print!("What is your guess?: ");
    let mut guess1 = String::new();
    std::io::stdin()
        .read_line(&mut guess1)
        .expect("Please Enter an int");
    let guess: i32 = guess1.trim().parse().unwrap();
    if guess > secret_key {
        println!("Too High!")
    } else if guess < secret_key {
        println!("Too Low!");
    } else if guess == secret_key {
        println!("You Win!");
    }
}
