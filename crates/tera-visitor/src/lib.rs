#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use tera::ast::{
    Block, Expr, ExprVal, FilterSection, Forloop, FunctionCall, If, In, LogicExpr, MacroCall,
    MacroDefinition, MathExpr, Node, Set, StringConcat, Test, WS,
};

pub type Ast = Vec<Node>;

pub trait Visitor {
    // Whole AST
    fn visit_ast(&self, ast: &Ast) {
        visit_ast(self, ast);
    }

    /// A call to `{{ super() }}` in a block
    fn visit_super(&self) {
        visit_super(self);
    }

    /// Some actual text
    fn visit_text(&self, text: &str) {
        visit_text(self, text);
    }

    /// A `{{ }}` block
    fn visit_variable_block(&self, ws: &WS, expr: &Expr) {
        visit_variable_block(self, ws, expr);
    }

    /// A `{% macro hello() %}...{% endmacro %}`
    fn visit_macro_definition(
        &self,
        left_ws: &WS,
        macro_definition: &MacroDefinition,
        right_ws: &WS,
    ) {
        visit_macro_definition(self, left_ws, macro_definition, right_ws);
    }

    /// The `{% extends "blabla.html" %}` node, contains the template name
    fn visit_extends(&self, ws: &WS, content: &str) {
        visit_extends(self, ws, content);
    }

    /// The `{% include "blabla.html" %}` node, contains the template name
    fn visit_include(&self, ws: &WS, includes: &[String], ignore_missing: bool) {
        visit_include(self, ws, includes, ignore_missing);
    }

    /// The `{% import "macros.html" as macros %}`
    fn visit_import_macro(&self, ws: &WS, path: &str, name: &str) {
        visit_import_macro(self, ws, path, name);
    }

    /// The `{% set val = something %}` tag
    fn visit_set(&self, ws: &WS, set: &Set) {
        visit_set(self, ws, set);
    }

    /// The text between `{% raw %}` and `{% endraw %}`
    fn visit_raw(&self, left_ws: &WS, content: &str, right_ws: &WS) {
        visit_raw(self, left_ws, content, right_ws);
    }

    /// A filter section node `{{ filter name(param="value") }} content {{ endfilter }}`
    fn visit_filter_section(&self, left_ws: &WS, filter_section: &FilterSection, right_ws: &WS) {
        visit_filter_section(self, left_ws, filter_section, right_ws);
    }

    /// A `{% block name %}...{% endblock %}`
    fn visit_block(&self, left_ws: &WS, block: &Block, right_ws: &WS) {
        visit_block(self, left_ws, block, right_ws);
    }

    /// A `{% for i in items %}...{% endfor %}`
    fn visit_forloop(&self, left_ws: &WS, forloop: &Forloop, right_ws: &WS) {
        visit_forloop(self, left_ws, forloop, right_ws);
    }

    fn visit_forloop_expr(&self, expr: &Expr) {
        visit_forloop_expr(self, expr);
    }

    /// A if/elif/else block, WS for the if/elif/else is directly in the struct
    fn visit_if(&self, if_: &If, ws: &WS) {
        visit_if(self, if_, ws);
    }

    fn visit_if_expr(&self, expr: &Expr) {
        visit_if_expr(self, expr);
    }

    fn visit_else_if_expr(&self, expr: &Expr) {
        visit_else_if_expr(self, expr);
    }

    /// The `{% break %}` tag
    fn visit_break(&self, ws: &WS) {
        visit_break(self, ws);
    }

    /// The `{% continue %}` tag
    fn visit_continue(&self, ws: &WS) {
        visit_continue(self, ws);
    }

    /// The `{# #} `comment tag and its content
    fn visit_comment(&self, ws: &WS, content: &str) {
        visit_comment(self, ws, content);
    }

    fn visit_expr(&self, expr: &Expr) {
        visit_expr(self, expr);
    }

    fn visit_expr_val(&self, expr_val: &ExprVal) {
        visit_expr_val(self, expr_val);
    }

    fn visit_string_expr_val(&self, s: &str) {
        visit_string_expr_val(self, s);
    }

    fn visit_int_expr_val(&self, i: i64) {
        visit_int_expr_val(self, i);
    }

    fn visit_float_expr_val(&self, f: f64) {
        visit_float_expr_val(self, f);
    }

    fn visit_bool_expr_val(&self, b: bool) {
        visit_bool_expr_val(self, b);
    }

