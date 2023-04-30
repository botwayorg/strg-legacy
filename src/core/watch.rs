use super::{strg::check_dir, tools::return_path};
use notify::*;
use owo_colors::OwoColorize;
use std::{path::Path, time::Duration};

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
                println!("{:?}", e)
            }
        }
    } else {
        check_dir(db);

        watcher
            .watch(
                Path::new(return_path(db).as_str()),
                RecursiveMode::Recursive,
            )
            .unwrap();

        for e in rx {
            if e.is_ok() {
                // gitx(db);
                println!(
                    "{}",
                    &(".".to_owned() + &db.to_string().to_owned() + " is Changed")
                )
            }
        }
    }
}
