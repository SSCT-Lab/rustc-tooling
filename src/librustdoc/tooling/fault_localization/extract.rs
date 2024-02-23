use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

use regex::Regex;

use crate::tooling::{database::model::LocInfo, utils::{select_dep, select_loc_info, select_loc_info_by_id}};

#[derive(Clone)]
pub struct FaultLoc {
    pub ident: String,
    pub line_num: usize,
    pub col_num: usize,
    pub file_path: PathBuf,
    pub is_dep: bool,
    pub depth: i32,
}

impl FaultLoc {
    pub fn new(loc_info: LocInfo, is_dep: bool, depth: i32) -> Self {
        FaultLoc {
            ident: loc_info.ident,
            line_num: loc_info.line_num as usize,
            col_num: loc_info.col_num as usize,
            file_path: PathBuf::from(&loc_info.file_path),
            is_dep,
            depth,
        }
    }
}

pub fn find_dependencies(lhs: FaultLoc) -> Vec<FaultLoc> {
    let lhs_loc = select_loc_info(lhs.file_path.display().to_string(), lhs.line_num as i32, lhs.col_num as i32);
    let mut dependencies = Vec::new();

    let deps = select_dep(&lhs_loc);
    for dep in deps {
        let rhs_loc = select_loc_info_by_id(dep.rhs_id);
        let rhs_dep = FaultLoc::new(rhs_loc, true, lhs.depth);
        dependencies.push(rhs_dep);
    }

    dependencies
}

pub fn extract_backtrace(path: PathBuf) -> Vec<FaultLoc> {
    let re_line1 = Regex::new(r"(\d+):\s+0x[0-9a-f]+ - (.+?)::(.+?)$").unwrap();
    let re_line2 = Regex::new(r"^\s*at (/.+?):(\d+):(\d+)").unwrap();
    let file = File::open(path).expect("Failed to open backtrace file!");
    let reader = BufReader::new(file);
    let mut fault_locs: Vec<FaultLoc> = Vec::new();

    let mut lines = reader.lines();
    while let Some(Ok(line1)) = lines.next() {
        if let Some(caps) = re_line1.captures(&line1) {
            let depth = caps[1].parse::<i32>().unwrap();
            let full_ident = caps[3].to_string();
            let ident_parts: Vec<&str> = full_ident.split("::").collect();
            let ident = if let Some(first_part) = ident_parts.first() {
                first_part.to_string()
            } else {
                full_ident
            };

            if let Some(Ok(line2)) = lines.next() {
                if let Some(caps) = re_line2.captures(&line2) {
                    let file_path = PathBuf::from(&caps[1]);

                    if file_path.display().to_string().contains("/rustc") {
                        continue;
                    }

                    let line_num = caps[2].parse::<usize>().unwrap_or(0);
                    let col_num = caps[3].parse::<usize>().unwrap_or(0);
                    let is_dep = false;

                    let lhs = FaultLoc {
                        ident,
                        line_num,
                        col_num,
                        file_path,
                        is_dep,
                        depth,
                    };
                    fault_locs.push(lhs.clone());

                    let mut dependencies = find_dependencies(lhs.clone());
                    if !dependencies.is_empty() {
                        fault_locs.append(&mut dependencies);
                    }
                }
            }
        }
    }

    fault_locs
}
