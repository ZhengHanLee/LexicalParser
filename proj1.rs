use std::io::Write;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::str::SplitWhitespace;
use std::fs::File;

//Project 1 Rust
//Created by Johan ZhengHan Lee

//Struct to store the word and part of speech
struct Word {
   word: String,
   part_of_speech: String,
}

//Main Function, opens all the file arguments, parses the input and writes to an output file
fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 
    {
        eprintln!("Usage: {} <input_file> <parts_of_speech_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let parts_of_speech_file = &args[2];
    let output_file = &args[3];
    //Open input file and create output file
    let input = BufReader::new(File::open(input_file)?);
    let mut output = File::create(output_file)?;
    //Create a list to store word structs and open file
    let mut word_list: Vec<Word> = Vec::new();
    let word_file = BufReader::new(File::open(parts_of_speech_file)?);
    //Read word file line by line and Store words in structs with correct information,
    for line in word_file.lines() 
    {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 2 
        {
            let word_struct = Word 
            {
                word: tokens[0].to_string(),
                part_of_speech: tokens[1].to_string(),
            };
         // println!("Word: {}", word_struct.word);
         // println!("Part of Speech: {}", word_struct.part_of_speech);
            word_list.push(word_struct);
        }
    }

   //Read input file line by line
    for line in input.lines() 
    {
        let line = line?;
        //Split into tokens
        let mut tokens = line.split_whitespace();
        
        //Check for invalid tokens, store words into a list to parse for grammar
        let check_tokens = tokens.clone();
        let mut invalid_token = false;
        for check_token in check_tokens 
        {
            if word_list.iter().any(|word| word.word == check_token) 
            {
                //Token is valid
            } else {
                invalid_token = true
            }
        }
        // Initialize variables for error handling
        let mut token: Option<&str> = tokens.next();  // Initialize the token

        let mut results = String::new();
        let is_valid_sentence = valid_sentence(&mut tokens, &mut token, &mut results, &word_list);

        // Write the formatted line to the output file
        let formatted_line = format!("input-line => {}\n", line);
        output.write_all(formatted_line.as_bytes())?;

        if invalid_token
        {
            writeln!(output, "\tInput has invalid tokens.")?;
        } else if is_valid_sentence
        {
            writeln!(output, "\t{}", results)?;
        } else {
            writeln!(output, "\tInput is not a sentence.")?;
        }
    }

  Ok(())
}

fn valid_sentence<'a>( tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word] ) -> bool
{
    if !token.is_none()
    {
        *results = results.to_owned() + &String::from("("); 
        let subject_result = is_subject(tokens, token, results, word_list);
        *results = results.to_owned() + &String::from(" ");
        let verb_result = verb_phrase(tokens, token, results, word_list);
        *results = results.to_owned() + &String::from(" ");
        let object_result = is_object(tokens, token, results, word_list);

        if subject_result && verb_result && object_result 
        {
            *results = results.to_owned() + &String::from(")");
            return true;
        }
    }
    return false;
}


fn is_subject<'a>( tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word] ) -> bool
{
    if !token.is_none()
    {
        *results = results.to_owned() + &String::from("(");
        if noun_phrase(tokens, token, results, word_list)
        {
            *results = results.to_owned() + &String::from(")");
            return true;
        }
    }
    return false;
}


fn noun_phrase<'a>( tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word] ) -> bool
{
    if !token.is_none()
    {
        *results = results.to_owned() + &String::from("(");
        //Check for optional adjective phrase
        adj_phrase(tokens, token, results, word_list);
        //Check for the noun
        if a_noun(&token.unwrap(), word_list)
        {
            *results = results.to_owned() + &token.unwrap();
            *token = tokens.next();
            //Check for the prepositional phrase
            if !token.is_none() && a_preposition(&token.unwrap(), word_list) {
                if prep_phrase(tokens, token, results, word_list) {
                    *results = results.to_owned() + &String::from(")");
                }
            }
            *results = results.to_owned() + &String::from(")");
            return true;
        }
    }
    return false;
}




fn adj_phrase<'a>( tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word] ) -> bool
{
    let mut result = false;
    while !token.is_none() && an_adj(&token.unwrap(), word_list) {
        *results = results.to_owned() + &String::from("(");
        *results += &token.unwrap();
        *token = tokens.next(); 

        // Check if there are more adjectives in the sequence
        if !token.is_none() && an_adj(&token.unwrap(), word_list) {
            *results = results.to_owned() + &String::from("(");
            *results += &token.unwrap();
            *token = tokens.next(); // Move to the next token
            *results = results.to_owned() + &String::from(")");
        }
        *results = results.to_owned() + &String::from(")");
        result = true;
    }
    return result;
}

fn verb_phrase<'a>(
    tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word]) -> bool {
    let mut result = false;
    if !token.is_none() {
        if a_verb(&token.unwrap(), word_list) {
            *results = results.to_owned() + &String::from("(");
            *results = results.to_owned() + &token.unwrap();
            *token = tokens.next(); 
            // Check for an optional adverb
            if !token.is_none() && an_adverb(&token.unwrap(), word_list)
            {
                *results += " "; 
                *results = results.to_owned() + &token.unwrap();
                *token = tokens.next(); 
            }
            *results = results.to_owned() + &String::from(")");
            result = true;
        } 
    } 
    return result
}



fn prep_phrase<'a>( tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word]) -> bool
{
    let mut result = false;
    if !token.is_none()
    {
        if a_preposition(&token.unwrap(), word_list)
        {
            *results = results.to_owned() + &String::from("(");
            *results = results.to_owned() + &token.unwrap();
            *token = tokens.next();
            if !token.is_none() && noun_phrase(tokens, token, results, word_list)
            {
                result = true;
            }
        }
    }
    return result;
}

fn is_object<'a>(tokens: &mut SplitWhitespace<'a>, token: &mut Option<&'a str>, results: &mut String, word_list: &[Word]) -> bool {
    *results = results.to_owned() + &String::from("(");
    if noun_phrase(tokens, token, results, word_list)
    {
        *results = results.to_owned() + &String::from(")");
        return true;
    }
    return false;
}



//Basic functions
fn an_adj(token: &str, word_list: &[Word]) -> bool 
{
   if word_list.iter().any(|word| word.word == token && word.part_of_speech == "adjective") {
      true
  } else {
      false
  }
}

fn a_noun(token: &str, word_list: &[Word]) -> bool 
{
   if word_list.iter().any(|word| word.word == token && word.part_of_speech == "noun") {
      true
  } else {
      false
  }
}

fn a_verb(token: &str, word_list: &[Word]) -> bool 
{
   if word_list.iter().any(|word| word.word == token && word.part_of_speech == "verb") {
      true
  } else {
      false
  }
}

fn a_preposition(token: &str, word_list: &[Word]) -> bool 
{
   if word_list.iter().any(|word| word.word == token && word.part_of_speech == "preposition") {
      true
  } else {
      false
  }
}

fn an_adverb(token: &str, word_list: &[Word]) -> bool 
{
   if word_list.iter().any(|word| word.word == token && word.part_of_speech == "adverb") {
      true
  } else {
      false
  }
}