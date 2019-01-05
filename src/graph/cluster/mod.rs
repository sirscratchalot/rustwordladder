    #[derive(Debug)]
    pub struct Cluster {
        pub key:String,
        pub members:Vec<usize>
    }
    #[derive(Debug)]
    pub struct ClusterLink {
        pub key:String,
        pub clusterEdge: (usize,usize),
        pub clusterLink: (usize,usize)
    }

    impl Cluster {

        pub fn new(key:String) -> Cluster{
            return Cluster {
                key:String,
                members:Vec::new()
            };
        }

        pub fn add_neighbor(&mut self,index:usize){
            self.neighbors.push(index);
        }

        pub fn check_neighbor(&self, compare_clust:&mut Cluster,entry_point:usize) -> (bool,usize)  {
            for compare in compare_clust.members {
                if compare!=entry_point && self.members.contains(compare){
             (true,compare)
                }
            }
            (false,0)
            
        }
    }


