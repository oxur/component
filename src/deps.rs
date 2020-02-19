use petgraph::algo;
use petgraph::graphmap::DiGraphMap;
use crate::types::Component;

// The verb to use when thinking about the relationships between the dependency
// vertices is "depends upon." A component will declare the things it depends
// upon, and the direction of the graph is from component -> dep 1,
// component -> dep 2, etc.
//
// Great resources for reading up on petgraph:
// * https://timothy.hobbs.cz/rust-play/petgraph_review.html
// * https://timothy.hobbs.cz/rust-play/petgraph-internals.html
// * https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/

pub fn add_component(g: &mut DiGraphMap<&str, String>, c: &Component) {
    let comp_vert = g.add_node(c.name);
    for dep in c.dependencies.iter() {
        let dep_vert = g.add_node(dep);
        g.add_edge(comp_vert, dep_vert, format!("{} -> {}", c.name, dep));
    }
}

pub fn add_components(g: &mut DiGraphMap<&str, String>, cs: Vec<Component>) {
    for c in cs.iter() {
        add_component(g, c);
    }
}

// pub fn sort<'a>(g: &DiGraph<&str, String>) -> Vec<NodeIndex> {
pub fn sort<'a>(g: &DiGraphMap<&'a str, String>) -> Vec<&'a str> {
    match algo::toposort(g, Option::None) {
        Ok(sorted) => {
            sorted
                .iter()
                .rev()
                .map(| item: &&str | *item)
                .collect()
        },
        Err(err) => {
            panic!("Error: {:?}", err);
            // g.node_weight(err.node_id()).map(|weight|
            //     println!("Error: graph has cyclic dependency at {}", weight));
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::Directed;
    use petgraph::graphmap::GraphMap;
    use super::*;

    #[test]
    fn new_test() {
        let mut g = GraphMap::<&str, String, Directed>::new();
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