    fn visit_ident_expr_val(&self, ident: &str) {
        visit_ident_expr_val(self, ident);
    }

    fn visit_math_expr_val(&self, math_expr: &MathExpr) {
        visit_math_expr_val(self, math_expr);
    }

    fn visit_logic_expr_val(&self, logic_expr: &LogicExpr) {
        visit_logic_expr_val(self, logic_expr);
    }

    fn visit_test_expr_val(&self, test: &Test) {
        visit_test_expr_val(self, test);
    }

    fn visit_array_expr_val(&self, array: &Vec<Expr>) {
        visit_array_expr_val(self, array);
    }

    fn visit_string_concat_expr_val(&self, string_concat: &StringConcat) {
        visit_string_concat_expr_val(self, string_concat);
    }

    fn visit_macro_call(&self, macro_call: &MacroCall) {
        visit_macro_call(self, macro_call);
    }

    fn visit_function_call(&self, function_call: &FunctionCall) {
        visit_function_call(self, function_call);
    }

    fn visit_in_expr_val(&self, in_: &In) {
        visit_in_expr_val(self, in_);
    }
}

// Whole AST
pub fn visit_ast<V: Visitor + ?Sized>(visitor: &V, ast: &Ast) {
    for node in ast {
        match node {
            Node::Super => visitor.visit_super(),
            Node::Text(text) => visitor.visit_text(text),
            Node::VariableBlock(ws, expr) => visitor.visit_variable_block(ws, expr),
            Node::MacroDefinition(left_ws, macro_definition, right_ws) => {
                visitor.visit_macro_definition(left_ws, macro_definition, right_ws);
            }
            Node::Extends(ws, content) => visitor.visit_extends(ws, content),
            Node::Include(ws, includes, ignore_missing) => {
                visitor.visit_include(ws, includes, *ignore_missing);
            }
            Node::ImportMacro(ws, path, name) => visitor.visit_import_macro(ws, path, name),
            Node::Set(ws, set) => visitor.visit_set(ws, set),
            Node::Raw(left_ws, content, right_ws) => visitor.visit_raw(left_ws, content, right_ws),
            Node::FilterSection(left_ws, filter_section, right_ws) => {
                visitor.visit_filter_section(left_ws, filter_section, right_ws);
            }
            Node::Block(left_ws, block, right_ws) => visitor.visit_block(left_ws, block, right_ws),
            Node::Forloop(left_ws, forloop, right_ws) => {
                visitor.visit_forloop(left_ws, forloop, right_ws);
            }
            Node::If(if_, ws) => visitor.visit_if(if_, ws),
            Node::Break(ws) => visitor.visit_break(ws),
            Node::Continue(ws) => visitor.visit_continue(ws),
            Node::Comment(ws, content) => visitor.visit_comment(ws, content),
        }
    }
}

/// A call to `{{ super() }}` in a block
pub fn visit_super<V: Visitor + ?Sized>(_visitor: &V) {}

/// Some actual text
pub fn visit_text<V: Visitor + ?Sized>(_visitor: &V, _text: &str) {}

/// A `{{ }}` block
pub fn visit_variable_block<V: Visitor + ?Sized>(visitor: &V, _ws: &WS, expr: &Expr) {
    visitor.visit_expr(expr);
}

/// A `{% macro hello() %}...{% endmacro %}`
pub fn visit_macro_definition<V: Visitor + ?Sized>(
    visitor: &V,
    _left_ws: &WS,
    macro_definition: &MacroDefinition,
    _right_ws: &WS,
) {
    visitor.visit_ast(&macro_definition.body);

    for expr in macro_definition.args.values().flatten() {
        visitor.visit_expr(expr);
    }
}

/// The `{% extends "blabla.html" %}` node, contains the template name
pub fn visit_extends<V: Visitor + ?Sized>(_visitor: &V, _ws: &WS, _content: &str) {}

/// The `{% include "blabla.html" %}` node, contains the template name
pub fn visit_include<V: Visitor + ?Sized>(
    _visitor: &V,
    _ws: &WS,
    _includes: &[String],
    _ignore_missing: bool,
) {
}

/// The `{% import "macros.html" as macros %}`
pub fn visit_import_macro<V: Visitor + ?Sized>(_visitor: &V, _ws: &WS, _path: &str, _name: &str) {}

