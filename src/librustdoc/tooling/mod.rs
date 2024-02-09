use rustc_middle::ty::TyCtxt;
use rustc_middle::hir::map::Map;
use rustc_session::Session;
use rustc_span::source_map::SourceMap;
use rustc_hir::intravisit::Visitor;
use rustc_hir::Expr;
use rustc_hir::ExprKind;

#[allow(dead_code)]
pub(crate) struct DependencyGraph<'tcx> {
    tcx: TyCtxt<'tcx>,
    hir: Map<'tcx>,
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
        if let ExprKind::Assign(_lhs, _rhs, _) = &ex.kind {
            println!("this is assign!");
        }
        rustc_hir::intravisit::walk_expr(self, ex);
    }
}

#[allow(dead_code)]
pub fn analyze_dependencies(tcx: TyCtxt<'_>) {
    let hir = tcx.hir();
    let dependency_graph = DependencyGraph {
        tcx,
        hir
    };

    let mut visitor = GraphVisitor {
        graph: dependency_graph,
    };

    tcx.hir().visit_all_item_likes_in_crate(&mut visitor);
}
