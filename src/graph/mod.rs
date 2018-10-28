pub mod word;
use graph::word::Word;
    pub struct WordGraph {
        pub nodes:Vec<Word>
    }

    impl WordGraph{
        pub fn new(nodes:Vec<Word>) -> WordGraph{
            WordGraph {
                nodes:nodes
            } 
        }

        /**
         *Map out neighbors
         */
        pub fn setup_neighbors(&mut self){
            for i in 0..self.nodes.len()-1 {
                let slice_tuple = self.nodes.split_at_mut(i+1); //Get two mutable slices from Vec
              //  println!("Split is: {} : {} ",slice_tuple.0.len(),slice_tuple.1.len());
                let node_one = &mut slice_tuple.0[slice_tuple.0.len()-1]; //Mutable ref number one.
                for (j,mut node_too) in slice_tuple.1.iter_mut().enumerate(){
                    let check = node_one.check_neighbor(&mut node_too); //Mutable ref number two.

                    if check {
                        node_one.add_neighbor(j+i);
                        node_too.add_neighbor(i);
            //            println!("{} {} are neighbors",node_one.full,node_too.full);
                    }
                
                }
            }
        }
        pub fn print_node(&self,node:&Word){
            println!("For node: {}",node.full);
            println!("Neighbord: ");
            for &i in &node.neighbors {
                println!("{}: {}",i,self.nodes[i].full);
                
            }
        
        }
        pub fn print_nodes(&self){
            for ref node in &self.nodes {
                self.print_node(&node);
            }
        }

        pub fn longest_ladder(graph: &WordGraph,start_node:usize,connections:Vec<usize>) -> Vec<usize>{
            let mut longest_walk:Vec<usize>=connections.to_owned();
            let node_origin:&Word = &graph.nodes[start_node];

            println!("Check in:");
            for  (i,j) in connections.iter().enumerate() {
                println!("{}: {}",graph.nodes[j.clone()].full);
            }

            for (i,node) in node_origin.neighbors.iter().enumerate() {
                if !connections.contains(node) {
                    let mut connect_me = connections.to_owned();
                    connect_me.push(node.clone());
                    //println!("So far: {} {}",connect_me.len(),connections.len());
                    let path = WordGraph::longest_ladder(graph,node.clone(),connect_me);
                    //println!("BACK! {}, {}, {}",path.len(),i,node_origin.neighbors.len());
                    if path.len() > longest_walk.len() {
                        //println!("Found a length from {} {}",node_origin.full,path.len());
                        longest_walk = path;
                    }
                    
                }
                
            }
            //println!("Return from {} : {} : {}",connections.len(),node_origin.full,longest_walk.len());
            longest_walk 
        
        }
 /*       pub fn setup_neighbors(&mut self){
            for i in 0..self.nodes.len()-1 {
                let mut node_one = &self.nodes[i];
                for j in (i+1)..self.nodes.len()-1 {
                    let mut node_too = &mut self.nodes[j];
                    let check = node_one.check_neighbor(&mut node_too);
                    
                    if check {
                        node_one.add_neighbor(j);
                        node_too.add_neighbor(i);
                    }

                }

            }
        }*/

    }
