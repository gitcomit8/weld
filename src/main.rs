mod generator;
mod wordbank;

use crate::generator::WeldGenerator;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use rand::seq::IndexedRandom;

fn main() {
    //Initialize the engine
    let generator = WeldGenerator::new();
    let term = Term::stdout();

    //Set default state
    let mut word_count: usize = 4;
    let mut separator: String = "-".to_string();
    let mut capitalize: bool = false;

    //Cool exit phrases
    let exit_phrases = [
        "Stay Secure. Goodbye!!",
        "Hack the Gibson.",
        "The grid is yours.",
        "May your passwords be strong.",
        "Follow the white rabbit!",
        "Remember, with great power comes great responsibility.",
    ];

    //Clear screen for a fresh start
    term.clear_screen().unwrap();
    println!("Weld - Secure Passphrase Generator");

    //Main Loop
    loop {
        println!("\n----------------------------------");

        let menu_options = &[
            "0. Generate Passphrase",
            "1. Configure Generation",
            "2. Add Custom Word",
            "3. Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .default(0)
            .items(&menu_options[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                //Generate
                let pass = generator.generate(word_count, &separator, capitalize);
                println!("\n Result: {}", pass);

                let mut rng = rand::rng();
                let quote = exit_phrases.choose(&mut rng).unwrap();
                println!("\n {}", quote);
                break;
            }
            1 => {
                //Configure
                word_count = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Word count")
                    .default(word_count)
                    .interact_text()
                    .unwrap();

                separator = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Separator")
                    .default(separator.clone())
                    .interact_text()
                    .unwrap();

                capitalize = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Capitalize first letters?")
                    .default(capitalize)
                    .interact()
                    .unwrap();

                println!("\n Settings updated.");
            }
            2 => {
                //Add custom word
                let new_word: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter word to add")
                    .interact_text()
                    .unwrap();

                if let Err(e) = generator.add_word(&new_word) {
                    println!("Error adding word: {}", e);
                } else {
                    println!("Added {} successfully to the dictionary.", new_word);
                }
            }
            3 => {
                //Exit
                let mut rng = rand::rng();
                let quote = exit_phrases.choose(&mut rng).unwrap();
                println!("\n {}", quote);
                break;
            }
            _ => unreachable!(),
        }
    }
}
