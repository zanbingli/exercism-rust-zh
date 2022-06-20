
pub mod graph {
    use std::collections::HashMap;
    use graph::graph_items::edge::Edge;
    use graph::graph_items::node::Node;

    #[derive(Clone, PartialEq,Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String,String>,
    }

    pub mod graph_items  {
        pub mod edge{
            use std::collections::HashMap;

            #[derive(Clone, PartialEq,Debug)]
            pub struct Edge {
                pub va:(String,String),
                pub attrs: HashMap<String,String>,
            }

            impl Edge {
                pub fn new(st:&str,ed:&str) -> Self {
                    Edge {
                        va:(st.to_owned(),ed.to_owned()),
                        attrs:HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self,attrs :&[(&str,&str)])-> Self{
                    for nd in attrs.iter(){
                        self.attrs.insert(nd.0.to_owned(),nd.1.to_owned());
                    }
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq,Debug)]
            pub struct Node {
                pub name:String,
                pub attrs: HashMap<String,String>,
            }

            impl Node {
                pub fn new(_va:&str) -> Self {
                    Node {
                        name:_va.to_string(),
                        attrs:HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|&(name, value)| {
                        self.attrs.insert(name.to_string(), value.to_string());
                    });

                    self
                }

                pub fn get_attr(&self, attr:&str) -> Option<&str> {
                    self.attrs.get(attr).map(|v|v.as_str())
                }
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes :&Vec<Node>) -> Self {
            for nd in nodes.iter(){
                self.nodes.push(nd.clone());
            }
            self
        }
        pub fn with_edges(mut self,edges :&Vec<Edge>) ->Self{
            for nd in edges.iter(){
                self.edges.push(nd.clone());
            }
            self
        }
        pub fn with_attrs(mut self,attrs :&[(&str,&str)])-> Self{
            for nd in attrs.iter(){
                self.attrs.insert(nd.0.to_owned(),nd.1.to_owned());
            }
            self
        }
        pub fn get_node(self,nd:&str)->Option<Node>{
            for n in self.nodes.iter() {
                if n.name == nd.to_owned() {
                    return Some(n.clone());
                }
            }
            None
        }
    }

}

