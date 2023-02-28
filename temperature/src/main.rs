use std::io;

fn main() {
    println!("Convert a Temperature!");

    let mut scale = String::new();
    let mut temperature = String::new();

    loop {
        println!("Please input the starting temperature scale (F for Fahrenheit or C for Celsius).");

        
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale = scale.trim();
        
        if scale == "C" || scale == "F" { 
            break; 
        }
    }

    loop {
        println!("Please input the starting temperature (Number only).");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if scale.trim() == "C" {
            println!("{temperature}º C is equivalent to {:.0}º F", (temperature * (9.0/5.0)) + 32.0);
        } else {
            println!("{temperature}º F is equivalent to {:.0}º C", (temperature - 32.0) * (5.0/9.0));
        }
        break;
    }


}