use std::io;

//TODO: Make a drawing guessing game!

mod dictionary;
use dictionary::prelude::*;
use unicode_segmentation::UnicodeSegmentation as US;

const ENGLISH_DIR: &str = "res/dictionaries/english/";
const JAPANESE_DIR: &str = "res/dictionaries/japanese/";
const SPANISH_DIR: &str = "res/dictionaries/spanish/";
const FRENCH_DIR: &str = "res/dictionaries/french/";

#[derive(PartialEq)]
enum GameState {
	Running,
	Stopped,
}

fn main() {
	let dictionary = match std::env::args().nth(1) {
		Some(arg) => arg,
		None => "default.txt".to_string(),
	};

	let language = match std::env::args().nth(2) {
		Some(arg) => arg,
		None => "english".to_string(),
	};

	let directory = match language.as_str() {
		"english" => ENGLISH_DIR.to_string(),
		"japanese" => JAPANESE_DIR.to_string(),
		"spanish" => SPANISH_DIR.to_string(),
		"french" => FRENCH_DIR.to_string(),
		_ => {
			//Interpret language as a directory!
			language
		}
	};

	let mut d = Dictionary::new();
	d.load(&format!("{}{}", directory, dictionary))
		.expect("Could not find file!");
	if d.is_empty() {
		panic!("File {} is empty!", dictionary);
	}
	let mut  word_to_guess = d.get_random_word();

	let mut guess = String::new();

	//Fill the guess with underscores to start.
	{
		let __word_to_guess = word_to_guess.clone();
		let graphemes = 
		US::graphemes(__word_to_guess.as_str(), true)
			.collect::<Vec<&str>>();
		for _ in &graphemes {
			guess.push('_');
		}
	}

	println!("Enter \"quit\" to exit the progam.");
	let mut guesses = 12;
	let mut state = GameState::Running;
	while state != GameState::Stopped {
		println!("{}", guess);
		println!("Enter a character to guess. Guesses left {}.", guesses);

		let mut input = String::new();
		io::stdin().read_line(&mut input)
			.expect("Failed to read input!");

		input = input.trim().to_string();

		if input.len() > 1 {
			match input.as_str() {
				"quit" => state = GameState::Stopped,
				_ => continue,
			}
		}
		//Converting a string into graphemes takes ownership of the string.
		//So to circumvent this I cloned the strings into seperate variables.
		//Im not sure if this is going againt rust guidlines at what is best,
		//but it works.
		//This blocks only purpose is to mark the code as questionable, nothing else.
		{
			let __word_to_guess = word_to_guess.clone();
			let __guess = guess.clone();
			let mut graphemes_i =
			US::graphemes(__word_to_guess.as_str(), true)
				.collect::<Vec<&str>>();

			let mut graphemes_j =
			US::graphemes(__guess.as_str(), true)
				.collect::<Vec<&str>>();

			let mut index = graphemes_i.iter().position(|&x| x == input);

			if index.is_none() {
				guesses -= 1;
			}

			while let Some(i) = index {
				graphemes_j[i] = &input;
				graphemes_i[i] = "_";
				index = graphemes_i.iter().position(|&x| x == input);
			}

			guess.clear();

			for string in &graphemes_j {
				guess.push_str(*string);
			}
		}
		if guesses <= 0 {
			println!("The word was {}.", word_to_guess);
			state = GameState::Stopped;
		}
		if guess == word_to_guess {
			println!("You guessed the word!");
			state = GameState::Stopped;
		}
	}
}
