use std::io;

fn main() {
     println!("Please enter the temperature in Fahrenheit"); 
 
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<f32>() {
            Ok(f) => println!("{}°F = {}°C", f, (((f-32.0)*5.0)/9.0)),
            Err(..) => println!("this was not an integer: {}", trimmed),
        };    
}
