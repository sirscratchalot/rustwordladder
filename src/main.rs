mod graph;
use graph::WordGraph;
use graph::graphstat::GraphStat;
use graph::word::Word;
use std::env;
use std::thread;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::error::Error;

fn main() {
    let args:Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => walk_the_tree_walk(arg.clone()),
        _ => ()
    }

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

    for (i,_node) in graph.nodes.iter().enumerate() {
        print!("\rNo. {}, max: {} ",i,graph_stats.max_length);
        let _ladder:Vec<usize>  =  WordGraph::longest_ladder(&graph,i,vec!(i),&mut 0,&mut graph_stats); 
    }
}

fn start_async_walk(graph:WordGraph){
    let (sender,receiver)= mpsc::channel();
    for _num in 0..4 {
      let send=sender.clone();
      thread::spawn(move||{
        send.send("GRRR")}); //Triggers walk of words for first X concurrent threads.
    }

    for resp in 0..graph.nodes.len() {
      match receiver.recv() {
         Ok(result) => println!("{}",result),
        _ => println!("No result for {}",resp)
      }
    }
}

fn print_the_thing(graph:&WordGraph,nodes:Vec<usize>) {
    for i in nodes {
        println!("{}",graph.nodes[i].full);
    }
    
}

