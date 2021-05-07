#[derive(Debug, Clone)]
struct Node {
    next: Option<Box<Node>>,
    name: String,
}

impl Node {
    // impl Into<String>
    // で使う側は&strとStringを気にしなくて良くなる
    fn new(name: impl Into<String>) -> Self {
        Self {
            next: None,
            name: name.into(),
        }
    }

    // 自分の前にnodeを追加する
    // selfがmoveする
    // 返却値はself
    fn insert(self, p: Self) -> Self {
        Self {
            next: Some(Box::new(self)),
            name: p.name
        }
    }
}

// impl Iterator for Node {
//     type Item = Node;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(hoge) = &self.next {
//             let fuga = **hoge.clone();
//             return Some(fuga);
//         } else {
//             return None;
//         }
//     }
// }

fn main() {
    let names = vec!["watanabe", "ito", "takahashi", "suzuki", "sato"];
    let mut list = Node::new("yamamoto");
    print_list(&list);

    for name in names {
        list = list.insert(Node::new(name));
        print_list(&list);
    }
}

fn print_list(list: &Node) {
    let mut ptr = list;
    let mut nodes = String::new();

    loop {
        nodes += &format!("{} -> ", &ptr.name);

        match &ptr.next {
            Some(next_node) => {
                ptr = &*next_node;
            }
            None => break,
        };
    }

    println!("{}", nodes);
}
