use clap::Parser;
use std::process::Command;
use anyhow::Result;

#[derive(Parser, Debug)]
struct Cli {
    /// Deletes all branches merged into the branch specified.
    #[clap(short, long, value_parser)]
    branch: Option<String>,

    /// Add branches to your git repo that should never be removed.
    #[clap(short, long, value_parser)] //, default_value = "master, staging")]
    add_protected_branches: Option<String>,

    /// Displays all branches merged into the branch specified.
    #[clap(short, long, value_parser)]
    show_merged: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args {
        Cli { branch: Some(branch), .. }                   => rip_merged_branches(&branch), 
        Cli { add_protected_branches: Some(branches), .. } => add_protected_branches(&branches),
        Cli { show_merged: Some(branch), .. }              => show_merged_branches(&branch),
        _ => println!("You must provide an option of some sort. Check --help for more information")
    };
}

fn rip_merged_branches(branch: &str) {
    let branches = filtered_merged_branches(branch);
    for name in branches.iter() {
        delete_branch(name);
    }
}

fn show_merged_branches(branch: &str) {
    let branches = filtered_merged_branches(branch);
    for name in branches.iter() {
        println!("{}", name.trim());
    }
}

fn filtered_merged_branches(branch: &str) -> Vec<String> {
    let local                = local_branch().unwrap();
    let merged               = merged_branches(branch).unwrap();
    let protected            = protected_branches().unwrap();
    let protected: Vec<&str> = protected.split(',').map(|x| x.trim()).collect();

   return merged
            .iter()
            .filter(|x| !protected.iter().any(|y| x.contains(y)))
            .filter(|x| !x.contains(&local))
            .cloned()
            .collect();
}

fn delete_branch(branch: &str) {
    Command::new("git")
            .arg("branch")
            .arg("-D")
            .arg(&branch.trim())
            .output()
            .expect("Unable to remove branch");
}

fn local_branch() -> Result<String> {
    let local = Command::new("git")
                        .arg("branch")
                        .arg("--show-current")
                        .output()
                        .expect("failed to retrieve local branch");
    let string = String::from_utf8(local.stdout)?;
    Ok(string)
}

fn merged_branches(branch: &str) -> Result<Vec<String>> {
    let branches = Command::new("git")
                           .arg("branch")
                           .arg("--merged")
                           .arg(branch)
                           .output()
                           .expect("failed to execute process");
    let branches = String::from_utf8(branches.stdout).unwrap();
    let branches: Vec<&str> = branches.lines().collect();

    Ok(branches.iter().map(|x| x.to_string()).collect())
}

// git config --add x-protected-branch.merge-branch 'staging, master
fn protected_branches() -> Result<String> {
    let branches = Command::new("git")
                           .arg("config")
                           .arg("--get")
                           .arg("x-protected-branch.merge-branch")
                           .output()
                           .expect("failed to execute process");
    let branches = String::from_utf8(branches.stdout).unwrap();
    Ok(branches)
}

fn add_protected_branches(branches: &str) {
    Command::new("git")
            .arg("config")
            .arg("--add")
            .arg("x-protected-branch.merge-branch")
            .arg(branches)
            .output()
            .expect("failed to execute process");
}
