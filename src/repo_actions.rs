use git2::Repository;

fn bump_tag(tag_schema: VersionSchema, path: String) -> () {
   info!("bumping version {}", (String::from(tag_schema)));

   let _rep = match Repository::open(&path) {
        Ok(r) => {
            info!("opened repo successfully at {}", r.path().to_string_lossy());
            info!("head: {}", &r.head().unwrap().name().unwrap().to_string()); 

            let _cache = GitTagObjectCache::new(r);
        },
        Err(_e) => info!("could not discover repo at path {}", path),
   };
}
