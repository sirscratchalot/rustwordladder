use graph::word::Word;
/**
 * Used to track stats about the longest graph.
 * For multi-threading might need a mutex.
 */
pub struct GraphStat {
    pub max_length:usize
}
impl GraphStat {
    pub fn new() -> GraphStat {
        GraphStat {
            max_length:0
        }

    }

}
