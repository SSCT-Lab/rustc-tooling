use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

use regex::Regex;

struct FaultLoc {
    pub ident: String,
    pub line_num: usize,
    pub col_num: usize,
    pub file_path: PathBuf,
    pub is_dep: bool,
    pub depth: i32,
}

pub fn extract_backtrace(path: PathBuf) {
    let re = Regex::new(r"(\d+):\s+0x[0-9a-f]+ - (.+?)::(.+?)\s+at (/.+?):(\d+):(\d+)").unwrap();
    let file = File::open(path).expect("Failed to open backtrace file!");
    let reader = BufReader::new(file);
    let mut fault_locs: Vec<FaultLoc> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = re.captures(&line) {
            let file_path = PathBuf::from(&caps[4]);
            if !file_path.display().to_string().contains("/rustc") {
                let depth = caps[1].parse::<i32>().unwrap();
                let ident = caps[3].to_string();
                let line_num = caps[5].parse::<usize>().unwrap_or(0);
                let col_num = caps[6].parse::<usize>().unwrap_or(0);
                let is_dep = false;

                fault_locs.push(FaultLoc {
                    ident,
                    line_num,
                    col_num,
                    file_path,
                    is_dep,
                    depth,
                });
            }
        }
    }
}
