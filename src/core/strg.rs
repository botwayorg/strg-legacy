use super::tools::return_path;
use super::watch::watch;
use owo_colors::OwoColorize;
use std::path::Path;
use std::process::Command;

pub fn check_dir(db: &String) {
    println!("{}", "Checking Directory...".yellow());

    let dbx = &(".".to_owned() + &db.to_string().to_owned());

    let check_db_dir: bool = Path::new(return_path(db).as_str()).is_dir();

    let user = std::env::var("RAILWAY_GIT_REPO_OWNER")
        .expect("RAILWAY_GIT_REPO_OWNER env variable is required");

    let check_repo = String::from_utf8(
        Command::new("gh")
            .arg("api")
            .arg(format!("repos/{}/.{}", user, db))
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    if !check_db_dir {
        if check_repo.contains("Not Found") {
            Command::new("cd").arg(return_path(db));

            let cmd = Command::new("gh")
                .args(["repo", "create", &dbx.to_string(), "--private", "--clone"])
                .output()
                .unwrap();
            let gh = String::from_utf8(cmd.stdout)
                .unwrap()
                .trim_end()
                .to_string();

            watch(db);

            println!("{}", gh.bright_green());
        } else {
            let cmd = Command::new("gh")
                .args(["repo", "clone", &dbx.to_string(), &return_path(db)])
                .output()
                .unwrap();
            String::from_utf8(cmd.stdout)
                .unwrap()
                .trim_end()
                .to_string();

            println!("{}", "Cloned Successfully".bright_green());

            watch(db);
        }
    } else {
        println!("{}", "Directory found".bright_green());

        watch(db);
    }
}

pub fn sync(db: &String) {
    watch(db);
}
