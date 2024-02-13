mod graph;
mod fault_localization;
mod utils;

use rustc_data_structures::fx::FxHashMap;

use rustc_middle::ty::TyCtxt;
use graph::{DependencyGraph, GraphVisitor};

pub fn analyze_dependencies(tcx: TyCtxt<'_>) {
    let hir = tcx.hir();
    let dependency_graph = DependencyGraph {
        tcx,
        hir,
        lhs_to_loc_info: FxHashMap::default(), // Initialize the map
    };

    let mut visitor = GraphVisitor::new(dependency_graph);

    // Visit all item likes in the crate
    tcx.hir().visit_all_item_likes_in_crate(&mut visitor);

    println!("Dependency Graph:");
    for (lhs, rhs_vec) in &visitor.graph.lhs_to_loc_info {
        println!("LHS: {:?}", lhs);
        for rhs in rhs_vec {
            println!("\tRHS: {:?}", rhs);
        }
    }
}
