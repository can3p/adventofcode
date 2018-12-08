#[test]
fn day08_1_test_sort() {
    const INPUT: &str = "1 2 3 4";

    assert_eq!(read_contents(INPUT), [1,2,3,4]);
}

#[test]
fn day08_1_test_tree() {
    const INPUT: &str = include_str!("../input/08_test.txt");
    let tree: Node = create_tree(read_contents(INPUT));
    assert_eq!(tree.metadata, [1, 1, 2]);
    assert_eq!(tree.children.len(), 2);
}

#[test]
fn day08_1_test() {
    const INPUT: &str = include_str!("../input/08_test.txt");
    assert_eq!(day08_1(INPUT), 138);
}

#[test]
fn day08_1_input() {
    const INPUT: &str = include_str!("../input/08_input.txt");
    assert_eq!(day08_1(INPUT), 138);
}

#[test]
fn day08_2_test() {
    const INPUT: &str = include_str!("../input/08_test.txt");
    assert_eq!(day08_2(INPUT), 66);
}

#[test]
fn day08_2_input() {
    const INPUT: &str = include_str!("../input/08_input.txt");
    assert_eq!(day08_2(INPUT), 66);
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>
}

fn day08_1(input: &str) -> i32 {
    let tree: Node = create_tree(read_contents(input));

    return sum_metadata(&tree);
}

fn day08_2(input: &str) -> i32 {
    let tree: Node = create_tree(read_contents(input));

    return calc_node_value(&tree);
}

fn calc_node_value(node: &Node) -> i32 {
    if node.children.len() == 0 {
        return node.metadata.iter().sum::<i32>();
    }

    return node.metadata.iter()
                        .map(|&val| {
                            if val == 0 || (val as usize) > node.children.len() {
                                return 0;
                            }

                            // we can care to cache value there but we don't
                            return calc_node_value(&node.children[(val - 1) as usize]);
                        }).sum::<i32>();
}

fn sum_metadata(node: &Node) -> i32 {
    return node.metadata.iter().sum::<i32>()
        + node.children.iter().map(|n| sum_metadata(&n)).sum::<i32>();
}

fn create_tree(seq: Vec<i32>) -> Node {
    let mut idx: usize = 0;
    return create_node(&seq, &mut idx);
}

fn create_node(seq: &Vec<i32>, cur_idx: &mut usize) -> Node {
    let num_children = seq[*cur_idx];
    *cur_idx += 1;
    let num_metadata = seq[*cur_idx];
    *cur_idx += 1;
    let mut children: Vec<Node> = Vec::new();
    let mut metadata: Vec<i32> = Vec::new();

    for _ in 0..num_children {
        children.push(create_node(seq, cur_idx));
    }

    for _ in 0..num_metadata {
        metadata.push(seq[*cur_idx]);
        *cur_idx += 1;
    }

    let node =  Node {
        children: children,
        metadata: metadata
    };

    return node;
}

fn read_contents(contents: &str) -> Vec<i32> {
    let input = contents.lines().next().unwrap();

    let vec = input.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

    return vec;
}
