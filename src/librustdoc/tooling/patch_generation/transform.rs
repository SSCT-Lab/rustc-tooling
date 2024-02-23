use std::path::PathBuf;
use syn::spanned::Spanned;

use crate::tooling::fault_localization::extract::FaultLoc;

#[allow(unused)]
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

            let syntax_tree = syn::parse_file(&file_content)
                .expect("Failed to parse file to syntax tree");

            let mut visitor = AstVisitor::new(fault_loc);
            syn::visit_mut::visit_file_mut(&mut visitor, &mut syntax_tree.clone());
        }
    }
}

#[allow(unused)]
pub struct AstVisitor<'ast> {
    fault_loc: &'ast FaultLoc,
}

impl<'ast> AstVisitor<'ast> {
    fn new(fault_loc: &'ast FaultLoc) -> Self {
        AstVisitor {
            fault_loc,
        }
    }
}

impl<'ast> syn::visit_mut::VisitMut for AstVisitor<'ast> {
    fn visit_file_mut(&mut self, f: &mut syn::File) {
        syn::visit_mut::visit_file_mut(self, f);
    }

    fn visit_expr_mut(&mut self, e: &mut syn::Expr) {
        // println!("visit expr!");
        match e {
            syn::Expr::Assign(expr_assign) => {
                // println!("  visit expr assign!");
                let span = &expr_assign.left.span();
                let start = span.start();
                let end = span.end();
                println!("{:?}-{:?}", start, end);
            }
            _ => {}
        }
        syn::visit_mut::visit_expr_mut(self, e);
    }

    fn visit_local_mut(&mut self, local: &mut syn::Local) {
        // println!("  visit local!");
        let span = &local.span();
        let start = span.start();
        let end = span.end();
        println!("{:?}-{:?}", start, end);

        syn::visit_mut::visit_local_mut(self, local);
    }
}