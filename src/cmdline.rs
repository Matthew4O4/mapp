use std::fs::File;
use std::io::Read;
use std::ops::Add;
use walkdir::{DirEntry, WalkDir};

pub fn pid() -> Vec<DirEntry> {
    // cmd_str可以是从输入流读取或从文件里读取
    // 读取并判断 comm 是否为 target Type
    ids()
}


fn ids() -> Vec<DirEntry> {
    let wal = WalkDir::new("/proc").max_depth(1);

    let mut ids: Vec<DirEntry> = Vec::new();

    for entry in wal {
        let entry = entry.unwrap();
        let path_name = entry.file_name().to_str().unwrap();
        match path_name.parse::<i32>() {
            Ok(a) => {
                let comm_path = String::from(entry.path().to_str().unwrap()).add("/comm");
                let mut comm = File::open(comm_path).unwrap();
                let mut con = String::new();
                // 所有应用名称
                comm.read_to_string(&mut con).unwrap();
                // 将JAVA 应用信息放好
                println!("{}", con);
                if con.as_str() == "java"  {
                    ids.push(entry);
                }
            }
            Err(_) => {
                ()
            }
        }
    }
    return ids;
}

