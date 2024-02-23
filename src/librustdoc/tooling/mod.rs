mod utils;
mod database;
mod fault_localization;
mod patch_generation;

use std::{path::PathBuf, time::Instant};
use rustc_data_structures::fx::FxHashMap;
use rustc_middle::ty::TyCtxt;
use fault_localization::graph::{DependencyGraph, GraphVisitor};

use crate::tooling::patch_generation::transform::Transform;

pub fn analyze_dependencies(tcx: TyCtxt<'_>) {
    let hir = tcx.hir();
    let mut dependency_graph: DependencyGraph<'_> = DependencyGraph {
        tcx,
        hir,
        lhs_to_loc_info: FxHashMap::default(), // Initialize the map
    };

    let mut visitor = GraphVisitor::new(&mut dependency_graph);

    // Visit all item likes in the crate
    // tcx.hir().visit_all_item_likes_in_crate(&mut visitor);
    tcx.hir().walk_toplevel_module(&mut visitor);

    let start_time = Instant::now();

    println!("Generate dependency graph...");
    for (lhs, rhs_vec) in &visitor.graph.lhs_to_loc_info {
        println!("LHS: {:?}", lhs);
        for rhs in rhs_vec {
            println!("\tRHS: {:?}", rhs);
        }
    }
    utils::insert_dependency_graph(&mut dependency_graph);

    let elapsed_time = start_time.elapsed().as_secs();
    println!("Finish generating dependency graph! Elapsed time: {:?}", elapsed_time);

    let output_path = Some(PathBuf::from("test.txt"));
    let keys_vec: Vec<_> = dependency_graph.lhs_to_loc_info.keys().cloned().collect();
    let transform = Transform::new(output_path, keys_vec);
    transform.transform();
}
