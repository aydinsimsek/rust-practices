use std::io; 

fn main() {
    let mut v: Vec<f32> = vec![0.0];  
    println!("Please enter the price of the 10 items in your shopping cart");  
    v.pop(); 
    for _i in 0..10 
    {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<f32>() {
            Ok(f) => v.push(f),   
            Err(..) => println!("this was not a float: {}", trimmed),
        }; 
    }
    
    let mut greatest: f32 = 0.0; 
    for i in 0..10
    {
        if v[i] > greatest
        {
            greatest = v[i]; 
        } 
    }
    println!("The price of the most expensive item in your cart = ${}", greatest);  
    let mut sum: f32 = 0.0; 
    for i in 0..10
    {
        sum += v[i]; 
    }
    println!("The total price of your items before discount = ${}", sum); 
    println!("The total price of your items after discount = ${}", sum - discount(greatest));  
}

fn discount(item: f32) -> f32  
{
    let result: f32; 
    result = item * 0.1;  
    result
} 
