fn main(){
    // let numbers: [i32; 5] = [1,2,3,4,5];
    // println!("Number array: {:?}", numbers);
    // let fruits: [&str; 3] = ["apple", "banana", "orange"];
    // println!("Fruits {:?}", fruits);


    //tuples

    let human: (String,i32, bool)  = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    let mix_tuple = ("kratos", 23, true, [1,2,3,4,5]);
    println!("Mix tuple: {:?}", mix_tuple);

    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["lion", "elephant", "crocodile"];
    println!("Animal: {:?}", animal_slices);

    let mut stone_cold: String = String::from("Hell ", );
    stone_cold.push_str("yeah!");
    println!("Stone cold says: {}", stone_cold);

    let string: String = String::from("hello, world");
    let slice: &str = &string[0..5];
    println!("Slice value: {}", slice);
}