/// The `{% set val = something %}` tag
pub fn visit_set<V: Visitor + ?Sized>(visitor: &V, _ws: &WS, set: &Set) {
    visitor.visit_expr(&set.value);
}

/// The text between `{% raw %}` and `{% endraw %}`
pub fn visit_raw<V: Visitor + ?Sized>(_visitor: &V, _left_ws: &WS, _content: &str, _right_ws: &WS) {
}

/// A filter section node `{{ filter name(param="value") }} content {{ endfilter }}`
pub fn visit_filter_section<V: Visitor + ?Sized>(
    visitor: &V,
    _left_ws: &WS,
    filter_section: &FilterSection,
    _right_ws: &WS,
) {
    visitor.visit_ast(&filter_section.body);
    visitor.visit_function_call(&filter_section.filter);
}

/// A `{% block name %}...{% endblock %}`
pub fn visit_block<V: Visitor + ?Sized>(visitor: &V, _left_ws: &WS, block: &Block, _right_ws: &WS) {
    visitor.visit_ast(&block.body);
}

/// A `{% for i in items %}...{% endfor %}`
pub fn visit_forloop<V: Visitor + ?Sized>(
    visitor: &V,
    _left_ws: &WS,
    forloop: &Forloop,
    _right_ws: &WS,
) {
    visitor.visit_forloop_expr(&forloop.container);
    visitor.visit_ast(&forloop.body);

    if let Some(empty_body) = &forloop.empty_body {
        visitor.visit_ast(empty_body);
    }
}

pub fn visit_forloop_expr<V: Visitor + ?Sized>(visitor: &V, expr: &Expr) {
    visitor.visit_expr(expr);
}

/// A if/elif/else block, WS for the if/elif/else is directly in the struct
pub fn visit_if<V: Visitor + ?Sized>(visitor: &V, if_: &If, _ws: &WS) {
    for (index, (_, expr, ast)) in if_.conditions.iter().enumerate() {
        if index == 0 {
            visitor.visit_if_expr(expr);
        } else {
            visitor.visit_else_if_expr(expr);
        }

        visitor.visit_ast(ast);
    }

    if let Some((_, ast)) = &if_.otherwise {
        visitor.visit_ast(ast);
    }
}

pub fn visit_if_expr<V: Visitor + ?Sized>(visitor: &V, expr: &Expr) {
    visitor.visit_expr(expr);
}

pub fn visit_else_if_expr<V: Visitor + ?Sized>(visitor: &V, expr: &Expr) {
    visitor.visit_expr(expr);
}

/// The `{% break %}` tag
pub fn visit_break<V: Visitor + ?Sized>(_visitor: &V, _ws: &WS) {}

/// The `{% continue %}` tag
pub fn visit_continue<V: Visitor + ?Sized>(_visitor: &V, _ws: &WS) {}

/// The `{# #} `comment tag and its content
pub fn visit_comment<V: Visitor + ?Sized>(_visitor: &V, _ws: &WS, _content: &str) {}

pub fn visit_expr<V: Visitor + ?Sized>(visitor: &V, expr: &Expr) {
    for function_call in &expr.filters {
        visitor.visit_function_call(function_call);
    }

    visitor.visit_expr_val(&expr.val);
}

pub fn visit_expr_val<V: Visitor + ?Sized>(visitor: &V, expr_val: &ExprVal) {
    match expr_val {
        ExprVal::String(s) => visitor.visit_string_expr_val(s),
        ExprVal::Int(i) => visitor.visit_int_expr_val(*i),
        ExprVal::Float(f) => visitor.visit_float_expr_val(*f),
        ExprVal::Bool(b) => visitor.visit_bool_expr_val(*b),
        ExprVal::Ident(ident) => visitor.visit_ident_expr_val(ident),
        ExprVal::Math(math_expr) => visitor.visit_math_expr_val(math_expr),
        ExprVal::Logic(logic_expr) => visitor.visit_logic_expr_val(logic_expr),
        ExprVal::Test(test) => visitor.visit_test_expr_val(test),
        ExprVal::MacroCall(macro_call) => visitor.visit_macro_call(macro_call),
        ExprVal::FunctionCall(function_call) => visitor.visit_function_call(function_call),
        ExprVal::Array(array) => visitor.visit_array_expr_val(array),
        ExprVal::StringConcat(string_concat) => visitor.visit_string_concat_expr_val(string_concat),
        ExprVal::In(in_) => visitor.visit_in_expr_val(in_),
    }
}

