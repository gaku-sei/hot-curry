// TODO: Add a "validation" visitor, that will checks that no include are used in templates, or if it's not too complex, allow for multiple files

use std::collections::HashSet;

use tera::ast::{Expr, ExprVal, WS};
use tera_visitor::VisitorMut as TeraVisitorMut;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct TeraVariableVisitor {
    idents: HashSet<String>,
    iterable_idents: HashSet<String>,
    optional_idents: HashSet<String>,
}

impl TeraVariableVisitor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn idents(&self) -> &HashSet<String> {
        &self.idents
    }

    pub fn sorted_idents(&mut self) -> Vec<String> {
        let mut idents = self.idents().iter().map(Clone::clone).collect::<Vec<_>>();

        idents.sort();

        idents
    }

    #[must_use]
    pub fn iterable_idents(&self) -> &HashSet<String> {
        &self.iterable_idents
    }

    pub fn sorted_iterable_idents(&mut self) -> Vec<String> {
        let mut iterable_idents = self
            .iterable_idents()
            .iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();

        iterable_idents.sort();

        iterable_idents
    }

    #[must_use]
    pub fn optional_idents(&self) -> &HashSet<String> {
        &self.optional_idents
    }

    pub fn sorted_optional_idents(&mut self) -> Vec<String> {
        let mut optional_idents = self
            .optional_idents()
            .iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();

        optional_idents.sort();

        optional_idents
    }
}

impl TeraVisitorMut for TeraVariableVisitor {
    fn visit_variable_block_mut(&mut self, ws: &WS, expr: &Expr) {
        if let ExprVal::Ident(ident) = &expr.val {
            self.idents.insert(ident.to_string());
        }

        tera_visitor::visit_variable_block_mut(self, ws, expr);
    }

    fn visit_forloop_expr_mut(&mut self, expr: &Expr) {
        if let ExprVal::Ident(ident) = &expr.val {
            self.idents.insert(ident.to_string());
            self.iterable_idents.insert(ident.to_string());
        }

        tera_visitor::visit_forloop_expr_mut(self, expr);
    }

    fn visit_if_expr_mut(&mut self, expr: &Expr) {
        if let ExprVal::Ident(ident) = &expr.val {
            self.idents.insert(ident.to_string());
            self.optional_idents.insert(ident.to_string());
        }

        tera_visitor::visit_if_expr_mut(self, expr);
    }

    fn visit_else_if_expr_mut(&mut self, expr: &Expr) {
        if let ExprVal::Ident(ident) = &expr.val {
            self.idents.insert(ident.to_string());
            self.optional_idents.insert(ident.to_string());
        }

        tera_visitor::visit_else_if_expr_mut(self, expr);
    }
}
