use rustc_hir::def;
use rustc_middle::ty::TyCtxt;



#[allow(unused_variables)]
pub fn get_type_info(tcx: TyCtxt<'_>) {
    let hir_krate = tcx.hir();
    for id in hir_krate.items() {
        let item = id.owner_id.def_id;
        match tcx.def_kind(item) {
            def::DefKind::Fn => {
                //函数
                let fn_id = item.to_def_id().clone();
                let mir = tcx.optimized_mir(item);
                println!("{:?}", mir);
            }
            _ => {
                println!("mir other kind: {:?}", tcx.def_kind(item));
            }
        }
    }
}