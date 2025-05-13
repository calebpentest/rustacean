fn main(){
get_result("Caleb");
human_id("Caleb", 24, 177.0);
let _x: i32 = {
    let price: i32 = 5;
    let qty: i32 = 10;
    price * qty
};

println!("Result is: {}", _x);
let y: i32 = add(4,8);
println!("Value of y is {}", y);

let weight: f64 = 70.0;
let height: f64 = 1.82;
let bmi = calculate_bmi(weight, height);
println!("Your bmi is {:.2}", bmi)
}

fn get_result(name: &str){
    println!("hello {}", name);
}

fn human_id(name: &str, age: i32, height: f64){
    println!("My name is {}, I turned {} and I'm {} cm", name, age, height);
}

fn add(a:i32, b:i32) -> i32{
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}

//expression and statement

