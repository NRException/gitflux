use std::str;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::info;

include!("types.rs");
include!("repo_actions.rs");

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
           info!("bumping version {}", (String::from(tag_schema)));

           let _rep = match Repository::open(&_args.repo_path) {
                Ok(r) => {
                    info!("opened repo successfully at {}", r.path().to_string_lossy());
                    info!("head: {}", r.head().unwrap().name().unwrap().to_string()); 

                    let mut _matched_tag_name: Option<String> = Default::default();

                    let _b = r.tag_foreach( |_o, n| -> bool {
                        let _tag_name = String::from_utf8(n.to_vec()).unwrap();

                        if _tag_name.contains("refs/tags/0.0.1") {
                            _matched_tag_name = Some(_tag_name);
                            false
                        } else {_matched_tag_name = None; true}
                    });
                    
                    let _b = match _matched_tag_name {
                        Some(_b) => {info!("found root tag: {}", _b)}
                        None => {panic!("could not find root tag");}
                    };

                },
                Err(_e) => info!("could not discover repo at path {}", &_args.repo_path),
           };

        }

        None => {}
    }

}
