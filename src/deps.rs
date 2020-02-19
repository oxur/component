use petgraph::algo;
use petgraph::graph::{DiGraph, Graph, NodeIndex};
use crate::types::Component;

// The verb to use when thinking about the relationships between the dependency
// vertices is "depends upon." A component will declare the things it depends
// upon, and the direction of the graph is from component -> dep 1,
// component -> dep 2, etc.

pub fn add_component(g: &mut DiGraph<&str, String>, c: &Component) {
    let comp_vert = g.add_node(c.name);
    for dep in c.dependencies.iter() {
        let dep_vert = g.add_node(dep);
        g.add_edge(comp_vert, dep_vert, format!("{} -> {}", c.name, dep));
    }
}

pub fn add_components(g: &mut DiGraph<&str, String>, cs: Vec<Component>) {
    for c in cs.iter() {
        add_component(g, c);
    }
}

// pub fn sort<'a>(g: &DiGraph<&str, String>) -> Vec<NodeIndex> {
pub fn sort<'a>(g: &DiGraph<&str, String>) {
    match algo::toposort(g, Option::None) {
        Ok(sorted) => {
            for idx in sorted {
                g.node_weight(idx).map(|weight| {
                    println!("index: {:?}; vertex: {}, ", idx.index(), weight);
                    weight;
                });
            }
        },
        Err(err) => {
            g.node_weight(err.node_id()).map(|weight|
                println!("Error: graph has cyclic dependency at {}", weight));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let mut g = GraphMap::new();
        let deps = vec![
            Component{
                name: "myapp::components::comp1",
                dependencies: vec![
                    "myapp::components::comp2",
                    "myapp::components::comp3",
                    "myapp::components::comp4",
                ]
            },
            Component {
                name: "myapp::components::comp4",
                dependencies: vec!["myapp::components::comp5"],
            },
            Component {
                name: "myapp::components::comp5",
                dependencies: vec!["myapp::components::comp6"],
            },
            Component {
                name: "myapp::components::comp6",
                dependencies: vec!["myapp::components::comp7"],
            },
        ];
        add_components(&mut g, deps);
        println!("{:?}", g);
        // match sort(&g) {
        //     x => println!("Got {:?}", x),
        // }
    }
}
