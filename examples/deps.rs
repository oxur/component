use component::deps;
use component::Component;
use petgraph::dot::Dot;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;

fn main() {
    let mut g = GraphMap::<&str, String, Directed>::new();
    let deps = vec![
        Component {
            name: "myapp::components::comp1",
            dependencies: vec![
                "myapp::components::comp2",
                "myapp::components::comp3",
                "myapp::components::comp4",
            ],
        },
        Component {
            name: "myapp::components::comp3",
            dependencies: vec!["myapp::components::comp7", "myapp::components::comp8"],
        },
        Component {
            name: "myapp::components::comp4",
            dependencies: vec!["myapp::components::comp5", "myapp::components::comp8"],
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
    println!("\nGraph struct:\n\n{:?}\n", g);
    println!("Sorted deps:\n");
    for comp in deps::sort(&g) {
        println!("{}", comp);
    }
    println!("\nDot language format:\n\n{}", Dot::new(&g));
}
