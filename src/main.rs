use std::io;

fn main() {
    // Mutable variables' values can be changed later
    let mut choice = String::new();
    let mut temperature = String::new();

    loop {
        //Clear string variables
        choice.clear();
        temperature.clear();
        //Menu
        println!("===========SIMPLE TEMPERATURE CONVERTER===========");
        println!("Press 1 to convert From Fahrenheit to Celsius");
        println!("Press 2 to convert from Celsius to Fahrenheit");
        println!("Press 0 to Exit");
        println!("==================================================");

        println!("Please select a converter:");
        //Read input from the user
        io::stdin()
            .read_line(&mut choice)
            .expect("Selection failed!");

        //trim any spaces from the input variable with trim()
        if choice.trim() == "1" || choice.trim() == "2" {
            println!("Please enter a value:");
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to the register the value.");

            clear_screen();

            println!(
                "The converted temperature to {}: {}",
                if choice.trim() == "1" {"fahrenheit"} else {"celsius"},
                convert_temp(
                    temperature.trim().parse::<f32>().expect("Not a valid temp"),
                    choice.chars().next().expect("Invalid choice")
                )
            )
        } else if choice.trim() == "0" {
            //Exit the program (Like return 0 but with a code)
            println!("Bye!");
            std::process::exit(200);
        } else {
            clear_screen();
            println!("Please a valid choice...");
        }
    }
}

// convert temp based on choice  --- returns value in floating point
fn convert_temp(temp: f32, choice: char) -> f32 {
    if choice == '1' {
        (temp - 32.0) * (5.0 / 9.0)
    }
    temp * (9.0 / 5.0) + 32.0
}

fn clear_screen() {
    std::process::Command::new("clear").status()
    .expect("Failed to clear screen.")
    .success();
}