pub fn visit_string_expr_val<V: Visitor + ?Sized>(_visitor: &V, _s: &str) {}

pub fn visit_int_expr_val<V: Visitor + ?Sized>(_visitor: &V, _i: i64) {}

pub fn visit_float_expr_val<V: Visitor + ?Sized>(_visitor: &V, _f: f64) {}

pub fn visit_bool_expr_val<V: Visitor + ?Sized>(_visitor: &V, _b: bool) {}

pub fn visit_ident_expr_val<V: Visitor + ?Sized>(_visitor: &V, _ident: &str) {}

pub fn visit_math_expr_val<V: Visitor + ?Sized>(_visitor: &V, _math_expr: &MathExpr) {
    // TODO: Visit lhs/rhs
}

pub fn visit_logic_expr_val<V: Visitor + ?Sized>(_visitor: &V, _logic_expr: &LogicExpr) {
    // TODO: Visit lhs/rhs
}

pub fn visit_test_expr_val<V: Visitor + ?Sized>(visitor: &V, test: &Test) {
    for expr in &test.args {
        visitor.visit_expr(expr);
    }
}

pub fn visit_array_expr_val<V: Visitor + ?Sized>(visitor: &V, array: &Vec<Expr>) {
    for expr in array {
        visitor.visit_expr(expr);
    }
}

pub fn visit_string_concat_expr_val<V: Visitor + ?Sized>(
    _visitor: &V,
    _string_concat: &StringConcat,
) {
}

pub fn visit_macro_call<V: Visitor + ?Sized>(visitor: &V, macro_call: &MacroCall) {
    for expr in macro_call.args.values() {
        visitor.visit_expr(expr);
    }
}

pub fn visit_function_call<V: Visitor + ?Sized>(visitor: &V, function_call: &FunctionCall) {
    for expr in function_call.args.values() {
        visitor.visit_expr(expr);
    }
}

pub fn visit_in_expr_val<V: Visitor + ?Sized>(_visitor: &V, _in_: &In) {
    // TODO: Visit lhs/rhs
}

pub trait VisitorMut {
    // Whole AST
    fn visit_ast_mut(&mut self, ast: &Ast) {
        visit_ast_mut(self, ast);
    }

    /// A call to `{{ super() }}` in a block
    fn visit_super_mut(&mut self) {
        visit_super_mut(self);
    }

    /// Some actual text
    fn visit_text_mut(&mut self, text: &str) {
        visit_text_mut(self, text);
    }

    /// A `{{ }}` block
    fn visit_variable_block_mut(&mut self, ws: &WS, expr: &Expr) {
        visit_variable_block_mut(self, ws, expr);
    }

    /// A `{% macro hello() %}...{% endmacro %}`
    fn visit_macro_definition_mut(
        &mut self,
        left_ws: &WS,
        macro_definition: &MacroDefinition,
        right_ws: &WS,
    ) {
        visit_macro_definition_mut(self, left_ws, macro_definition, right_ws);
    }

    /// The `{% extends "blabla.html" %}` node, contains the template name
    fn visit_extends_mut(&mut self, ws: &WS, content: &str) {
        visit_extends_mut(self, ws, content);
    }

    /// The `{% include "blabla.html" %}` node, contains the template name
    fn visit_include_mut(&mut self, ws: &WS, includes: &[String], ignore_missing: bool) {
        visit_include_mut(self, ws, includes, ignore_missing);
    }

    /// The `{% import "macros.html" as macros %}`
    fn visit_import_macro_mut(&mut self, ws: &WS, path: &str, name: &str) {
        visit_import_macro_mut(self, ws, path, name);
    }

    /// The `{% set val = something %}` tag
    fn visit_set_mut(&mut self, ws: &WS, set: &Set) {
        visit_set_mut(self, ws, set);
    }

    /// The text between `{% raw %}` and `{% endraw %}`
    fn visit_raw_mut(&mut self, left_ws: &WS, content: &str, right_ws: &WS) {
        visit_raw_mut(self, left_ws, content, right_ws);
    }

    /// A filter section node `{{ filter name(param="value") }} content {{ endfilter }}`
    fn visit_filter_section_mut(
        &mut self,
        left_ws: &WS,
        filter_section: &FilterSection,
        right_ws: &WS,
    ) {
        visit_filter_section_mut(self, left_ws, filter_section, right_ws);
    }

