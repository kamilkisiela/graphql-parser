//! Query syntax tree traversal.
//!
//! Each method of [`QueryVisitor`] is a hook that can be overridden to customize the behavior when
//! visiting the corresponding type of node. By default, the methods don't do anything. The actual
//! walking of the ast is done by the `walk_*` functions. So to run a visitor over the whole
//! document you should use [`walk_document`].
//!
//! Example:
//!
//! ```
//! use graphql_parser::query::{
//!     Field,
//!     parse_query,
//!     query_visitor::{QueryVisitor, walk_document},
//! };
//!
//! struct FieldsCounter {
//!     count: usize,
//! }
//!
//! impl FieldsCounter {
//!     fn new() -> Self {
//!         Self { count: 0 }
//!     }
//! }
//!
//! impl<'ast> QueryVisitor<'ast> for FieldsCounter {
//!     fn visit_field(&mut self, node: &'ast Field) {
//!         self.count += 1
//!     }
//! }
//!
//! fn main() {
//!     let mut number_of_type = FieldsCounter::new();
//!
//!     let doc = parse_query(r#"
//!         query TestQuery {
//!             users {
//!                 id
//!                 country {
//!                     id
//!                 }
//!             }
//!         }
//!     "#).expect("Failed to parse query");
//!
//!     walk_document(&mut number_of_type, &doc);
//!
//!     assert_eq!(number_of_type.count, 2);
//! }
//! ```
//!
//! [`QueryVisitor`]: /graphql_parser/query/query_visitor/trait.QueryVisitor.html
//! [`walk_document`]: /graphql_parser/query/query_visitor/fn.walk_document.html

#![allow(unused_variables)]

use super::ast::*;

/// Trait for easy query syntax tree traversal.
///
/// See [module docs](/graphql_parser/query/query_visitor/index.html) for more info.
pub trait QueryVisitor<'ast, T: Text<'ast>> {
    fn visit_document(&mut self, node: &'ast Document<'ast, T>) {}

    fn visit_definition(&mut self, node: &'ast Definition<'ast, T>) {}

    fn visit_fragment_definition(&mut self, node: &'ast FragmentDefinition<'ast, T>) {}

    fn visit_operation_definition(&mut self, node: &'ast OperationDefinition<'ast, T>) {}

    fn visit_query(&mut self, node: &'ast Query<'ast, T>) {}

    fn visit_mutation(&mut self, node: &'ast Mutation<'ast, T>) {}

    fn visit_subscription(&mut self, node: &'ast Subscription<'ast, T>) {}

    fn visit_selection_set(&mut self, node: &'ast SelectionSet<'ast, T>) {}

    fn visit_variable_definition(&mut self, node: &'ast VariableDefinition<'ast, T>) {}

    fn visit_selection(&mut self, node: &'ast Selection<'ast, T>) {}

    fn visit_field(&mut self, node: &'ast Field<'ast, T>) {}

    fn visit_fragment_spread(&mut self, node: &'ast FragmentSpread<'ast, T>) {}

    fn visit_inline_fragment(&mut self, node: &'ast InlineFragment<'ast, T>) {}
}


/// Walk a query syntax tree and call the visitor methods for each type of node.
///
/// This function is how you should initiate a visitor.
pub fn walk_document<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Document<'ast, T>) {
    visitor.visit_document(node);
    for def in &node.definitions {
        walk_definition(visitor, def);
    }
}

fn walk_definition<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Definition<'ast, T>) {
    use super::ast::Definition::*;

    visitor.visit_definition(node);
    match node {
        Operation(inner) => {
            walk_operation_definition(visitor, inner);
        },
        Fragment(inner) => {
            walk_fragment_definition(visitor, inner);
        },
    }
}

fn walk_fragment_definition<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast FragmentDefinition<'ast, T>) {
    walk_selection_set(visitor, &node.selection_set);
}

fn walk_operation_definition<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast OperationDefinition<'ast, T>) {
    use super::ast::OperationDefinition::*;

    visitor.visit_operation_definition(node);
    match node {
        SelectionSet(inner) => {
            walk_selection_set(visitor, inner);
        }
        Query(inner) => {
            walk_query(visitor, inner);
        }
        Mutation(inner) => {
            walk_mutation(visitor, inner);
        }
        Subscription(inner) => {
            walk_subscription(visitor, inner);
        }
    }
}

fn walk_query<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Query<'ast, T>) {
    visitor.visit_query(node);

    for var_def in &node.variable_definitions {
        walk_variable_definition(visitor, var_def);
    }

    walk_selection_set(visitor, &node.selection_set);
}

fn walk_mutation<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Mutation<'ast, T>) {
    visitor.visit_mutation(node);

    for var_def in &node.variable_definitions {
        walk_variable_definition(visitor, var_def);
    }

    walk_selection_set(visitor, &node.selection_set);
}

fn walk_subscription<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Subscription<'ast, T>) {
    visitor.visit_subscription(node);

    for var_def in &node.variable_definitions {
        walk_variable_definition(visitor, var_def);
    }

    walk_selection_set(visitor, &node.selection_set);
}

fn walk_selection_set<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast SelectionSet<'ast, T>) {
    visitor.visit_selection_set(node);

    for selection in &node.items {
        walk_selection(visitor, selection);
    }
}

fn walk_variable_definition<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast VariableDefinition<'ast, T>) {
    visitor.visit_variable_definition(node)
}

fn walk_selection<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Selection<'ast, T>) {
    use super::ast::Selection::*;

    visitor.visit_selection(node);
    match node {
        Field(inner) => {
            walk_field(visitor, inner);
        }
        FragmentSpread(inner) => {
            walk_fragment_spread(visitor, inner);
        }
        InlineFragment(inner) => {
            walk_inline_fragment(visitor, inner);
        }
    }
}

fn walk_field<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast Field<'ast, T>) {
    visitor.visit_field(node)
}

fn walk_fragment_spread<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast FragmentSpread<'ast, T>) {
    visitor.visit_fragment_spread(node)
}

fn walk_inline_fragment<'ast, T: Text<'ast>, V: QueryVisitor<'ast, T>>(visitor: &mut V, node: &'ast InlineFragment<'ast, T>) {
    visitor.visit_inline_fragment(node);
    walk_selection_set(visitor, &node.selection_set);
}