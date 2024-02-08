use std::path::PathBuf;
use std::rc::Rc;
use crate::clean::{Crate, Item};
use crate::config::RenderOptions;
use crate::error::Error;
use crate::formats::cache::Cache;
use crate::formats::FormatRenderer;
use rustc_middle::ty::TyCtxt;

#[derive(Clone)]
pub(crate) struct GraphRenderer<'tcx> {
    tcx: TyCtxt<'tcx>,
    out_path: PathBuf,
    cache: Rc<Cache>
}

impl <'tcx> GraphRenderer<'tcx> {

}

impl<'tcx> FormatRenderer<'tcx> for GraphRenderer<'tcx> {
    fn descr() -> &'static str {
        "graph"
    }

    const RUN_ON_MODULE: bool = false;

    fn init(krate: Crate, options: RenderOptions, cache: Cache, tcx: TyCtxt<'tcx>) -> Result<(Self, Crate), Error> {
        Ok((
            GraphRenderer {
                tcx,
                out_path: options.output,
                cache: Rc::new(cache),
            },
            krate
        ))
    }

    fn make_child_renderer(&self) -> Self {
        self.clone()
    }

    fn item(&mut self, item: Item) -> Result<(), Error> {
        todo!()
    }

    fn mod_item_in(&mut self, _item: &Item) -> Result<(), Error> {
        unreachable!("RUN_ON_MODULE = false should never call mod_item_in")
    }

    fn after_krate(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn cache(&self) -> &Cache {
        &self.cache
    }
}


