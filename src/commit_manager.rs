const COMMIT_TEMPLATE: &'static str =
    "<req_type>(<opt_scope>)<opt_tagged>: <opt_body>\n <opt_footers>";

// docs / spec here:
//https://www.conventionalcommits.org/en/v1.0.0/#summary
enum CommitType {
    FIX,
    FEAT,
    BUILD,
    CHORE,
    CI,
    DOCS,
    STYLE,
    REFACTOR,
    PERF,
    TEST,
}

// implementation isn't too flexible, but we don't need an outside package to implement to/as_str
// as these types probably aren't changing any time soon
impl CommitType {
    fn as_str(&self) -> &'static str {
        match &self {
            CommitType::FIX => "fix",
            CommitType::FEAT => "feat",
            CommitType::BUILD => "build",
            CommitType::CHORE => "chore",
            CommitType::CI => "ci",
            CommitType::DOCS => "docs",
            CommitType::STYLE => "style",
            CommitType::REFACTOR => "refactor",
            CommitType::PERF => "perf",
            CommitType::TEST => "test",
        }
    }
}

struct GitCommitManager {
    associated_repo: Repository,
}

impl GitCommitManager {
    fn new(associated_repo: Repository) -> Result<GitCommitManager, git2::Error> {
        let r = GitCommitManager { associated_repo };

        Ok(r)
    }

    pub fn format_commit(
        &self,
        commit_type: CommitType,
        commit_scope: Option<String>,
        is_flagged: bool,
        commit_body: Option<String>,
        commit_footers: Option<Vec<String>>,
    ) -> String {
        let mut r = COMMIT_TEMPLATE.replace("<req_type>", commit_type.as_str());
        r = r.replace(
            "<opt_scope>",
            match &commit_scope {
                Some(i) => i.as_str(),
                None => "",
            },
        );
        r = r.replace("<opt_flagged>", if is_flagged { "!" } else { "" });
        r = r.replace(
            "<opt_body>",
            match &commit_body {
                Some(i) => i.as_str(),
                None => "",
            },
        );
        let footers = match &commit_footers {
            Some(i) => i.iter().map(|o| format!("{}\n", o)).collect::<String>(),
            None => "".to_owned(),
        };
        r = r.replace("<opt_footers>", &footers);
        r
    }
}

// TODO - Write proper unit tests...
#[cfg(test)]
mod tests_commitmanager {}
