struct GitTagManager {
    tag_objects:    Vec<String>,
    root_tag:       Version,
    latest_tag:     Version,
    associated_repo: Repository
}

fn raw_tag_to_version(s: &String) -> Version {
    let _string_split: Vec<&str> = s.split("/").collect(); 
    return Version::parse(&_string_split[2]).unwrap();
}

#[allow(dead_code)]
impl GitTagManager {
    fn new(associated_repo: Repository) -> GitTagManager {
        let mut _r = GitTagManager {
            tag_objects: Vec::new(), 
            associated_repo, 
            root_tag: Version::parse("0.0.0").unwrap(),
            latest_tag: Version::parse("0.0.0").unwrap(),
        };
        _r.refresh_cache();
        _r
    }


    fn set_latest_tag(&mut self) -> () {
        info!("getting latest tag...");

        let mut _tags = self.tag_objects.clone();
        _tags.reverse();
        let _latest_tag = _tags[0].clone();
        info!("latest tag is: {}", _latest_tag); 

        // TODO - Add sense checking to parse only valid semver tags.

        self.latest_tag = raw_tag_to_version(&_latest_tag);
    }

    fn set_root_tag(&mut self) -> () {
        info!("validating root tag exists...");

        let mut _match_term = String::from("refs/tags/0.0.1");

        if self.tag_objects.contains(&_match_term) {
            info!("found root tag: {}", &_match_term);
            self.root_tag = raw_tag_to_version(&_match_term);
        } else {panic!("could not find / resolve target tag...")}
    }
    
    pub fn refresh_cache(&mut self) -> () {
        info!("refreshing git tag cache...");
        let mut _r = Vec::new();

        let _b = &self.associated_repo.tag_foreach(|_o, n| -> bool {
            let _tag_name = match String::from_utf8(n.to_vec()) {
                Ok(_t) => {_t},
                Err(_e) => panic!("could not convert tag name to string."),
            };
            _r.push(_tag_name.clone());
            true
        });

        self.tag_objects = _r;
        self.set_root_tag();
        self.set_latest_tag();
    }
}
