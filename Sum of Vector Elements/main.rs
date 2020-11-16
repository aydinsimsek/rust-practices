use std::io;

fn main() {
    let mut v: Vec<u32> = vec![0];  
    println!("Please enter 5 numbers"); 
    for _i in 0..5
    {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => v.push(i),
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
    }
    println!("Sum of the numbers you entered = {}", v[1]+v[2]+v[3]+v[4]+v[5]);    
}
