use std::path::PathBuf;
use syn::{punctuated::Punctuated, spanned::Spanned, ExprMethodCall};

use crate::tooling::fault_localization::extract::FaultLoc;
use crate::tooling::patch_generation::patterns::PATTERN;

use super::patterns::{AddType, ChangeType};

pub(crate) struct Transform {
    pub output_path: Option<PathBuf>,
    pub fault_locs: Vec<FaultLoc>
}

impl Transform {
    pub fn new(output_path: Option<PathBuf>, fault_locs: Vec<FaultLoc>) -> Self {
        Transform {
            output_path,
            fault_locs,
        }
    }

    pub fn transform(&self) {
        for fault_loc in &self.fault_locs {
            let file_content = std::fs::read_to_string(&fault_loc.file_path)
                .expect("Failed to read!");

            let mut syntax_tree = syn::parse_file(&file_content)
                .expect("Failed to parse file to syntax tree");

            let patterns: Vec<PATTERN> = vec![
                PATTERN::IndexMutate,
                PATTERN::IfPreAdd,
                PATTERN::IfPostAdd,
                PATTERN::IfCondChange,
                PATTERN::McAdd(AddType::AddAsBytes),
                PATTERN::McAdd(AddType::AddMax),
                PATTERN::McChange(ChangeType::ToSaturating),
                PATTERN::McChange(ChangeType::ToCheck),
                PATTERN::McChange(ChangeType::ToWrapping),
                PATTERN::McChange(ChangeType::ToFilterMap),
                PATTERN::McChange(ChangeType::ToUnwrap),
                PATTERN::McChange(ChangeType::ToUnwrapOrElse),
                PATTERN::McChange(ChangeType::ToUnwrapOrFault),
                PATTERN::McChange(ChangeType::ToExtendFromSlice),
            ];

            for pattern in patterns {
                let mut visitor = AstVisitor::new(fault_loc, pattern);
                syn::visit_mut::visit_file_mut(&mut visitor, &mut syntax_tree);

                let new_code = prettyplease::unparse(&syntax_tree);

                let new_file_path = self.output_path.as_ref().unwrap_or_else(|| {
                    panic!("Output path must be specified!");
                });
     
                std::fs::write(new_file_path, new_code)
                    .expect("Failed to write to file!");
            }
        }
    }
}

pub struct AstVisitor<'ast> {
    fault_loc: &'ast FaultLoc,
    fix_pattern: PATTERN,
}

impl<'ast> AstVisitor<'ast> {
    fn new(fault_loc: &'ast FaultLoc, fix_pattern: PATTERN) -> Self {
        AstVisitor {
            fault_loc,
            fix_pattern,
        }
    }

    fn get_loc_num(&self) -> (i32, i32) {
        (self.fault_loc.line_num as i32, self.fault_loc.col_num as i32)
    }

    fn get_mc_idents(&self, expr: &ExprMethodCall) -> Vec<syn::Ident> {
        let mut idents = Vec::new();
        idents.push(expr.method.clone());

        let mut current_expr = &*expr.receiver;
        while let syn::Expr::MethodCall(inner_expr) = current_expr {
            idents.push(inner_expr.method.clone());
            current_expr = &*inner_expr.receiver;
        }

        idents
    }
}

impl<'ast> syn::visit_mut::VisitMut for AstVisitor<'ast> {
    fn visit_file_mut(&mut self, f: &mut syn::File) {
        syn::visit_mut::visit_file_mut(self, f);
    }

