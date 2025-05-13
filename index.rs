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

//expression and statement

