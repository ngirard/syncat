use tree_sitter::TreeCursor;
use super::{helper::*, Node};
use crate::{Query, QuerySlice, Matches};

#[derive(Clone, Debug)]
pub(crate) struct Selector {
    pub(crate) nodes: Vec<Node>,
}

impl Selector {
    pub(crate) fn matches<'a>(&self, query: &'a Query<'a>) -> Option<Matches<'a>> {
        Self::matches2(&self.nodes[..], &QuerySlice::new(query))
    }

    fn matches2<'a>(nodes: &[Node], query: &QuerySlice<'a>) -> Option<Matches<'a>> {
        if nodes.is_empty() {
            return Some(Matches::default());
        }
        let (subquery, new_matches) = query.find(nodes.last().unwrap())?;
        if let Some(mut matches) = Self::matches2(&nodes[..nodes.len() - 1], &subquery) {
            for (key, value) in new_matches {
                matches.insert(key, value);
            }
            Some(matches)
        } else if nodes.last().unwrap().can_try_again() {
            Self::matches2(nodes, &subquery)
        } else {
            None
        }
    }
}

impl FromSource for Selector {
    fn from_source(tree: &mut TreeCursor, source: &[u8]) -> crate::Result<Self> {
        children!(tree, "selector");
        let mut nodes = vec![];
        while {
            if !tree.node().is_extra() {
                nodes.push(Node::from_source(tree, source)?);
            }
            tree.goto_next_sibling()
        } {}
        tree.goto_parent();
        Ok(Self { nodes })
    }
}
