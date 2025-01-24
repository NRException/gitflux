use std::str;
use git2::Repository;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::info;

include!("generic_types.rs");
include!("tag_manager.rs");

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


fn main() {
    let _args = GlobalOptions::parse();
    let _discovery_directories = [String::from(".")]; 
    env_logger::Builder::new().filter_level(_args.verbose.log_level_filter()).init();

    match &_args.command_list {
        Some(Commands::Tag { tag_schema }) => {
            let _ts: VersionSchema = String::into(tag_schema.to_owned());
            let _rep = match Repository::open(&_args.repo_path) {
                Ok(r) => {
                    let mut _cache = GitTagManager::new(r);
                    _cache.bump_latest_tag(_ts, Some(1)).unwrap();
                },
                Err(_e) => panic!("could not discover repo at path ."),
            };
        },
        None => {},
    }

}
