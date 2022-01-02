use std::collections::HashMap;
use std::collections::HashSet;

pub struct Node<'a> {
    name: String,
    distance: i32,
    path: Vec<&Node>,
    neighbors: HashMap<&Node, i32>
}

pub fn find(start: &Node) {
    let mut settled_nodes: HashSet<&Node> = HashSet::new();
    let mut unsettled_nodes: HashSet<&Node> = HashSet::new();

    unsettled_nodes.insert(start);

    while !unsettled_nodes.is_empty() {
        let mut lowest_distance_node = find_lowest_distance_node(&mut unsettled_nodes);
        unsettled_nodes.remove( &lowest_distance_node);
        let mut neighbors: &HashMap<&Node, i32> = &mut lowest_distance_node.neighbors;
        for (key, value) in neighbors.iter_mut() {
            if !settled_nodes.contains(&key) {
                update_distance(lowest_distance_node, &value, *key);
                unsettled_nodes.insert( &key);
            }
        }
        settled_nodes.insert(lowest_distance_node);
    }
}

fn find_lowest_distance_node(nodes: &mut HashSet<&Node>) -> &mut Node {
    let mut node: &mut Node = &mut Node {
        name: "".to_owned(),
        distance: i32::MAX,
        path: vec![],
        neighbors: HashMap::new()
    };
    for value in nodes.iter_mut() {
        if value.distance < node.distance {
            node = value;
        }
    }
    node
}

fn update_distance(current: &Node, distance_between: &i32, to_be_updated: &mut Node) {
    if to_be_updated.distance > current.distance + distance_between {
        let mut new_path = current.path.clone();
        new_path.insert(new_path.len(), current);
        to_be_updated.path = path;
        to_be_updated.distance = current.distance + distance_between;
    }
}