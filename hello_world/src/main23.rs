//Gemeric Structs

struct Point<T>{
    x: T,
    y: T
}

fn main(){
    let first_point: Point<f64> = Point{
        x: 3.14,
        y: 2.614
    };

    let second_point: Point<u32> = Point{
        x: 25,
        y: 30
    };

    println!("The x: {} and y: {}", &first_point.x, &first_point.y);
    println!("The x: {} and y: {}", &second_point.x, &second_point.y);
}