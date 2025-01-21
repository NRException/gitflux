use git2::Repository;

include!("types.rs");

fn bump_tag(tag_schema: VersionSchema, path: String) -> () {
   info!("bumping version {}", (String::from(tag_schema)));

   let _rep = match Repository::open(path) {
        Ok(r) => {
            info!("opened repo successfully at {}", r.path().to_string_lossy());
            info!("head: {}", r.head().unwrap().name().unwrap().to_string()); 

            let mut _matched_tag_name: Option<String> = Default::default();
            let _b = r.tag_foreach( |_o, n| -> bool {
                let _tag_name = match String::from_utf8(n.to_vec()) {
                    Ok(_t) => _t,
                    Err(_e) => Err(_e),
                };

                if _tag_name.contains("refs/tags/0.0.1") {
                    _matched_tag_name = Some(_tag_name);
                    false
                } else {_matched_tag_name = None; true}
            });
            
            let _b = match _matched_tag_name {
                Some(_b) => {info!("found root tag: {}", _b)}
                None => {panic!("could not find root tag");}
            };



        },
        Err(_e) => info!("could not discover repo at path {}", path),
   };
}
