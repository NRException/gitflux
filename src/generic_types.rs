use semver::Version;

struct VersionTagSchema {
    major: bool,
    minor: bool,
    patch: bool,
}

impl From<String> for VersionTagSchema {
    fn from(s: String) -> VersionTagSchema {
        let _s_upper = s.to_uppercase();
        let mut _r = VersionTagSchema{major: false, minor: false, patch: false};

        if _s_upper == "MAJOR" {_r.major = true};
        if _s_upper == "MINOR" {_r.minor = true};
        if _s_upper == "PATCH" {_r.patch = true};

        _r
    }
}

impl From<VersionTagSchema> for String {
    fn from(vs:VersionTagSchema) -> String {
        let mut _r = String::default();

        if vs.major {_r = String::from("MAJOR")};
        if vs.minor {_r = String::from("MINOR")};
        if vs.patch {_r = String::from("PATCH")};

        _r
    }
}
