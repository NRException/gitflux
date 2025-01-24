use semver::Version;

struct VersionSchema {
    major: bool,
    minor: bool,
    patch: bool,
}

impl From<String> for VersionSchema {
    fn from(s: String) -> VersionSchema {
        let _s_upper = s.to_uppercase();
        let mut _r = VersionSchema{major: false, minor: false, patch: false};

        if _s_upper == "MAJOR" {_r.major = false};
        if _s_upper == "MINOR" {_r.minor = false};
        if _s_upper == "PATCH" {_r.patch = false};

        _r
    }
}

impl From<VersionSchema> for String {
    fn from(vs:VersionSchema) -> String {
        let mut _r = String::default();

        if vs.major {_r = String::from("MAJOR")};
        if vs.minor {_r = String::from("MINOR")};
        if vs.patch {_r = String::from("PATCH")};

        _r
    }
}
