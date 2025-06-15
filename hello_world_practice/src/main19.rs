fn main(){
    let score = 85;

    if score >= 90{
        let result = format!("Excellent score: {}", score);
        println!("{}", result);
    } else if score >= 70 &&  score <90 {
        let result = format!("Good score: {}", score);
        println!("{}", result);
    } else {
        let result = format!("Needs improvement: {}", score);
        println!("{}", result);
    }
}