use crate::trees::{TreeBuild, TreeMap};
use crate::{Str, Tree};

struct TreePath();

static EMPTY_VEC: &Vec<Tree> = &vec![];

impl TreePath {
    /// Returns the text value of this tree or a debug representation.
    pub fn value(tree: &Tree) -> Str {
        match tree {
            Tree::String(text) => text.to_string(),
            _ => format!("{:#?}", tree),
        }
    }

    /// Returns the child elements if this is a Seq or List, or an empty vec.
    pub fn list_value(tree: &Tree) -> &Vec<Tree> {
        if let Some(items) = TreeBuild::seq_items(tree) {
            return items
        }
        EMPTY_VEC
    }

    /// Returns the child elements as text values, or an empty vec.
    pub fn str_list_value(tree: &Tree) -> Vec<Str> {
        Self::list_value(tree).iter().map(|t| t.to_string()).collect()
    }

    /// Returns the inner TreeMap if this is a Map variant.
    pub fn map_value(tree: &Tree) -> Option<&TreeMap> {
        TreeBuild::tree_map(tree)
    }

    /// Looks up a key in the Map variant and returns the corresponding tree.
    pub fn get<'a>(tree: &'a Tree, key: &str) -> Option<&'a Tree> {
        if let Some(map) = TreeBuild::tree_map(tree) {
            return map.get(key)
        }
        None
    }

    /// Looks up a key and returns its text value, or an empty string.
    pub fn get_value(tree: &Tree, key: &str) -> Str {
        if let Some(valtree) = Self::get(tree, key) {
            return Self::value(valtree)
        }
        "".to_string()
    }

    /// Looks up a key and returns its list children, or an empty vec.
    pub fn get_list<'a>(tree: &'a Tree, key: &str) -> &'a Vec<Tree> {
        if let Some(valtree) = Self::get(tree, key)
            && let Some(items) = TreeBuild::seq_items(valtree) {
            return items
        }
        EMPTY_VEC
    }

    /// Looks up a key and returns its children as text values.
    pub fn get_str_list<'a>(tree: &Tree, key: &str) -> Vec<Str> {
        Self::get_list(tree, key)
            .iter()
            .map(|t| Self::value(t))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 96;

    #[test]
    fn test_tree_size() {
        let size = size_of::<Tree>();
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }
    #[test]
    fn test_keyval_size() {
        let size = size_of::<KeyValue>();
        assert!(size <= TARGET, "KeyVal size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_node_nil_removal() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = Tree::fold(raw);

        assert_eq!(result, Tree::fold(Tree::Bottom));
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = Tree::fold(raw);

        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::from(vec![Tree::Bottom, Tree::Nil, Tree::Bottom]);
        let result = Tree::fold(raw);

        if let Tree::Array(v) = result {
            assert_eq!(v.len(), 2);
            assert_eq!(*v[0], Tree::Bottom);
            assert_eq!(*v[1], Tree::Bottom);
        } else {
            panic!("Expected Closure, got {:?}", result);
        }
    }

    #[test]
    fn test_node_nil_purging_preserves_count() {
        let raw = Tree::from(vec![Tree::Nil, Tree::Bottom, Tree::Nil]);
        let result = Tree::fold(raw);

        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_named_group_with_inner_names() {
        let tree = Tree::named(
            "x".into(),
            Tree::Seq(
                [
                    Tree::named("a".into(), Tree::text("a".into()).into()).into(),
                    Tree::named("b".into(), Tree::text("b".into()).into()).into(),
                ]
                    .into(),
            )
                .into(),
        );

        let result = Tree::fold(tree);

        assert!(matches!(result, Tree::Object(_)));
        if let Tree::Object(m) = result {
            assert!(m.get("x").is_some(), "key 'x' should be present");
            assert!(m.get("a").is_some(), "key 'a' should be present");
            assert!(m.get("b").is_some(), "key 'b' should be present");
        }
    }
}
