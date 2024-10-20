use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let pictures = ["|
|
|
|
|","|
|
|
|
|----","|---|
|
|
|
|----","|---|
|/
|
|
|----","|---|
|/  □
|
|
|----","|---|
|/  □
|   |
|
|----","|---|
|/  □
|   |
|   /
|----","|---|
|/  □
|   |
|   /\\

|----","|---|
|/  □
|  /|
|   /\\

|----", "|---|
|/  □
|  /|\\
|   /\\
|----"];
    let word = generate_word();
    let mut wrong_guesses: usize = 0;
    let mut guessed_word:String = generate_guessed_word(word.clone());
    let mut correct_letters: Vec<char> = Vec::new();
    let mut incorrect_letters: Vec<char> = Vec::new();

    while (guessed_word != word) & (wrong_guesses < 10){  
        println!("Here is your guessed word so far: {}", guessed_word);
        println!("Here are your incorrect guesses so far: {}", incorrect_letters.iter().map(|x| x.to_string() + ", ").collect::<String>());
        let guessed = get_input();
        if guessed.len() == 1 {
            let guessed_letter = guessed.chars().next().unwrap();
            let iter = word.chars().zip(guessed_word.chars());
            let iter = iter.map(|x| if x.0 == guessed_letter {x.0} else {x.1});
            let new_guessed_word = iter.collect();
            if new_guessed_word == guessed_word {
                if correct_letters.contains(&guessed_letter) | incorrect_letters.contains(&guessed_letter) {
                    println!("You already guessed this letter!");
                } else {
                    incorrect_letters.push(guessed_letter);
                    println!("{}", pictures[wrong_guesses]);
                    wrong_guesses+=1;

                }
            } else {
                correct_letters.push(guessed_letter);
            }
            guessed_word = new_guessed_word;
            
        } else if guessed.len() > 1 {
            if guessed == word {
                guessed_word = word.clone()
            } else {
                println!("That was not the right word!");
                wrong_guesses+=1;
            }
        } else {
            println!("You must guess a word or a letter!")
        }
        println!("")
    }
    if wrong_guesses<10 {
        println!("You have won!");
    } else {
        println!("You have lost");
    }
    println!("The word was {}", word);
}

fn generate_word()-> String {
    let file = File::open("words.txt");
    let mut contents = String::new();
    let _possible_error =file.expect("REASON").read_to_string(&mut contents);
    let words: Vec<&str> = contents.split(",").collect();
    let word = words.choose(&mut rand::thread_rng()).expect("REASON").to_string().trim().to_string();
    

    return word
}

fn generate_guessed_word(word: String)-> String{
    let guessed_word: String = "_".repeat(word.len());
    return guessed_word
}

fn get_input()-> String{

    print!("Please input your guess: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.to_lowercase();
    let input = input.to_string().trim().to_string();
    return input

}

