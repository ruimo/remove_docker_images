use std::hash::{Hash, Hasher};

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use super::version;

pub struct ImageEntry {
    pub id: String,
    pub ver: version::Version,
}

impl PartialEq for ImageEntry {
    fn eq(&self, other: &Self) -> bool {
        self.ver == other.ver
    }
}

impl Eq for ImageEntry {}

impl Hash for ImageEntry {
    fn hash<H:Hasher>(&self, state: &mut H) {
        self.ver.hash(state);
    }
}

impl fmt::Debug for ImageEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImageEntry [id: {}, ver: {}]", self.id, self.ver)
    }
}

pub struct Images {
    // key: repository
    pub entries: HashMap<String, HashSet<ImageEntry>>
}

impl Images {
    pub fn delete<F>(&self, canonical_keep_count: usize, snapshot_keep_count: usize, mut del: F) -> ()
      where F : FnMut(&str, &version::Version) -> () // repo, version
    {
        for (repo, entry) in &self.entries {
            let mut sum_canonical: HashMap<&Option<String>, Vec<&version::Version>> = HashMap::new();
            let mut sum_snapshot: HashMap<&Option<String>, Vec<&version::Version>> = HashMap::new();

            for e in entry {
                let sum = if e.ver.is_snapshot { &mut sum_snapshot } else { &mut sum_canonical };
                let keep_count = if e.ver.is_snapshot { snapshot_keep_count } else {canonical_keep_count };
                let tbl = sum.entry(&e.ver.branch).or_insert_with(|| Vec::new());
                match tbl.binary_search(&&e.ver) {
                    Ok(_idx) => {
                    },
                    Err(idx) => {
                        tbl.insert(idx, &e.ver);
                    }
                }

                if keep_count < tbl.len() {
                    let v = tbl.remove(0);
                    del(repo, v);
                }
            }
        }
    }
}

#[test]
fn delete_test() {
    let parser = version::parser();
    let mut map: HashMap<String, HashSet<image::ImageEntry>> = HashMap::new();
    let mut entries0 = HashSet::new();
    entries0.insert(ImageEntry { id: "id00".to_string(), ver: parser.parse("1.0").unwrap() });
    entries0.insert(ImageEntry { id: "id01".to_string(), ver: parser.parse("1.1").unwrap() });
    entries0.insert(ImageEntry { id: "id02".to_string(), ver: parser.parse("1.10").unwrap() });
    entries0.insert(ImageEntry { id: "id03".to_string(), ver: parser.parse("1.2").unwrap() });

    entries0.insert(ImageEntry { id: "id04".to_string(), ver: parser.parse("1.2-SNAPSHOT").unwrap() });
    entries0.insert(ImageEntry { id: "id05".to_string(), ver: parser.parse("1.1-SNAPSHOT").unwrap() });

    entries0.insert(ImageEntry { id: "id06".to_string(), ver: parser.parse("1.2.0-BR123").unwrap() });
    entries0.insert(ImageEntry { id: "id07".to_string(), ver: parser.parse("1.2.1-BR123").unwrap() });
    entries0.insert(ImageEntry { id: "id08".to_string(), ver: parser.parse("1.2.10-BR123").unwrap() });
    entries0.insert(ImageEntry { id: "id09".to_string(), ver: parser.parse("1.2.2-BR123").unwrap() });

    entries0.insert(ImageEntry { id: "id10".to_string(), ver: parser.parse("1.2.2-BR123-SNAPSHOT").unwrap() });
    entries0.insert(ImageEntry { id: "id11".to_string(), ver: parser.parse("1.2.1-BR123-SNAPSHOT").unwrap() });

    map.insert("repo0".to_string(), entries0);

    let mut entries1 = HashSet::new();
    entries1.insert(ImageEntry { id: "id12".to_string(), ver: parser.parse("2.0").unwrap() });
    entries1.insert(ImageEntry { id: "id13".to_string(), ver: parser.parse("2.1").unwrap() });
    entries1.insert(ImageEntry { id: "id14".to_string(), ver: parser.parse("2.10").unwrap() });
    entries1.insert(ImageEntry { id: "id15".to_string(), ver: parser.parse("2.2").unwrap() });

    map.insert("repo1".to_string(), entries1);

    let images = Images { entries: map };

    let mut deleted = HashSet::new();
    images.delete(3, 1, |repo, ver| {
        deleted.insert(format!("{}:{}", repo, ver.to_string()));
    });

    assert_eq!(deleted.len(), 5);
    assert_eq!(deleted.contains("repo0:1.0"), true);
    assert_eq!(deleted.contains("repo0:1.1-SNAPSHOT"), true);
    assert_eq!(deleted.contains("repo0:1.2.0-BR123"), true);
    assert_eq!(deleted.contains("repo0:1.2.1-BR123-SNAPSHOT"), true);
    assert_eq!(deleted.contains("repo1:2.0"), true);
}
