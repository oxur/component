use component::deps;
use component::types::Component;
use petgraph::dot::Dot;
use petgraph::Directed;
use petgraph::graphmap::GraphMap;

fn main() {
    // let mut g = Graph::new();
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
            name: "myapp::components::comp3",
            dependencies: vec![
                "myapp::components::comp7",
                "myapp::components::comp8"],
        },
        Component {
            name: "myapp::components::comp4",
            dependencies: vec![
                "myapp::components::comp5",
                "myapp::components::comp8"],
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
    println!("Sorted:");
    for comp in deps::sort(&g) {
        println!("{}", comp);
    }
    // match deps::sort(&g) {
    //     indices => {
    //         for ni in indices.iter() {
    //             println!("Got {:?}", ni);
    //             println!("{:?}", g.get(ni.index()));
    //         }
    //     }
    // }
    println!("Dot language format:\n\n{}", Dot::new(&g));
}
