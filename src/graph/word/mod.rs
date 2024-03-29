use std::clone::Clone;

    #[derive(Debug)]
    pub struct Word {
        pub full:String,
        pub neighbors:Vec<usize>,
        pub char_vec:Vec<char>
    }

    impl Word {

        pub fn new(word:String) -> Word{
            return Word {
                full:word.clone(),
                neighbors: Vec::new(),
                char_vec: word.chars().collect()
            };
        }

        pub fn add_neighbor(&mut self,index:usize){
            self.neighbors.push(index);
        }
        pub fn check_neighbor(&self, compare_word:&mut Word) -> bool  {
            //Assume same length words, could assume pure ascii one-byte chars but.. nah.
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
impl Clone for Word {
    fn clone(&self) -> Word {
        Word {
            full:self.full.clone(),
            neighbors:self.neighbors.clone(),
            char_vec:self.char_vec.clone()
        }
    }
    }