    /// A `{% block name %}...{% endblock %}`
    fn visit_block_mut(&mut self, left_ws: &WS, block: &Block, right_ws: &WS) {
        visit_block_mut(self, left_ws, block, right_ws);
    }

    /// A `{% for i in items %}...{% endfor %}`
    fn visit_forloop_mut(&mut self, left_ws: &WS, forloop: &Forloop, right_ws: &WS) {
        visit_forloop_mut(self, left_ws, forloop, right_ws);
    }

    fn visit_forloop_expr_mut(&mut self, expr: &Expr) {
        visit_forloop_expr_mut(self, expr);
    }

    /// A if/elif/else block, WS for the if/elif/else is directly in the struct
    fn visit_if_mut(&mut self, if_: &If, ws: &WS) {
        visit_if_mut(self, if_, ws);
    }

    fn visit_if_expr_mut(&mut self, expr: &Expr) {
        visit_if_expr_mut(self, expr);
    }

    fn visit_else_if_expr_mut(&mut self, expr: &Expr) {
        visit_else_if_expr_mut(self, expr);
    }

    /// The `{% break %}` tag
    fn visit_break_mut(&mut self, ws: &WS) {
        visit_break_mut(self, ws);
    }

    /// The `{% continue %}` tag
    fn visit_continue_mut(&mut self, ws: &WS) {
        visit_continue_mut(self, ws);
    }

    /// The `{# #} `comment tag and its content
    fn visit_comment_mut(&mut self, ws: &WS, content: &str) {
        visit_comment_mut(self, ws, content);
    }

    fn visit_expr_mut(&mut self, expr: &Expr) {
        visit_expr_mut(self, expr);
    }

    fn visit_expr_val_mut(&mut self, expr_val: &ExprVal) {
        visit_expr_val_mut(self, expr_val);
    }

    fn visit_string_expr_val_mut(&mut self, s: &str) {
        visit_string_expr_val_mut(self, s);
    }

    fn visit_int_expr_val_mut(&mut self, i: i64) {
        visit_int_expr_val_mut(self, i);
    }

    fn visit_float_expr_val_mut(&mut self, f: f64) {
        visit_float_expr_val_mut(self, f);
    }

    fn visit_bool_expr_val_mut(&mut self, b: bool) {
        visit_bool_expr_val_mut(self, b);
    }

    fn visit_ident_expr_val_mut(&mut self, ident: &str) {
        visit_ident_expr_val_mut(self, ident);
    }

    fn visit_math_expr_val_mut(&mut self, math_expr: &MathExpr) {
        visit_math_expr_val_mut(self, math_expr);
    }

    fn visit_logic_expr_val_mut(&mut self, logic_expr: &LogicExpr) {
        visit_logic_expr_val_mut(self, logic_expr);
    }

    fn visit_test_expr_val_mut(&mut self, test: &Test) {
        visit_test_expr_val_mut(self, test);
    }

    fn visit_array_expr_val_mut(&mut self, array: &Vec<Expr>) {
        visit_array_expr_val_mut(self, array);
    }

    fn visit_string_concat_expr_val_mut(&mut self, string_concat: &StringConcat) {
        visit_string_concat_expr_val_mut(self, string_concat);
    }

    fn visit_macro_call_mut(&mut self, macro_call: &MacroCall) {
        visit_macro_call_mut(self, macro_call);
    }

    fn visit_function_call_mut(&mut self, function_call: &FunctionCall) {
        visit_function_call_mut(self, function_call);
    }

    fn visit_in_expr_val_mut(&mut self, in_: &In) {
        visit_in_expr_val_mut(self, in_);
    }
}

