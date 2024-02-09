use std::collections::HashMap;
use std::path::PathBuf;

use rustc_middle::ty::TyCtxt;
use rustc_middle::hir::map::Map;
use rustc_session::Session;
use rustc_span::source_map::{SourceMap, Span};
use rustc_hir::intravisit::{Visitor, NestedVisitorMap};
use rustc_hir::{Expr, ExprKind};

#[derive(Debug)]
pub(crate) struct LocInfo {
    pub ident: String,
    pub line_num: usize,
    pub col_num: usize,
    pub file_path: PathBuf,
}

#[allow(dead_code)]
pub(crate) struct DependencyGraph<'tcx> {
    tcx: TyCtxt<'tcx>,
    hir: Map<'tcx>,
    lhs_to_loc_info: HashMap<LocInfo, Vec<LocInfo>>,
}

#[allow(dead_code)]
impl<'tcx> DependencyGraph<'tcx> {
    fn sess(&self) -> &'tcx Session {
        self.tcx.sess
    }

    fn source_map(&self) -> &SourceMap {
        self.sess().source_map()
    }
}

#[allow(dead_code)]
pub struct GraphVisitor<'tcx> {
    graph: DependencyGraph<'tcx>,
}

impl<'tcx> Visitor<'tcx> for GraphVisitor<'tcx> {
    fn visit_item(&mut self, item: &'tcx rustc_hir::Item<'tcx>) {
        rustc_hir::intravisit::walk_item(self, item);
        if let rustc_hir::ItemKind::Fn(.., body_id) = &item.kind {
            let body = self.graph.tcx.hir().body(*body_id);
            self.visit_body(body);
        }
    }

    fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) {
        if let ExprKind::Assign(lhs, rhs, _) = &ex.kind {
            let src_map = self.graph.source_map();

            
        }
        rustc_hir::intravisit::walk_expr(self, ex);
    }
}

#[allow(dead_code)]
pub fn analyze_dependencies(tcx: TyCtxt<'_>) {
    let hir = tcx.hir();
    let dependency_graph = DependencyGraph {
        tcx,
        hir,
        lhs_to_loc_info: HashMap::new(), // Initialize the map
    };

    let mut visitor = GraphVisitor {
        graph: dependency_graph,
    };

    // Visit all item likes in the crate
    tcx.hir().krate().visit_all_item_likes(&mut NestedVisitorMap::All(&mut visitor));
}
