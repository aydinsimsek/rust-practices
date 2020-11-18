use std::io; 

fn main() { 
    let mut v: Vec<f32> = vec![0.0];   
    println!("Please enter the exam results");  
    v.pop(); 
    for _i in 0..3 
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
    
    let score: f32 = (v[0]+v[1]) * 0.3 + v[2] * 0.4;   
    println!("Your score = {}", score); 
    let mut grade: String = "None".to_string();      
    if score >= 90.0 {grade = "AA".to_string();}  
    else if score >= 85.0 && score < 90.0 {grade = "BA".to_string();}
    else if score >= 80.0 && score < 85.0 {grade = "BB".to_string();}
    else if score >= 75.0 && score < 80.0 {grade = "CB".to_string();}
    else if score >= 70.0 && score < 75.0 {grade = "CC".to_string();}
    else if score >= 65.0 && score < 70.0 {grade = "DC".to_string();}
    else if score >= 60.0 && score < 65.0 {grade = "DD".to_string();}
    else if score >= 50.0 && score < 60.0 {grade = "FD".to_string();}
    else if score < 50.0 {grade = "FF".to_string();}
    
    // Following part is added to practice the implementation of the push_str method
    if grade == "AA" || grade == "BA" {grade.push_str(". Fantastic!");} 
    else if grade == "BB" || grade == "CB" {grade.push_str(". Good job!");} 
    else if grade == "CC" {grade.push_str(". Not bad!");} 
    else if grade == "DC" || grade == "DD" {grade.push_str(". At least you passed the course!");}  
    else if grade == "FD" || grade == "FF" {grade.push_str(". Don't worry you'll bounce back!");}  
    
    println!("Your letter grade is {}", grade); 
}
