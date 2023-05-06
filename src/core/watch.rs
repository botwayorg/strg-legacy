use super::{strg::check_dir, tools::return_path};
use git2::{BranchType, Cred, PushOptions, RemoteCallbacks, Repository};
use notify::*;
use owo_colors::OwoColorize;
use std::process::Command;
use std::{path::Path, time::Duration};

fn work(db: &String) {
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(&return_path(db))
        .output()
        .unwrap();

    let cmd2 = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("New Changes")
        .current_dir(&return_path(db))
        .output()
        .unwrap();

    println!("{}", String::from_utf8(cmd2.stdout).unwrap());
    println!("{}", String::from_utf8(cmd2.stderr).unwrap());

    let repo = Repository::open(&return_path(db)).unwrap();

    let branch = repo.head().unwrap();
    let branch_name = branch
        .shorthand()
        .expect("Failed to get current branch name");

    let upstream = repo.find_branch(&format!("origin/{}", branch_name), BranchType::Remote);
    if upstream.is_err() {
        let cmd = Command::new("git")
            .args(["push", "-u", "origin", "main"])
            .current_dir(&return_path(db))
            .output()
            .unwrap();

        println!("{}", String::from_utf8(cmd.stdout).unwrap());

        let cmd2 = Command::new("git")
            .arg("push")
            .current_dir(&return_path(db))
            .output()
            .unwrap();

        println!("{}", String::from_utf8(cmd2.stdout).unwrap());
    } else {
        let cmd = Command::new("git")
            .arg("push")
            .current_dir(&return_path(db))
            .output()
            .unwrap();

        println!("{}", String::from_utf8(cmd.stdout).unwrap());
    }

    println!("{}", "Changes Saved".bright_magenta());
}

pub fn watch(db: &String) {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        let config = Config::default().with_poll_interval(Duration::from_secs(1));

        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    let check_db_dir: bool = Path::new(return_path(db).as_str()).is_dir();

    if check_db_dir {
        println!("{}", "Directory found".bright_green());

        watcher
            .watch(
                Path::new(return_path(db).as_str()),
                RecursiveMode::Recursive,
            )
            .unwrap();

        for e in rx {
            if e.is_ok() {
                work(db);
            }
        }
    } else {
        check_dir(db, false);

        watcher
            .watch(
                Path::new(return_path(db).as_str()),
                RecursiveMode::Recursive,
            )
            .unwrap();

        for e in rx {
            if e.is_ok() {
                work(db);
            }
        }
    }
}