// Whole AST
pub fn visit_ast_mut<V: VisitorMut + ?Sized>(visitor: &mut V, ast: &Ast) {
    for node in ast {
        match node {
            Node::Super => visitor.visit_super_mut(),
            Node::Text(text) => visitor.visit_text_mut(text),
            Node::VariableBlock(ws, expr) => visitor.visit_variable_block_mut(ws, expr),
            Node::MacroDefinition(left_ws, macro_definition, right_ws) => {
                visitor.visit_macro_definition_mut(left_ws, macro_definition, right_ws);
            }
            Node::Extends(ws, content) => visitor.visit_extends_mut(ws, content),
            Node::Include(ws, includes, ignore_missing) => {
                visitor.visit_include_mut(ws, includes, *ignore_missing);
            }
            Node::ImportMacro(ws, path, name) => visitor.visit_import_macro_mut(ws, path, name),
            Node::Set(ws, set) => visitor.visit_set_mut(ws, set),
            Node::Raw(left_ws, content, right_ws) => {
                visitor.visit_raw_mut(left_ws, content, right_ws);
            }
            Node::FilterSection(left_ws, filter_section, right_ws) => {
                visitor.visit_filter_section_mut(left_ws, filter_section, right_ws);
            }
            Node::Block(left_ws, block, right_ws) => {
                visitor.visit_block_mut(left_ws, block, right_ws);
            }
            Node::Forloop(left_ws, forloop, right_ws) => {
                visitor.visit_forloop_mut(left_ws, forloop, right_ws);
            }
            Node::If(if_, ws) => visitor.visit_if_mut(if_, ws),
            Node::Break(ws) => visitor.visit_break_mut(ws),
            Node::Continue(ws) => visitor.visit_continue_mut(ws),
            Node::Comment(ws, content) => visitor.visit_comment_mut(ws, content),
        }
    }
}

/// A call to `{{ super() }}` in a block
pub fn visit_super_mut<V: VisitorMut + ?Sized>(_visitor: &mut V) {}

/// Some actual text
pub fn visit_text_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _text: &str) {}

/// A `{{ }}` block
pub fn visit_variable_block_mut<V: VisitorMut + ?Sized>(visitor: &mut V, _ws: &WS, expr: &Expr) {
    visitor.visit_expr_mut(expr);
}

/// A `{% macro hello() %}...{% endmacro %}`
pub fn visit_macro_definition_mut<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    _left_ws: &WS,
    macro_definition: &MacroDefinition,
    _right_ws: &WS,
) {
    visitor.visit_ast_mut(&macro_definition.body);

    for expr in macro_definition.args.values().flatten() {
        visitor.visit_expr_mut(expr);
    }
}

/// The `{% extends "blabla.html" %}` node, contains the template name
pub fn visit_extends_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _ws: &WS, _content: &str) {}

/// The `{% include "blabla.html" %}` node, contains the template name
pub fn visit_include_mut<V: VisitorMut + ?Sized>(
    _visitor: &mut V,
    _ws: &WS,
    _includes: &[String],
    _ignore_missing: bool,
) {
}

/// The `{% import "macros.html" as macros %}`
pub fn visit_import_macro_mut<V: VisitorMut + ?Sized>(
    _visitor: &mut V,
    _ws: &WS,
    _path: &str,
    _name: &str,
) {
}

/// The `{% set val = something %}` tag
pub fn visit_set_mut<V: VisitorMut + ?Sized>(visitor: &mut V, _ws: &WS, set: &Set) {
    visitor.visit_expr_mut(&set.value);
}

/// The text between `{% raw %}` and `{% endraw %}`
pub fn visit_raw_mut<V: VisitorMut + ?Sized>(
    _visitor: &mut V,
    _left_ws: &WS,
    _content: &str,
    _right_ws: &WS,
) {
}

/// A filter section node `{{ filter name(param="value") }} content {{ endfilter }}`
pub fn visit_filter_section_mut<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    _left_ws: &WS,
    filter_section: &FilterSection,
    _right_ws: &WS,
) {
    visitor.visit_ast_mut(&filter_section.body);
    visitor.visit_function_call_mut(&filter_section.filter);
}

/// A `{% block name %}...{% endblock %}`
pub fn visit_block_mut<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    _left_ws: &WS,
    block: &Block,
    _right_ws: &WS,
) {
    visitor.visit_ast_mut(&block.body);
}

/// A `{% for i in items %}...{% endfor %}`
pub fn visit_forloop_mut<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    _left_ws: &WS,
    forloop: &Forloop,
    _right_ws: &WS,
) {
    visitor.visit_forloop_expr_mut(&forloop.container);
    visitor.visit_ast_mut(&forloop.body);

    if let Some(empty_body) = &forloop.empty_body {
        visitor.visit_ast_mut(empty_body);
    }
}

pub fn visit_forloop_expr_mut<V: VisitorMut + ?Sized>(visitor: &mut V, expr: &Expr) {
    visitor.visit_expr_mut(expr);
}

