fn main() {
    let numbers = vec![5, 10, 15, 20];

    let mut iters = numbers.iter();

    let mut sum = 0;

    for element in &numbers {
        sum += element;
    }

    let result = format!(
        "{} + {} + {} + {} = {}",
        iters.next().unwrap(),
        iters.next().unwrap(),
        iters.next().unwrap(),
        iters.next().unwrap(),
        sum
    );

    println!("{}", result);
}
