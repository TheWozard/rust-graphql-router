pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
}

pub struct Node<T> {
    pub typ: T,
    pub targets: Vec<Target<T>>,
}

pub struct Target<T> {
    pub typ: T,
    pub rel: Relationship,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Relationship {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl Relationship {
    fn invert(self: Self) -> Self {
        match self {
            Self::OneToOne => Self::OneToOne,
            Self::OneToMany => Self::ManyToOne,
            Self::ManyToOne => Self::OneToMany,
            Self::ManyToMany => Self::ManyToMany,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link<'a,T> {
    pub from: &'a T,
    pub to: &'a T,
    pub rel: &'a Relationship,
}

impl<T> Graph<T> {
    fn links(&self) -> Vec<Link<T>> {
        return self.nodes.iter()
            .flat_map(|n| n.targets.iter()
            .map(|t| Link{from: &n.typ, to: &t.typ, rel: &t.rel})).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum TestNodes {
        A, B,
    }

    fn vec_compare<T: std::cmp::PartialEq>(va: &Vec<T>, vb:  &Vec<T>) -> bool {
        (va.len() == vb.len()) &&
         va.iter().zip(vb).all(|(a,b)| a == b)
    }

    #[test]
    fn test_links_from_graph() {
        let graph = Graph{nodes:vec![
            Node{typ: TestNodes::A, targets:vec![
                Target{typ: TestNodes::B, rel:Relationship::OneToOne}
                ]}
            ]};
        assert!(vec_compare(&graph.links(), &vec![
            Link{from: &TestNodes::A, to: &TestNodes::B, rel: &Relationship::OneToOne }
        ]));
    }
}


