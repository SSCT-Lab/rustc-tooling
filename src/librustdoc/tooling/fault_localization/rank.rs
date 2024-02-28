use super::extract::FaultLoc;

pub struct RankList {
    pub rk_list: Vec<FaultLoc>
}

impl RankList {
    pub fn new(fault_locs: Vec<FaultLoc>) -> Self {
        RankList {
            rk_list: fault_locs
        }
    }

    fn tune_depth(&mut self) {
        self.rk_list.sort_by_key(|fault_loc| fault_loc.depth);

        for (index, fault_loc) in self.rk_list.iter_mut().enumerate() {
            fault_loc.depth = (index + 1) as i32;
        }
    }

    pub fn rank(&mut self) -> Vec<FaultLoc> {
        self.tune_depth();
        self.rk_list.clone()
    }
}