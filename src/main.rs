use component::deps;
use component::types::Component;
use petgraph::dot::Dot;
use petgraph::graph::Graph;

fn main() {
    let mut g = Graph::new();
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
    deps::add_components(&mut g, deps);
    println!("{:?}\n", g);
    deps::sort(&g);
    // match deps::sort(&g) {
    //     indices => {
    //         for ni in indices.iter() {
    //             println!("Got {:?}", ni);
    //             println!("{:?}", g.get(ni.index()));
    //         }
    //     }
    // }
    println!("{}", Dot::new(&g));
}
