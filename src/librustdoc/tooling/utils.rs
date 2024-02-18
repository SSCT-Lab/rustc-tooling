use std::path::PathBuf;

use rustc_span::{FileName, FileNameDisplayPreference};
use diesel::mysql::MysqlConnection;
use super::database::model::{NewDependency, NewLocInfo};
use super::database::schema::loc_info::dsl::*;
use super::database::schema::dependencies::dsl::*;
use diesel::sql_function;

use diesel::prelude::*;

use super::fault_localization::graph::DependencyGraph;

pub fn filename_to_pathbuf(file_name: &FileName) -> PathBuf {
    match file_name {
        FileName::Real(path) => PathBuf::from(path.to_string_lossy(FileNameDisplayPreference::Local).into_owned()),
        _ => PathBuf::new()
    }
}

pub fn get_connection() -> MysqlConnection {
    super::database::establish_connection()
}

pub fn insert_dependency_graph(graph: &DependencyGraph<'_>) {
    for (lhs, rhs_vec) in &graph.lhs_to_loc_info {
        let dep_lhs_id = insert_loc_info(NewLocInfo {
            ident: &lhs.ident,
            line_num: lhs.line_num as i32,
            col_num: lhs.col_num as i32,
            file_path: &lhs.file_path.to_string_lossy(),
        });

        for rhs in rhs_vec {
            let dep_rhs_id = insert_loc_info(NewLocInfo {
                ident: &rhs.ident,
                line_num: rhs.line_num as i32,
                col_num: rhs.col_num as i32,
                file_path: &rhs.file_path.to_string_lossy(),
            });

            insert_dependency(dep_lhs_id, dep_rhs_id);
        }
    }
}

sql_function! {
    #[sql_name = "LAST_INSERT_ID"]
    fn last_insert_id() -> Unsigned<Bigint>;
}

pub fn insert_loc_info(new_loc: NewLocInfo<'_>) -> i32 {
    let conn = &mut get_connection();

    diesel::insert_into(loc_info)
            .values(&new_loc)
            .execute(conn)
            .expect("Error when saving loc_info");

    let last_id: u64 = diesel::select(last_insert_id()).first(conn).expect("Error getting last insert ID");

    if last_id > i32::MAX as u64 {
        panic!("Last insert ID exceeds i32::MAX");
    }

    last_id as i32  
}

pub fn insert_dependency(dep_lhs_id: i32, dep_rhs_id: i32) {
    let conn = &mut get_connection();

    let new_dep = NewDependency {
        lhs_id: dep_lhs_id,
        rhs_id: dep_rhs_id,
    };

    diesel::insert_into(dependencies)
        .values(&new_dep)
        .execute(conn)
        .expect("Error inserting dependency");
}
