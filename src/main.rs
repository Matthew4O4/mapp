use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Add;
use std::time::Duration;
use crossbeam_channel::{bounded, Receiver, select, tick};
use human_panic::setup_panic;
use tabled::{Table, Tabled};
use walkdir::DirEntry;
use crate::AppType::JavaApp;
use std::process::{Command, id};
use std::str::Split;

mod cmdline;


fn main() -> Result<(), exitfailure::ExitFailure> {
    setup_panic!();
    test();
    let table = get_thread_source(OSType::LINUX(AppType::JavaApp));
    println!("{}", Table::new(table.thread_vec));
    Ok(())
}

fn test() {
    let mut ls_path_command = String::from("netstat -antup |grep LISTEN | grep 26517 | awk -F ':' '{if($4<20000) {print $4}}'");
    // println!("{}", ls_path_command);
    let mut cmd = Command::new("sh");
    let output = cmd
        .arg("-c")
        .arg(ls_path_command)
        .output()
        .expect("failed to execute process");
    let vec_content = output.stdout;
    let id_cont: String = String::from_utf8(vec_content).unwrap();
    // println!("{}", id_cont);
}

fn sh_get_thread_vec() {}


fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}


fn get_thread_source(os_type: OSType) -> ThreadVec {
    match os_type {
        OSType::LINUX(app_type) => {
            get_app_info(app_type)
        }
    }
}

/// 获取所有 app_info by Type
fn get_app_info(os_type: AppType) -> ThreadVec {
    match os_type {
        AppType::JavaApp => {
            get_java_app_vec_info()
        }
    }
}


fn get_java_app_vec_info() -> ThreadVec {
    let mut ids = get_pid_vec();
    let mut thread_source_vec = Vec::with_capacity(ids.len());
    for id in ids {
        thread_source_vec.push(get_java_app_info(id));
    }
    ThreadVec { thread_vec: thread_source_vec }
}

/// 获取pid 集合
fn get_pid_vec() -> Vec<String> {
    let mut ls_path_command = String::from("grep  '^java$' /proc/*/comm  | cut -d '/'  -f 3");
    // println!("{}", ls_path_command);
    let mut cmd = Command::new("sh");
    let output = cmd
        .arg("-c")
        .arg(ls_path_command)
        .output()
        .expect("failed to execute process");
    let vec_content = output.stdout;
    // println!("{:?}", vec_content);
    let id_cont: String = String::from_utf8(vec_content).unwrap();
    // println!("{}", id_cont);
    id_cont.lines().map(|l| String::from(l))
        .collect::<Vec<_>>()
}


fn get_java_app_info(id: String) -> ThreadSource {
    let app_package_name: String;
    let start_command_line = get_start_command_line(&id);
    let port: String;
    let package_dir_path = get_package_dir_path(&id);
    ThreadSource {
        pid: id,
        app_package_name: "".to_string(),
        start_command_line,
        port: "".to_string(),
        package_dir_path,
    }
}

/// 获取包名
fn get_package_dir_path(id: &String) -> String {
    let mut ls_path_command = String::from("ls -l /proc/");
    let path = String::from(id);
    let grep_command = String::from(" | grep 'cwd ->' | grep -v 'grep' | awk '{print $NF}'");
    ls_path_command = ls_path_command.add(&*path).add(&*grep_command);
    // println!("{}", ls_path_command);
    let mut cmd = Command::new("sh");
    let output = cmd
        .arg("-c")
        .arg(ls_path_command)
        .output()
        .expect("failed to execute process");
    let vec_content = output.stdout;
    let result = String::from_utf8(vec_content).unwrap().trim().to_string();
    // println!("{}", result);
    result
}

/// 获取启动参数
fn get_start_command_line(id: &String) -> String {
    let mut ls_path_command = String::from(" ps -ef  | grep ");
    let path = String::from(id);
    let grep_command = String::from(" | grep -v 'grep'| awk '{print substr($0, index($0,$8))}'");
    ls_path_command = ls_path_command.add(&*path).add(&*grep_command);
    // println!("{}", ls_path_command);
    let mut cmd = Command::new("sh");
    let output = cmd
        .arg("-c")
        .arg(ls_path_command)
        .output()
        .expect("failed to execute process");
    let vec_content = output.stdout;
    let mut result = String::from_utf8(vec_content).unwrap().trim().to_string();
    if result.len() > 100 {
        result = "超出了长度限制，最多支持100个字符".to_string();
    }
    // println!("{}", result);
    result
}
/// 获取端口号


struct ThreadVec {
    thread_vec: Vec<ThreadSource>,
}

#[derive(Debug, Tabled)]
struct ThreadSource {
    pid: String,
    app_package_name: String,
    package_dir_path: String,
    port: String,
    start_command_line: String,
}

enum OSType {
    LINUX(AppType),
}

enum AppType {
    JavaApp,
}

fn get_current_os_type() -> OSType {
    todo!()
}


