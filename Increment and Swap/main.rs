use std::io; 

fn main() {
    let mut v: Vec<i8> = vec![0];  
    println!("Please enter 5 integers");  
    v.pop(); 
    for _i in 0..5 
    {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<i8>() {
            Ok(i) => v.push(i),   
            Err(..) => println!("this was not an integer: {}", trimmed),
        }; 
    }
    println!("Initial vector: {:?}", v);   
    let mut temp;
    for i in 0..3
    {
        temp = v[4-i] + 1;  
        v[4-i] = v[i] + 1;  
        v[i] = temp; 
    }
    println!("Final vector: {:?}", v); 
} 