/// A if/elif/else block, WS for the if/elif/else is directly in the struct
pub fn visit_if_mut<V: VisitorMut + ?Sized>(visitor: &mut V, if_: &If, _ws: &WS) {
    for (index, (_, expr, ast)) in if_.conditions.iter().enumerate() {
        if index == 0 {
            visitor.visit_if_expr_mut(expr);
        } else {
            visitor.visit_else_if_expr_mut(expr);
        }

        visitor.visit_ast_mut(ast);
    }

    if let Some((_, ast)) = &if_.otherwise {
        visitor.visit_ast_mut(ast);
    }
}

pub fn visit_if_expr_mut<V: VisitorMut + ?Sized>(visitor: &mut V, expr: &Expr) {
    visitor.visit_expr_mut(expr);
}

pub fn visit_else_if_expr_mut<V: VisitorMut + ?Sized>(visitor: &mut V, expr: &Expr) {
    visitor.visit_expr_mut(expr);
}

/// The `{% break %}` tag
pub fn visit_break_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _ws: &WS) {}

/// The `{% continue %}` tag
pub fn visit_continue_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _ws: &WS) {}

/// The `{# #} `comment tag and its content
pub fn visit_comment_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _ws: &WS, _content: &str) {}

pub fn visit_expr_mut<V: VisitorMut + ?Sized>(visitor: &mut V, expr: &Expr) {
    for function_call in &expr.filters {
        visitor.visit_function_call_mut(function_call);
    }

    visitor.visit_expr_val_mut(&expr.val);
}

pub fn visit_expr_val_mut<V: VisitorMut + ?Sized>(visitor: &mut V, expr_val: &ExprVal) {
    match expr_val {
        ExprVal::String(s) => visitor.visit_string_expr_val_mut(s),
        ExprVal::Int(i) => visitor.visit_int_expr_val_mut(*i),
        ExprVal::Float(f) => visitor.visit_float_expr_val_mut(*f),
        ExprVal::Bool(b) => visitor.visit_bool_expr_val_mut(*b),
        ExprVal::Ident(ident) => visitor.visit_ident_expr_val_mut(ident),
        ExprVal::Math(math_expr) => visitor.visit_math_expr_val_mut(math_expr),
        ExprVal::Logic(logic_expr) => visitor.visit_logic_expr_val_mut(logic_expr),
        ExprVal::Test(test) => visitor.visit_test_expr_val_mut(test),
        ExprVal::MacroCall(macro_call) => visitor.visit_macro_call_mut(macro_call),
        ExprVal::FunctionCall(function_call) => visitor.visit_function_call_mut(function_call),
        ExprVal::Array(array) => visitor.visit_array_expr_val_mut(array),
        ExprVal::StringConcat(string_concat) => {
            visitor.visit_string_concat_expr_val_mut(string_concat);
        }
        ExprVal::In(in_) => visitor.visit_in_expr_val_mut(in_),
    }
}

pub fn visit_string_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _s: &str) {}

pub fn visit_int_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _i: i64) {}

pub fn visit_float_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _f: f64) {}

pub fn visit_bool_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _b: bool) {}

pub fn visit_ident_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _ident: &str) {}

pub fn visit_math_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _math_expr: &MathExpr) {
    // TODO: Visit lhs/rhs
}

pub fn visit_logic_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _logic_expr: &LogicExpr) {
    // TODO: Visit lhs/rhs
}

pub fn visit_test_expr_val_mut<V: VisitorMut + ?Sized>(visitor: &mut V, test: &Test) {
    for expr in &test.args {
        visitor.visit_expr_mut(expr);
    }
}

pub fn visit_array_expr_val_mut<V: VisitorMut + ?Sized>(visitor: &mut V, array: &Vec<Expr>) {
    for expr in array {
        visitor.visit_expr_mut(expr);
    }
}

pub fn visit_string_concat_expr_val_mut<V: VisitorMut + ?Sized>(
    _visitor: &mut V,
    _string_concat: &StringConcat,
) {
}

pub fn visit_macro_call_mut<V: VisitorMut + ?Sized>(visitor: &mut V, macro_call: &MacroCall) {
    for expr in macro_call.args.values() {
        visitor.visit_expr_mut(expr);
    }
}

pub fn visit_function_call_mut<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    function_call: &FunctionCall,
) {
    for expr in function_call.args.values() {
        visitor.visit_expr_mut(expr);
    }
}

pub fn visit_in_expr_val_mut<V: VisitorMut + ?Sized>(_visitor: &mut V, _in_: &In) {
    // TODO: Visit lhs/rhs
}
