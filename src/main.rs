use std::str;
use std::process::exit;
use git2::Repository;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::{info, warn, error};

include!("generic_types.rs");
include!("tag_manager.rs");

#[derive(Parser, Debug)]
#[command(version, about="gitflux - simple semver tag and commit management", long_about = None, arg_required_else_help=true)]
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
    /// Provides semver tag management
    Bump {
        /// Overrides version to increment git tag by. Valid values are MAJOR, MINOR, PATCH.
        #[clap(short, long , default_value_t=String::from("PATCH"))]
        tag_schema: String,

        /// Creates a default semver tag with version 0.0.1, sequential bumps can be made off of this tag. NOTE: tag will point to current HEAD
        #[clap(short, long , default_value_t=false)]
        init: bool,
    },
}


fn main() {
    let _args = GlobalOptions::parse();
    let _discovery_directories = [String::from(".")]; 
    env_logger::Builder::new().filter_level(_args.verbose.log_level_filter()).init();

    match &_args.command_list {
        Some(Commands::Bump { tag_schema, init }) => {
            let _ts: VersionTagSchema = String::into(tag_schema.to_owned());
            
            // TODO - Handle errors passed up correctly!
            let _rep = match Repository::open(&_args.repo_path) {
                Ok(r) => {

                    // Init new tag manager in current repo
                    let mut tm = match GitTagManager::new(r) {
                        Ok(r) => r, 
                        Err(e) => {error!("{}", e.to_string()); exit(1)}
                    };

                    if *init {
                        info!("--init specified, creating tag with ver 0.0.1");
                        
                        match tm.create_version_tag(Version::new(0,0,1)) {
                            Ok(r) => r, 
                            Err(e) => {error!("{}", e.to_string()); exit(1)}
                        };
                    } else {
                        match tm.bump_latest_tag(_ts, Some(1)){
                            Ok(r) => r, 
                            Err(e) => {error!("{}", e.to_string()); exit(1)}
                        };
                    }

                },
                Err(_e) => panic!("could not discover repo at path {}", &_args.repo_path),
            };
        },
        None => {},
    }

}
