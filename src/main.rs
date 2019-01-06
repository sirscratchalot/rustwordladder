mod graph;
use graph::WordGraph;
use graph::graphstat::GraphStat;
use graph::word::Word;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use std::error::Error;
fn main() {
    let args:Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => walk_the_tree_walk(arg.clone()),
        _ => ()
    }
    let size = args.len() as u32; //Usize is always copied. "as" keyword allows casting.


}
fn walk_the_tree_walk(file_path:String) {
    let path = Path::new(&file_path);
    let reader = match File::open(Path::new(path)) {
        Err(why_man_why) => panic!("I just couldn't do it. I'm sorry. {} {}",file_path,why_man_why.description()),
        Ok(file) => 
            BufReader::new(file)
    };
    let word_up:Vec<Word> = reader.lines().filter(|line| match line { Ok(_string) => true, _ => false})
        .map(|string| {Word::new(string.unwrap())}).collect(); //words
    let mut graph_stats = GraphStat::new();
    let mut graph = WordGraph::new(word_up);

    graph.setup_neighbors(); 
    graph.print_nodes();

    let mut longest:usize = 0;
    for (i,node) in graph.nodes.iter().enumerate() {
        print!("\rNo. {}, max: {} ",i,graph_stats.max_length);
        let mut total_iter = 0;
        let ladder:Vec<usize>  =  WordGraph::longest_ladder(&graph,i,vec!(i),&mut total_iter,&mut graph_stats); 
    }
}

fn print_the_thing(graph:&WordGraph,nodes:Vec<usize>) {
    for i in nodes {
        println!("{}",graph.nodes[i].full);
    }
    
}

