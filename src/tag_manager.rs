use git2::{ErrorClass, ErrorCode};

struct GitTagManager {
    tag_objects:        Vec<String>,
    associated_repo:    Repository
}

fn raw_tag_to_version(s: &String) -> Version {
    let _string_split: Vec<&str> = s.split("/").collect(); 
    return Version::parse(&_string_split[2]).unwrap();
}

impl GitTagManager {
    fn new(associated_repo: Repository) -> Result<GitTagManager, git2::Error> {
        let mut _r = GitTagManager {
            tag_objects: Vec::new(), 
            associated_repo, 
        };

        _r.refresh_tags();
        Ok(_r)
    }

    fn create_version_tag(&mut self, v: Version) -> Result<(), git2::Error> {
        let obj = self.associated_repo.revparse_single("HEAD")?;
        let t = self.associated_repo.tag_lightweight(v.to_string().as_str(), &obj, false)?;
        info!("tag created {} : ref {}", v, t); 
        Ok(())
    }


    pub fn bump_latest_tag(&mut self, v: VersionSchema, i: Option<u64>) -> Result<(), git2::Error> {
        info!("bumping latest tag...");
        match i {
            Some (inc) => {
                info!("incremental specified, bumping by {:?}", inc);
                let mut _incremental_tag = self.get_latest_tag()?;
                info!("latest tag: {}", _incremental_tag);
                if v.major {_incremental_tag.major += inc}
                if v.minor {_incremental_tag.minor += inc}
                if v.patch {_incremental_tag.patch += inc}

                // Get latest commit and assign tag
                self.create_version_tag(_incremental_tag)?;

                self.refresh_tags();
                
                Ok(())
            }
            None => {
                info!("no incremental tag version specified, bumping by 1 according to VersionSchema");
                let mut _incremental_tag = self.get_latest_tag()?;
                info!("latest tag: {}", _incremental_tag);
                if v.major {_incremental_tag.major += 1}
                if v.minor {_incremental_tag.minor += 1}
                if v.patch {_incremental_tag.patch += 1}

                self.create_version_tag(_incremental_tag)?;

                self.refresh_tags();

                Ok(())
            }
        }

    }

    pub fn get_latest_tag(&mut self) -> Result<Version, git2::Error> {
        self.refresh_tags();

        let mut _tags = self.tag_objects.clone();
        
        // TODO - Add sense checking to parse only valid semver tags.
        if _tags.len() > 0 {
            _tags.reverse();
            let _latest_tag = _tags[0].clone();
            Ok(raw_tag_to_version(&_latest_tag))
        } else {
            Err(git2::Error::new(ErrorCode::NotFound, ErrorClass::Tag, String::from("matching semver tag not found. create one with 'bump --init'")))
        }

    }

    fn refresh_tags(&mut self) -> () {
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
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_git_tag_manager_init() -> Result<(), git2::Error>{
        let r = Repository::open(".")?;
        let _cache = GitTagManager::new(r)?;
        Ok(())
    }

    #[test]
    fn test_git_tag_manager_bump() -> Result<(), git2::Error>{
        let r = Repository::open(".")?;
        let mut _cache = GitTagManager::new(r)?;

        _cache.bump_latest_tag(VersionSchema{major: false, minor: false, patch:true}, Some(1))?;
        _cache.bump_latest_tag(VersionSchema{major: false, minor: true, patch:false}, Some(1))?;
        _cache.bump_latest_tag(VersionSchema{major: true, minor: false, patch:false}, Some(1))?;

        Ok(())
    }


}
