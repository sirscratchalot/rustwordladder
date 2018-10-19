#[derive(Debug)]
pub struct Word {
    full:String,
    neighbors:Vec<usize>,
    char_vec:Vec<char>
}

impl Word {

    pub fn new(word:String) -> Word{
        return Word {
            full:word.clone(),
            neighbors: Vec::new(),
            char_vec: word.chars().collect()
        };
    }

    pub fn check_neighbor(&self, compare_word:&Word) -> bool  {
        //Assume same length words, could pure ascii one-byte chars but.. nah.
        //This works with scalars..
        let mut mismatches = 0;

        for i in 0..self.char_vec.len() {
         if self.char_vec[i] != compare_word.char_vec[i] {
            if mismatches >=1 {
            return false;
            }
            mismatches = mismatches+1;
         }
        }
        true
    }


}
