use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
use word::Word;
mod word;
fn main() {
    let args:Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => do_the_thing(arg.clone()),
        _ => ()
    }
    let size = args.len() as u32; //Usize is always copied. "as" keyword allows casting.


}
fn do_the_thing(file_path:String) {
    let path = Path::new(&file_path);
    let reader = match File::open(Path::new(path)) {
        Err(why_man_why) => panic!("I just couldn't do it. I'm sorry. {} {}",file_path,why_man_why.description()),
        Ok(file) => 
            BufReader::new(file)
    };
    let word_up:Vec<Word> = reader.lines().filter(|line| match line { Ok(_string) => true, _ => false})
        .map(|string| {Word::new(string.unwrap())}).collect(); //words

}

fn map_neighbors(mut words_up:Vec<Word>){
    for mut word in words_up.iter_mut() {
//        word.store_neighbors(&mut words_up);
    }
    
}
