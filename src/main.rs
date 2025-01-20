use git2::Repository;
use std::path::Path;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::info;

include!("types.rs");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GlobalOptions {
   
    /// Provides verbose output.
    #[command(flatten)]
    verbose: Verbosity,

    #[clap(subcommand)]
    command_list: Option<Commands>,

    /// Overrides the target path of the repo, can be relative or absolute.
    #[clap(short, long , default_value_t=String::from("."))]
    repo_path: String,
}


#[derive(Subcommand, Debug)]
enum Commands {
    /// Bumps tag according to configuration. Overrides available in argument parameters.
    Tag {
        /// Overrides version to increment git tag by. Valid values are MAJOR, MINOR, PATCH.
        #[clap(short, long , default_value_t=String::from("PATCH"))]
        tag_schema: String,

    },
}

fn validate_repo(path: String) -> Repository {
        let _relpath = Path::new(&path);
        Repository::discover(_relpath).unwrap()
}

fn validate_root_tag(repo: &Repository) -> bool {
    let _refmatch = repo.find_reference("");

    for (_i, item) in _refmatch.iter().enumerate() {
        println!("enumerating reference {}", item.name().unwrap().to_string());

        if item.is_tag() {
            println!("tag found: {}", item.name().unwrap().to_string());
        }
    }

    return false;
}

fn main() {
    let _args = GlobalOptions::parse();
    let _discovery_directories = [String::from(".")];
    
    env_logger::Builder::new().filter_level(_args.verbose.log_level_filter()).init();

    match &_args.command_list {
        Some(Commands::Tag { tag_schema }) => {
           info!("bumping version {}", (String::from(tag_schema)));
           let _rep = validate_repo(_args.repo_path);
           let _res = validate_root_tag(&_rep);


        }

        None => {}
    }

}
