use std::io;

pub fn input_loop<T>(prompt: &str) -> T 
where T: std::str::FromStr,
      <T as std::str::FromStr>::Err: std::fmt::Debug {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(..) => println!("Invalid input"),
        }
    }
}