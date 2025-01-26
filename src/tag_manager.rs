use git2::{ErrorClass, ErrorCode};

struct GitTagManager {
    tag_objects:        Vec<String>,
    associated_repo:    Repository
}


fn raw_tag_to_version(s: &String) -> Result<Version, semver::Error> {
    let _string_split: Vec<&str> = s.split("/").collect(); 
    Ok(Version::parse(&_string_split[2])?)
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
        info!("tag created: {} ref: {}", v, t); 
        Ok(())
    }

    fn refresh_tags(&mut self) -> () {
        info!("refreshing git tag cache...");
        let mut _r = Vec::new();

        let _b = &self.associated_repo.tag_foreach(|_o, n| {
            let _tag_name = match String::from_utf8(n.to_vec()) {
                Ok(t) => _r.push(t.clone()),
                Err(e) => warn!("could not convert tag to string vector dropping tag with error: {}", e.to_string()),
            };

            true // callback expects true if we want to stop the tag_foreach iter loop
        });

        self.tag_objects = _r;
    }

    pub fn get_latest_tag(&mut self) -> Result<Version, git2::Error> {
        info!("getting latest tag...");
        self.refresh_tags();

        let mut _tags = self.tag_objects.clone();
        
        if _tags.len() > 0 {
            _tags.reverse(); //sort but backwards. (desc)

            for t in _tags.iter() {
                match raw_tag_to_version(t) {
                    Ok(r) => return Ok(r),
                    Err(_) => {
                        warn!("tag '{}' is not semver compatible. moving to next tag...", t);
                        continue; // Continue loop until we find a good semver tag.
                    }
                };
            };

            // If we exit the loop without returning a good tag, its a bad day...
            Err(git2::Error::new(
                ErrorCode::NotFound, 
                ErrorClass::Tag, 
                "matching semver tag not found. create one with 'bump --init'"
            ))

        } else {
            Err(git2::Error::new(
                ErrorCode::NotFound, 
                ErrorClass::Tag, 
                "no tags found in repo. create one with 'bump --init'"
            ))
        }

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
}

// TODO - Write proper unit tests...
#[cfg(test)]
mod tests{
}
