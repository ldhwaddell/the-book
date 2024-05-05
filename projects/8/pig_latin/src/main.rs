/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
fn main() {
    let sentence = String::from("  Hello My     name is first apple   ");
    println!("Sentence is {:?}", sentence);

    let pig_latin_sentence = pig_latin(&sentence);

    println!("Pig Latin Sentence is {:?}", pig_latin_sentence);
}

fn pig_latin(s: &str) -> String {
    if s.trim().is_empty() {
        return String::new();
    }

    let mut ans = String::new();

    for word in s.split_whitespace() {
        let pig = pig_word(word).to_lowercase();
        ans.push_str(&pig);
    }

    ans
}

fn pig_word(s: &str) -> String {
    let (first, rest) = s.split_at(1);

    if is_vowel(first.chars().next().unwrap()) {
        format!("{s}-hay ")
    } else {
        format!("{rest}-{first}ay ")
    }
}

fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}
