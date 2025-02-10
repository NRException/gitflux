use std::fmt;

const COMMIT_TEMPLATE: &'static str =
    "<req_type>(<opt_scope>)<opt_tagged>: <opt_body>\n <opt_footers>";

//docs / spec here:
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

#[derive(Debug, Clone)]
struct CommitTypeFromParseError;
#[derive(Debug, Clone)]
struct CommitTypeToParseError;

impl fmt::Display for CommitTypeFromParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to successfully parse string to CommitType")
    }
}

impl fmt::Display for CommitTypeToParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to successfully parse CommitType to string")
    }
}

// implementation isn't too flexible, but we don't need an outside package to implement to/as_str
// as these types probably aren't changing any time soon
impl CommitType {
    fn as_str(&self) -> Result<&str, CommitTypeToParseError> {
        match &self {
            CommitType::FIX => Ok("fix"),
            CommitType::FEAT => Ok("feat"),
            CommitType::BUILD => Ok("build"),
            CommitType::CHORE => Ok("chore"),
            CommitType::CI => Ok("ci"),
            CommitType::DOCS => Ok("docs"),
            CommitType::STYLE => Ok("style"),
            CommitType::REFACTOR => Ok("refactor"),
            CommitType::PERF => Ok("perf"),
            CommitType::TEST => Ok("test"),
        }
    }

    fn from_str(s: &str) -> Result<Self, CommitTypeFromParseError> {
        match s.to_ascii_lowercase().as_str() {
            "fix" => Ok(CommitType::FIX),
            "feat" => Ok(CommitType::FEAT),
            "build" => Ok(CommitType::BUILD),
            "chore" => Ok(CommitType::CHORE),
            "ci" => Ok(CommitType::CI),
            "docs" => Ok(CommitType::DOCS),
            "style" => Ok(CommitType::STYLE),
            "refactor" => Ok(CommitType::REFACTOR),
            "perf" => Ok(CommitType::PERF),
            "test" => Ok(CommitType::TEST),
            _ => Err(CommitTypeFromParseError),
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
        // TODO: forward error
        let mut r = COMMIT_TEMPLATE.replace("<req_type>", commit_type.as_str().unwrap());
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
