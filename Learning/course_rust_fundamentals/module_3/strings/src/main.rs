// Basic string maniplation
// slicing, collectning words into a vector, reversing, longest word
fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..4]);

    // concatenate using format!
    // let description = format!("Title: Quick story\n{}", sentence);
    // println!("{}", description);

    // iterate over the characters in the sentence
    // let mut count = 0;
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => count += 1,
    //         _ => continue,
    //     }
    // }
    // println!("The sentence has {} vowels.", count);

    // Split and collect words into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    // Reverse the string
    //let reversed = sentence.chars().rev().collect::<String>();
    //println!("{}", reversed);

    // Find the longest word in a vector of words (strings)
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    let longest_word_i = words.iter().max_by_key(|s| s.len()); // this is a great way
    let longest_word_ii = words.iter().max_by(|a, b| a.len().cmp(&b.len())); // slower alternative version
    println!("{:?}, {:?}", longest_word_i, longest_word_ii);
}
