/**
 * Used to track stats about the longest graph.
 * For multi-threading test with Sync,Send and RWLock
 */
pub struct GraphStat {
    pub max_length:usize
}
unsafe impl Send for GraphStat{}
unsafe impl Sync for GraphStat{}
impl GraphStat {
    pub fn new() -> GraphStat {
        GraphStat {
            max_length:0
        }

    }

}
