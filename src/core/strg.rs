use super::tools::{get_home_dir, return_path};
use super::watch::watch;
use owo_colors::OwoColorize;
use std::fs;
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
            match fs::create_dir(&return_path(db)) {
                Ok(_) => println!("{}", "Directory created successfully.".bright_green()),
                Err(e) => println!("Error creating directory: {:?}", e),
            }

            Command::new("git")
                .args(["init"])
                .current_dir(&return_path(db))
                .output()
                .unwrap();

            Command::new("git")
                .args(["branch", "-m", "main"])
                .current_dir(&return_path(db))
                .output()
                .unwrap();

            let cmd = Command::new("gh")
                .args([
                    "repo",
                    "create",
                    &dbx.to_string(),
                    "--private",
                    "--source",
                    &return_path(db),
                ])
                .current_dir(get_home_dir())
                .output()
                .unwrap();

            let gh = String::from_utf8(cmd.stdout)
                .unwrap()
                .trim_end()
                .to_string();

            watch(db);

            println!("{}", gh.bright_green());
        } else {
            let _ = Command::new("gh")
                .args(["repo", "clone", &dbx.to_string(), &return_path(db)])
                .output()
                .unwrap();

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

pub fn init() {
    Command::new("gh")
        .args(["auth", "setup-git"])
        .output()
        .unwrap();

    Command::new("wget")
        .arg("https://raw.githubusercontent.com/botwayorg/strg/main/package.json")
        .output()
        .unwrap();

    Command::new("wget")
        .arg("https://raw.githubusercontent.com/botwayorg/strg/main/turbo.json")
        .output()
        .unwrap();

    match fs::create_dir("./runner") {
        Ok(_) => {
            println!("{}", "Directory created successfully.".bright_green());

            Command::new("wget")
                .arg("https://raw.githubusercontent.com/botwayorg/strg/main/runner/package.json")
                .current_dir("./runner")
                .output()
                .unwrap();
        }
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    match fs::create_dir("./runner/cmd") {
        Ok(_) => {
            println!("{}", "Directory created successfully.".bright_green());

            Command::new("wget")
                .arg("https://raw.githubusercontent.com/botwayorg/strg/main/runner/cmd/package.json")
                .current_dir("./runner/cmd")
                .output()
                .unwrap();
        }
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}
