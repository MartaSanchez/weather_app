use std::io;
use std::env;
use dotenv::dotenv;

mod weather;
use weather::*;

fn main() {
    println!("Welcome to the Marta weather app!" );
    println!("Let's see if we need ☂️ today!" );

    // Load .env file
    dotenv().ok();

    // Get API key from environment variable
    let api_key = env::var("OPENWEATHERMAP_API_KEY")
        .expect("OPENWEATHERMAP_API_KEY must be set in .env file");

    loop {
        println!("Please enter the name of the city:"); 

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input"); 
        let city = city.trim();

        println!("Please enter the country code:"); 

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input"); 
        let country_code = country_code.trim();

        match get_weather_info(&city, &country_code, &api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!("Do you want to search for weather in another city? (yes/no):"); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input"); 
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}
