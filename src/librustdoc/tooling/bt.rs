use std::collections::HashMap;
use std::collections::HashSet;
use std::Vec;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use super::database::model::{LocInfo,Dependency};
use  super::utils::{select_locInfo,select_Dep,select_LocInfo_by_id};
#[derive(Serialize, Deserialize)]
struct LocInfoWithDep {
    pub ident: String,
    pub line_num: usize,
    pub col_num: usize,
    pub file_path: PathBuf,
    pub dep: option<Vec<LocInfoWithDep>>,
}

pub fn create_LocInfoWithDep(newloc:LocInfo)->LocInfoWithDep
{
    let depVec=select_Dep(LocInfo);
    let count=0;
    let buf:Vec<LocInfoWithDep>=Vec::new();
    for dep in depVec
    {
        let rLocInfo=select_LocInfo_by_id(dep.rhs_id);
        let newLIWD=create_LocInfoWithDep(rLocInfo);
        buf.push(newLIWD);
    }
    if (count==0)
    {
        let newquery= LocInfoWithDep{
            ident:temp.ident,
            line_num:temp.line_num,
            col_num:temp.col_num,
            file_path:temp.file_path,
            dep:None,
        };
        newquery
    }
    else
    {
        let newquery= LocInfoWithDep{
            ident:temp.ident,
            line_num:temp.line_num,
            col_num:temp.col_num,
            file_path:temp.file_path,
            dep:Some(buf),
        };
        newquery
    }
}

pub fn extract_backtrace(path:PathBuf)
{
    let jsonpath="./backtrace_info.json";
    let json_file = OpenOptions::new()
                            .append(true)
                            .create(true) // 新建，若文件存在则打开这个文件
                            .open(jsonpath)
                            .expect("json file open fails");
    let _file = File::open(path).expect("fail to open backtrace file");
    let mut newline = String::new();
    let buf = io::BufReader::new(_file);
    let re=Regex::new(r"(/.*)+\.rs:[0-9]+:[0-9]").unwrap();
    let mark: HashSet<i32> =  HashSet::new();
    for line in buf.lines() {
        let newline = line.unwrap();
        for cap in re.captures_iter(&newline)
        {
            let strt=&cap[0][0..6];
            if strt.to_string().ne("/rustc")
            {
                println!("{}\n", &cap[0]);
                let tempset: Vec<&str> = cap[0].split(':').collect();
                if (tempset.len()==3)
                {
                    /*println!("{}\n", tempset[0]);
                    println!("{}\n", tempset[1]);
                    println!("{}\n", tempset[2]);*/
                    let filepath=tempset[0];
                    //let num: i32 = input_num.trim().parse().unwrap();
                    let line:i32=tempset[1].trim().parse().unwrap();
                    let col:i32=tempset[2].trim().parse().unwrap();
                    let temp=select_locInfo(filepath,line,col);
                    if !mark.contains(temp.id)
                    {
                        let newquery=create_LocInfoWithDep(temp);
                        mark.insert(temp.id);
                        let newjson=serde_json::to_string(&newquery).unwrap();
                        write!(json_file,newjson).expect("json write fails");
                    }
                }
                else
                {
                    ;
                }
            }
        }
    }
}