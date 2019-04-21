mod graph;
use graph::WordGraph;
use graph::graphstat::GraphStat;
use graph::word::Word;
use std::env;
use std::sync::{Arc,RwLock};
use std::thread;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver,SendError};
use std::error::Error;

fn main() {
    let args:Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => walk_the_tree_walk(arg.clone()),
        _ => println!("This program needs an input of equivalent length words to run.")
    }

}
/**
 *Read file of equivalent length words, calculate neighbors for each block and
 *calculate longest path via a depth-first approach.
 */
fn walk_the_tree_walk(file_path:String) {
    let path = Path::new(&file_path);
    let reader = match File::open(Path::new(path)) {
        Err(why_man_why) => panic!("I just couldn't do it. I'm sorry. {} {}",file_path,why_man_why.description()),
        Ok(file) => 
            BufReader::new(file)
    };
    let word_up:Vec<Word> = reader.lines().filter(|line| match line { Ok(_string) => true, _ => false})
        .map(|string| {Word::new(string.unwrap())}).collect(); //words
    let mut graph = WordGraph::new(word_up);
    //let mut graph_stats = GraphStat::new();
    graph.setup_neighbors(); 
    start_async_walk(graph);
    /*for (i,_node) in graph.nodes.iter().enumerate() {
        print!("\rNo. {}, max: {} ",i,graph_stats.max_length);
        let _ladder:Vec<usize>  =  WordGraph::longest_ladder(&graph,i,vec!(i),&mut 0,&mut graph_stats); 
    }*/
}


fn start_async_walk(graph:WordGraph){
    let mut longest:Vec<usize> = vec!();
    let threads = 4; //Could be input
    let total = graph.nodes.len();
    let immutable_share = Arc::new(graph);

    let stat_lock = Arc::new(RwLock::new(GraphStat::new()));
    let (sender,receiver):(Sender<(usize,Vec<usize>)>,Receiver<(usize,Vec<usize>)>)= mpsc::channel();
    for num in 0..threads {
      start_thread(num,&stat_lock,&sender,&immutable_share);
    }
    /*
     *Waits for result, keeps if longer. Prints index for progress monitoring.
     */
    for resp in 0..total {
      match receiver.recv() {
         Ok(result) => {
           println!("{}",result.0);
           longest = compare_longest(longest,result.1); 
         },
        _ => println!("No result for {}",resp)
      }
      start_thread(resp+threads,
                   &stat_lock,
                   &sender,
                   &immutable_share);//Put next iteration on queue.
    }

    println!("Longest graph identified is: {}",longest.len());
    print_the_nodes(&immutable_share.nodes,longest);
}

fn compare_longest(first:Vec<usize>,second:Vec<usize>) -> Vec<usize>{
   return if first.len()>=second.len() { first } else { second };
}

fn start_thread(start_node:usize,
                stat:&Arc<RwLock<GraphStat>>,
                sender:&Sender<(usize,Vec<usize>)>,
                graph:&Arc<WordGraph>){ //Look at RC type for WordGraph
      /*
       *Shadowing of parameters here.
       *Feels wrong, but saves a lot of 
       *coming up with variable names.
       */
      let sender=sender.clone();
      let stat = stat.clone();
      let graph = graph.clone();
      thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move||{
        let calculated_value = (start_node,WordGraph::_longest_ladder_async(&graph,start_node,vec!(start_node),&mut 0,&stat));
        write_check(sender.send(calculated_value));
        }
        ).unwrap(); //Triggers walk of words for first X concurrent threads.
}
fn write_check(result:Result<(),SendError<(usize,Vec<usize>)>>){
  match result {
    Ok(())=> (),
    Err(_e) => ()
  }
}

fn _print_the_thing(graph:&WordGraph,nodes:Vec<usize>) {
  print_the_nodes(&graph.nodes,nodes);
}
fn print_the_nodes(all_nodes:&Vec<Word>,nodes:Vec<usize>){
  for i in nodes {
        println!("{}",all_nodes[i].full);
  }
}

