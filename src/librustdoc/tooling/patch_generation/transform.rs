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

            let mut syntax_tree = syn::parse_file(&file_content)
                .expect("Failed to parse file to syntax tree");

            let mut visitor = AstVisitor::new(fault_loc);
            syn::visit_mut::visit_file_mut(&mut visitor, &mut syntax_tree);

            println!("Finish visiting ast!");

            let new_code = prettyplease::unparse(&syntax_tree);

            let new_file_path = self.output_path.as_ref().unwrap_or_else(|| {
                panic!("Output path must be specified!");
            });
 
            std::fs::write(new_file_path, new_code)
                .expect("Failed to write to file!");
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

    fn get_loc_num(&self) -> (i32, i32) {
        (self.fault_loc.line_num as i32, self.fault_loc.col_num as i32)
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
                let start = span.start().line;
                let end = span.end().line;
                
                if self.get_loc_num().0 <= end as i32 && self.get_loc_num().0 >= start as i32 {
                    println!("{start} <= {} <= {end}", self.get_loc_num().0);
                    let rhs = &expr_assign.right;

                    let modified_rhs = syn::parse_quote! {
                        #rhs - 1
                    };

                    expr_assign.right = Box::new(modified_rhs);

                    return;
                }
            },
            _ => {}
        }
        syn::visit_mut::visit_expr_mut(self, e);
    }

    // fn visit_local_mut(&mut self, local: &mut syn::Local) {
    //     // println!("  visit local!");
    //     let span = &local.span();
    //     let start = span.start().line;
    //     let end = span.end().line;
    //     println!("{:?}-{:?}", start, end);

    //     syn::visit_mut::visit_local_mut(self, local);
    // }
}
