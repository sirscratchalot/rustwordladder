pub mod word;
pub mod graphstat;

use graph::word::Word;
use graph::graphstat::GraphStat;
const MAX_ITERATIONS:usize = 100000;
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
                    node_one.add_neighbor(j+i+1);
                    node_too.add_neighbor(i);
                }

            }
        }
        self.sort_node_neighbors();
    }
    /**
     * Sort neighbor array so that neighbor with the most neighbors is index 0.
     */
    fn sort_node_neighbors(&mut self){
        let refNodes = self.nodes.clone(); //Simplest way to allow retrieving fields in closure.
        for mut node in self.nodes.iter_mut() {
            node.neighbors.sort_by(|a,b| refNodes[*a]
                                   .neighbors.len().clone().cmp(&refNodes[*b].neighbors.len()));
        }
    }
    pub fn print_node(&self,node:&Word){
        println!("For node: {}",node.full);
        println!("Neighbors: ");
        for &i in &node.neighbors {
            println!("{}: {}",i,self.nodes[i].full);

        }

    }
    pub fn print_nodes(&self){
        for ref node in &self.nodes {
            self.print_node(&node);
        }
    }
    pub fn print_graph(graph:&WordGraph,connections:&Vec<usize>){
        println!("New max length! {}",connections.len());
        for (i,word) in connections.iter().enumerate() {
            println!("{}: {}",i,graph.nodes[*word].full);
        }
    }
    fn check_circular(nodes:&Vec<usize>) -> bool{
        for i in nodes {
            let mut count:usize = 0;
            for j in nodes {
                if i == j {
                    count = count +1;
                }
            }
            if count > 1 {
               println!("Circular vector! Found {} at {} positions",i,count);
               return true;
            }
        }
        return false;
    }

    pub fn longest_ladder(graph: &WordGraph
    ,start_node:usize
    ,connections:Vec<usize>
    ,total_iter:&mut usize
    ,graph_stats:&mut GraphStat) -> Vec<usize> {
     /*   
     //I forgot this was here and spent an hour per evening for a week trying to figure out why my graphs were stalling at 50 length. 
     //Left in here because humble pie is healthy for you.
     if connections.len()>50 {
            return connections;
        }*/
        *total_iter=*total_iter+1;
        let mut longest_walk:Vec<usize>=connections.to_owned();
        let node_origin:&Word = &graph.nodes[start_node];

        for (i,node) in node_origin.neighbors.iter().enumerate() {
            if(total_iter.clone()>MAX_ITERATIONS){
                return longest_walk;
            }
            if !connections.contains(node) {
                let mut connect_me = connections.to_owned();
                connect_me.push(node.clone());
                let path = WordGraph::longest_ladder(
                    graph,node.clone()
                    ,connect_me
                    ,total_iter
                    ,graph_stats);

                if path.len() > longest_walk.len() {
                    longest_walk = path;
                    if graph_stats.max_length < longest_walk.len() {
                        graph_stats.max_length = longest_walk.len().clone();
                        println!("New max length vector found: {}",graph_stats.max_length);
                        WordGraph::print_graph(graph,&longest_walk);
                    }
                }

            }

        }
        //print!("\rThe end my friend!: {}, {}",longest_walk.len(),graph.nodes[start_node].full); 
        longest_walk 

    }

}
#[cfg(test)]
mod tests {
    use graph::WordGraph;
    use graph::word::Word;
    use graph::graphstat::GraphStat;
    #[test]
    fn test_circular() {
        let circular = vec!(1,2,3,4,3,3);
        let not_circular = vec!(1,2,3,4,5,6);

        assert_eq!(WordGraph::check_circular(&circular),true);
        assert_eq!(WordGraph::check_circular(&not_circular),false);


    }
    #[test]
    fn test_longest() {
        let words:Vec<Word> = vec!("burr","durr","hurr","buor","noor","boor")
            .iter().map(|word| { Word::new(word.to_string())}).collect();
        let mut graph = WordGraph::new(words);
        let mut graph_stats = GraphStat::new();
        graph.setup_neighbors();
        graph.print_nodes();
        let mut longest:Vec<usize> = Vec::new();

    for (i,node) in graph.nodes.iter().enumerate() {
        println!("No. {}, max: {} ",i,graph_stats.max_length);
        let ladder:Vec<usize>  =  WordGraph::longest_ladder(&graph,i,vec!(i),&mut graph_stats); 
        if ladder.len()>longest.len() {
            longest = ladder;
        }
    }
    assert_eq!(longest,vec!(1,2,0,3,5,4)); 


    }
}