    fn visit_expr_binary_mut(&mut self, i: &mut syn::ExprBinary) {
        let span = &i.span();
        let start = span.start().line;
        let end = span.end().line;

        if self.get_loc_num().0 <= end as i32 && self.get_loc_num().0 >= start as i32 {
            match &self.fix_pattern {
                PATTERN::McChange(ChangeType::ToSaturating) => {
                    match i.op {
                        syn::BinOp::Add(_) => {
                            let saturating_add = syn::Expr::MethodCall(syn::ExprMethodCall {
                                attrs: Vec::new(),
                                receiver: i.left.clone(),
                                dot_token: syn::token::Dot::default(),
                                method: syn::Ident::new("saturating_add", i.op.span()),
                                turbofish: None,
                                paren_token: Default::default(),
                                args: Punctuated::from_iter(vec![*i.right.clone()]),
                            });

                            *i = syn::ExprBinary {
                                attrs: Vec::new(),
                                left: Box::new(syn::Expr::Lit(syn::ExprLit {
                                    attrs: Vec::new(),
                                    lit: syn::Lit::Int(syn::LitInt::new("0", i.span()))
                                })),
                                op: syn::BinOp::Add(Default::default()),
                                right: Box::new(saturating_add),
                            }
                        },
                        syn::BinOp::AddAssign(_) => {
                            let saturating_add = syn::Expr::MethodCall(syn::ExprMethodCall {
                                attrs: Vec::new(),
                                receiver: i.left.clone(),
                                dot_token: syn::token::Dot::default(),
                                method: syn::Ident::new("saturating_add", i.op.span()),
                                turbofish: None,
                                paren_token: Default::default(),
                                args: Punctuated::from_iter(vec![*i.right.clone()]),
                            });

                            *i = syn::ExprBinary {
                                attrs: Vec::new(),
                                left: Box::new(syn::Expr::Lit(syn::ExprLit {
                                    attrs: Vec::new(),
                                    lit: syn::Lit::Int(syn::LitInt::new("0", i.span()))
                                })),
                                op: syn::BinOp::Add(Default::default()),
                                right: Box::new(saturating_add),
                            }
                        },
                        syn::BinOp::Sub(_) => {
                            let saturating_add = syn::Expr::MethodCall(syn::ExprMethodCall {
                                attrs: Vec::new(),
                                receiver: i.left.clone(),
                                dot_token: syn::token::Dot::default(),
                                method: syn::Ident::new("saturating_sub", i.op.span()),
                                turbofish: None,
                                paren_token: Default::default(),
                                args: Punctuated::from_iter(vec![*i.right.clone()]),
                            });

                            *i = syn::ExprBinary {
                                attrs: Vec::new(),
                                left: Box::new(syn::Expr::Lit(syn::ExprLit {
                                    attrs: Vec::new(),
                                    lit: syn::Lit::Int(syn::LitInt::new("0", i.span()))
                                })),
                                op: syn::BinOp::Add(Default::default()),
                                right: Box::new(saturating_add),
                            }
                        },
                        syn::BinOp::SubAssign(_) => {
                            let saturating_add = syn::Expr::MethodCall(syn::ExprMethodCall {
                                attrs: Vec::new(),
                                receiver: i.left.clone(),
                                dot_token: syn::token::Dot::default(),
                                method: syn::Ident::new("saturating_sub", i.op.span()),
                                turbofish: None,
                                paren_token: Default::default(),
                                args: Punctuated::from_iter(vec![*i.right.clone()]),
                            });

                            *i = syn::ExprBinary {
                                attrs: Vec::new(),
                                left: Box::new(syn::Expr::Lit(syn::ExprLit {
                                    attrs: Vec::new(),
                                    lit: syn::Lit::Int(syn::LitInt::new("0", i.span()))
                                })),
                                op: syn::BinOp::Add(Default::default()),
                                right: Box::new(saturating_add),
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    fn visit_expr_index_mut(&mut self, i: &mut syn::ExprIndex) {
        // println!("visit_expr_index_mut!");
        let span = &i.span();
        let start = span.start().line;
        let end = span.end().line;

        if self.get_loc_num().0 <= end as i32 && self.get_loc_num().0 >= start as i32 {
            match &self.fix_pattern {
                PATTERN::IndexMutate => {
                    let span = i.index.span();

                    if let syn::Expr::Binary(expr_binary) = i.index.as_mut() {
                        let new_expr = syn::Expr::Binary(syn::ExprBinary {
                            attrs: Vec::new(),
                            left: Box::new(syn::Expr::Binary(expr_binary.clone())),
                            op: syn::BinOp::Sub(syn::token::Minus::default()),
                            right: Box::new(syn::Expr::Lit(syn::ExprLit {
                                attrs: Vec::new(),
                                lit: syn::Lit::Int(syn::LitInt::new("1", span))
                            })),
                        });

                        i.index = Box::new(new_expr); 
                    }

                    if let syn::Expr::Range(expr_range) = i.index.as_mut() {
                        let new_start_expr = match expr_range.start.take() {
                            Some(start_expr) => {
                                syn::Expr::Binary(syn::ExprBinary {
                                    attrs: Vec::new(),
                                    left: start_expr,
                                    op: syn::BinOp::Sub(syn::token::Minus::default()),
                                    right: Box::new(syn::Expr::Lit(syn::ExprLit {
                                        attrs: Vec::new(),
                                        lit: syn::Lit::Int(syn::LitInt::new("1", span)),
                                    })),
                                })
                            }
                            None => {
                                syn::Expr::Binary(syn::ExprBinary {
                                    attrs: Vec::new(),
                                    left: Box::new(syn::Expr::Lit(syn::ExprLit {
                                        attrs: Vec::new(),
                                        lit: syn::Lit::Int(syn::LitInt::new("0", span)),
                                    })),
                                    op: syn::BinOp::Sub(syn::token::Minus::default()),
                                    right: Box::new(syn::Expr::Lit(syn::ExprLit {
                                        attrs: Vec::new(),
                                        lit: syn::Lit::Int(syn::LitInt::new("0", span)),
                                    })),
                                })
                            }
                        };
                    
                        expr_range.start = Some(Box::new(new_start_expr));
                    }
                },
                _ => {}
            }
        }
    }

    fn visit_expr_method_call_mut(&mut self, i: &mut syn::ExprMethodCall) {
        let span = &i.span();
        let start = span.start().line;
        let end = span.end().line;

        if self.get_loc_num().0 <= end as i32 && self.get_loc_num().0 >= start as i32 {
            match &self.fix_pattern {
                PATTERN::McAdd(add_type) => {
                    match add_type {
                        AddType::AddAsBytes => {
                            if i.method.to_string() == "add" {
                                i.method = syn::Ident::new("as_bytes", i.method.span());
                            }
                        },
                        AddType::AddMax => {
                            if i.method.to_string() == "add" {
                                i.method = syn::Ident::new("max", i.method.span());
                            }
                        },
                    }
                }, 
                PATTERN::McChange(change_type) => {
                    match change_type {
                        ChangeType::ToFilterMap => {
                            if i.method.to_string() == "map" {
                                if let syn::Expr::Closure(ref mut expr_closure) = *i.args.first_mut().unwrap() {
                                    if let syn::Expr::MethodCall(ref mut expr_mc) = expr_closure.body.as_mut() {
                                        let mut idents: Vec<syn::Ident> = self.get_mc_idents(&expr_mc);
                                        idents.reverse();

                                        println!("{:?}", idents);

                                        let unwrap_index = idents.iter().position(|ident| ident.to_string() == "unwrap");
                                        idents = match unwrap_index {
                                            Some(index) => {
                                                idents.into_iter().skip(index + 1).collect()
                                            },
                                            None => {
                                                return;
                                            }
                                        };

                                        let closure_arg = syn::Pat::Ident(syn::PatIdent {
                                            attrs: Vec::new(),
                                            by_ref: None,
                                            mutability: None,
                                            ident: syn::Ident::new("a", i.method.span()),
                                            subpat: None,
                                        });

                                        let mut closure_body = syn::Expr::MethodCall(syn::ExprMethodCall {
                                            attrs: Vec::new(),
                                            receiver: Box::new(syn::Expr::Path(syn::ExprPath {
                                                attrs: Vec::new(),
                                                qself: None,
                                                path: syn::Path {
                                                    leading_colon: None,
                                                    segments: syn::punctuated::Punctuated::from_iter(vec![
                                                        syn::PathSegment {
                                                            ident: syn::Ident::new("a", i.method.span()),
                                                            arguments: syn::PathArguments::None,
                                                        }
                                                    ]),
                                                },
                                            })),
                                            dot_token: syn::token::Dot::default(),
                                            method: idents[0].clone(),
                                            turbofish: None,
                                            paren_token: Default::default(),
                                            args: Punctuated::new(),
                                        });

                                        for ident in &idents[1..] {
                                            let new_closure_body = syn::Expr::MethodCall(syn::ExprMethodCall {
                                                attrs: Vec::new(),
                                                receiver: Box::new(closure_body),
                                                dot_token: syn::token::Dot::default(),
                                                method: ident.clone(),
                                                turbofish: None,
                                                paren_token: Default::default(),
                                                args: Punctuated::new(),
                                            });

                                            closure_body = new_closure_body;
                                        }

                                        let closure = syn::Expr::Closure(syn::ExprClosure {
                                            attrs: Vec::new(),
                                            lifetimes: None,
                                            constness: None,
                                            movability: None,
                                            asyncness: None,
                                            capture: None,
                                            or1_token: Default::default(),
                                            inputs: syn::punctuated::Punctuated::from_iter(vec![closure_arg]),
                                            or2_token: Default::default(),
                                            output: syn::ReturnType::Default,
                                            body: Box::new(closure_body),
                                        });

                                        let mut tmp_expr_mc = expr_mc.clone();
                                        while let syn::Expr::MethodCall(ref mut inner_expr) = tmp_expr_mc.receiver.as_mut() {
                                            if inner_expr.method.to_string() == "unwrap" {
                                                inner_expr.method = syn::Ident::new("map", inner_expr.method.span());
                                                inner_expr.args.clear();
                                                inner_expr.args.push(closure);

                                                expr_closure.body = Box::new(syn::Expr::MethodCall(inner_expr.clone()));

                                                break;
                                            }

                                            tmp_expr_mc = inner_expr.clone();
                                        }
                                    }
                                }

                                i.method = syn::Ident::new("filter_map", i.method.span());
                            }
                        },
                        ChangeType::ToExtendFromSlice => {
                            if i.method.to_string() == "copy_from_slice" {
                                i.method = syn::Ident::new("extend_from_slice", i.method.span());
                            }
                        },
                        ChangeType::ToSaturating => {
                            if i.method.to_string() == "abs" {
                                i.method = syn::Ident::new("saturating_abs", i.method.span());
                            }
                        },
                        ChangeType::ToUnwrapOrElse => {
                            if i.method.to_string() == "expect" {
                                i.method = syn::Ident::new("unwrap_or_else", i.method.span());

                                let closure_body = syn::Expr::MethodCall(syn::ExprMethodCall {
                                    attrs: Vec::new(),
                                    receiver: Box::new(syn::Expr::Lit(syn::ExprLit {
                                        attrs: Vec::new(),
                                        lit: syn::Lit::Int(syn::LitInt::new("1", i.method.span())),
                                    })),
                                    dot_token: syn::token::Dot::default(),
                                    method: syn::Ident::new("into", i.method.span()),
                                    turbofish: None,
                                    paren_token: Default::default(),
                                    args: Punctuated::new(),
                                });

                                let closure = syn::Expr::Closure(syn::ExprClosure {
                                    attrs: Vec::new(),
                                    lifetimes: None,
                                    constness: None,
                                    movability: None,
                                    asyncness: None,
                                    capture: None,
                                    or1_token: Default::default(),
                                    inputs: syn::punctuated::Punctuated::new(),
                                    or2_token: Default::default(),
                                    output: syn::ReturnType::Default,
                                    body: Box::new(closure_body),
                                });

                                i.args.clear();
                                i.args.push(closure);

                                println!("{:?}", i.args);
                            }
                        },
                        ChangeType::ToUnwrapOrFault => {
                            if i.method.to_string() == "expect" {
                                i.method = syn::Ident::new("unwrap_or_default", i.method.span());
                                i.args.clear();
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        syn::visit_mut::visit_expr_method_call_mut(self, i);
    }
}
