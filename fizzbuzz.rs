fn main()
{
    let mut num = 1; 

    while num <= 100 
    {
        if num % 15 == 0 {println!("FizzBuzz");}
        else if num % 3 == 0 {println!("Fizz");} 
        else if num % 5 == 0 {println!("Buzz");} 
        else {println!("{}", num);} 
        num += 1; 
    }
}
