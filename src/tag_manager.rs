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

    pub fn bump_latest_tag(&mut self, v: VersionSchema, i: Option<u64>) -> Result<(), git2::Error> {
        info!("bumping latest tag...");
        match i {
            Some (inc) => {
                info!("incremental specified, bumping by {:?}", inc);
                let mut _incremental_tag = self.latest_tag.to_owned();
                if v.major {_incremental_tag.major += inc}
                if v.minor {_incremental_tag.minor += inc}
                if v.patch {_incremental_tag.patch += inc}

                // Get latest commit and assign tag
                let obj = self.associated_repo.revparse_single("HEAD")?;
                self.associated_repo.tag_lightweight(_incremental_tag.to_string().as_str(), &obj, false)?;
                
                drop(obj); //I'm not so sure about this scoob! 󰩄, done to drop scope of obj so
                //ownership can be returned to call:
                self.refresh_cache();

                Ok(())
            }
            None => {
                info!("no incremental tag version specified, bumping by 1 according to VersionSchema");
                let mut _incremental_tag = self.latest_tag.to_owned();
                if v.major {_incremental_tag.major += 1}
                if v.minor {_incremental_tag.minor += 1}
                if v.patch {_incremental_tag.patch += 1}

                let obj = self.associated_repo.revparse_single("HEAD")?;
                self.associated_repo.tag_lightweight(_incremental_tag.to_string().as_str(), &obj, false)?;

                drop(obj); 
                self.refresh_cache();

                Ok(())
            }
        }

    }

    fn init_latest_tag(&mut self) -> () {
        info!("getting latest tag...");

        let mut _tags = self.tag_objects.clone();
        _tags.reverse();
        let _latest_tag = _tags[0].clone();
        info!("latest tag is: {}", _latest_tag); 

        // TODO - Add sense checking to parse only valid semver tags.

        self.latest_tag = raw_tag_to_version(&_latest_tag);
    }

    fn init_root_tag(&mut self) -> () {
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
        self.init_root_tag();
        self.init_latest_tag();
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_git_tag_manager_init() {
        let _rep = match Repository::open(".") {
            Ok(r) => {
                let _cache = GitTagManager::new(r);
            },
            Err(_e) => info!("could not discover repo at path ."),
        };
    }

    #[test]
    fn test_git_tag_manager_bump() -> Result<(), git2::Error>{
        let _rep = match Repository::open(".") {
            Ok(r) => {
                // This should call all private functions so is a good test for all.
                let mut _cache = GitTagManager::new(r);

                _cache.bump_latest_tag(VersionSchema{major: false, minor: false, patch:true}, Some(1))?;
                _cache.bump_latest_tag(VersionSchema{major: false, minor: true, patch:false}, Some(1))?;
                _cache.bump_latest_tag(VersionSchema{major: true, minor: false, patch:false}, Some(1))?;
            },
            Err(_e) => panic!("could not discover repo at path ."),
        };

        Ok(())
    }


}
