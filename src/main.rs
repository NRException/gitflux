use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use git2::Repository;
use log::{error, info, warn};
use std::process::exit;
use std::str;

include!("generic_types.rs");
include!("tag_manager.rs");
include!("commit_manager.rs");

#[derive(Parser, Debug)]
#[command(version, about="gitflux - simple semver tag and commit management", long_about = None, arg_required_else_help=true)]
struct GlobalOptions {
    /// Provides verbose output.
    #[command(flatten)]
    verbose: Verbosity,

    #[clap(subcommand)]
    command_list: Option<Arguments>,

    /// Overrides the target path of the repo, can be relative or absolute.
    #[clap(short, long , default_value_t=String::from("."))]
    repo_path: String,
}

#[derive(Subcommand, Debug)]
enum Arguments {
    /// Provides semver tag management
    Bump {
        /// Overrides version to increment git tag by. Valid values are MAJOR, MINOR, PATCH.
        #[clap(short, long , default_value_t=String::from("PATCH"))]
        tag_schema: String,

        /// Creates a default semver tag with version 0.0.1, sequential bumps can be made off of this tag. NOTE: tag will point to current HEAD
        #[clap(short, long, default_value_t = false)]
        init: bool,
    },

    /// Provides conventional commit management
    Commit {
        /// Message to format into conventional commit
        #[clap(short, long)]
        message: String,

        /// Defines the type of commit.
        #[clap(short, long, value_enum)]
        commit_type: CommitType,

        /// Defines the scope of the commit. (can be a file name or generic scope of change)
        #[clap(short, long)]
        commit_scope: String,

        /// Defines whether or not the scope is flagged, this will add a "!" to the commit message
        /// if this is true, intended to draw more attention to a potentially breaking change.
        #[clap(short, long)]
        commit_flagged: bool,

        /// If specified, this will print the commit message rather than adding the current staged
        /// changes to a new commit.
        #[clap(short, long, default_value_t = false)]
        print_only: bool,
    },
}

fn main() {
    let _args = GlobalOptions::parse();
    let _discovery_directories = [String::from(".")];
    env_logger::Builder::new()
        .filter_level(_args.verbose.log_level_filter())
        .init();

    match &_args.command_list {
        Some(Arguments::Bump { tag_schema, init }) => {
            let _ts: VersionTagSchema = String::into(tag_schema.to_owned());

            let _rep = match Repository::open(&_args.repo_path) {
                Ok(r) => {
                    // Init new tag manager in current repo
                    let mut tm = match GitTagManager::new(r) {
                        Ok(r) => r,
                        Err(e) => {
                            error!("{}", e.to_string());
                            exit(1)
                        }
                    };

                    if *init {
                        info!("--init specified, creating tag with ver 0.0.1.");

                        match tm.create_version_tag(Version::new(0, 0, 1)) {
                            Ok(r) => r,
                            Err(e) => {
                                error!("{}", e.to_string());
                                exit(1)
                            }
                        };
                    } else {
                        match tm.bump_latest_tag(_ts, Some(1)) {
                            Ok(r) => r,
                            Err(e) => {
                                error!("{}", e.to_string());
                                exit(1)
                            }
                        };
                    }
                }
                Err(_e) => panic!("could not discover repo at path {}", &_args.repo_path),
            };
        }

        Some(Arguments::Commit {
            message,
            commit_type,
            commit_scope,
            commit_flagged,
            print_only,
        }) => {
            let _rep = match Repository::open(&_args.repo_path) {
                Ok(r) => {
                    // Init new commit manager in current repo
                    let mut cm = match GitCommitManager::new(r) {
                        Ok(r) => r,
                        Err(e) => {
                            error!("{}", e.to_string());
                            exit(1)
                        }
                    };

                    // IDEA - If certain type of flag exists, add short name of changed files to
                    // commit scope automagically?
                    if *print_only {
                        // TODO - Add arg handling for vec<String>
                        info!("--print-only specified, not commiting.");
                        writeln!(cm.format_commit(
                            commit_type,
                            commit_scope,
                            commit_flagged,
                            message,
                            commit_footers
                        ))
                    } else {
                    }
                }
                Err(_e) => panic!("could not discover repo at path {}", &_args.repo_path),
            };
        }
        None => {}
    }
}
