struct GitCommitManager {
    associated_repo:    Repository
}

impl GitCommitManager {
    fn new(associated_repo: Repository) -> Result<GitCommitManager, git2::Error> {
        let _r = GitCommitManager {
            associated_repo, 
        };

        Ok(_r)
    }

    pub fn format_commit(message: String) -> Result<String, git2::Error> {

    }
}


