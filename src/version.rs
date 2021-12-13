extern crate regex;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt;

pub struct Version {
    pub major: i32,
    pub minor: i32,  // If minor version does not exist, minor = -1
    pub patch: i32,  // If patch version does not exist, patch = -1
    pub is_snapshot: bool,
    pub branch: Option<String>,
    pub raw: String,
}

pub struct VersionParser {
    re: regex::Regex
}

pub fn parser() -> VersionParser {
    VersionParser {
        re: regex::Regex::new(r"^[vV]?(\d{1,5})(\.\d{1,5})?(\.\d{1,5})?(-.*)?$").unwrap()
    }
}

impl VersionParser {
    pub fn parse(&self, s: &str) -> Option<Version> {
        self.re.captures(s).map(|caps| {
            let minor = match caps.get(2) {
                Some(m) => m.as_str()[1..].parse().unwrap(),
                None => -1
            };

            let patch = match caps.get(3) {
                Some(m) => m.as_str()[1..].parse().unwrap(),
                None => -1
            };

            let (branch, is_snapshot) = match caps.get(4) {
                Some(m) => {
                    let v: Vec<&str> = m.as_str().split("-SNAPSHOT").collect();
                    let br = v[0].to_string();
                    (if br.len() > 1 {Some(br[1..].to_string())} else {None}, v.len() == 2)
                },
                None => (None, false)
            };


            Version {
                major: caps.get(1).unwrap().as_str().parse().unwrap(),
                minor: minor,
                patch: patch,
                is_snapshot: is_snapshot,
                branch: branch,
                raw: s.to_string(),
            }
        })
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major &&
            self.minor == other.minor &&
            self.patch == other.patch &&
            self.is_snapshot == other.is_snapshot &&
            self.branch == other.branch
    }
}

impl Eq for Version {}

impl Hash for Version {
    fn hash<H:Hasher>(&self, state: &mut H) {
        self.major.hash(state);
        self.minor.hash(state);
        self.patch.hash(state);
        self.is_snapshot.hash(state);
        self.branch.hash(state);
    }
}

impl fmt::Display for Version {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{}", self.raw)
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Version [major: {}, minor: {}, patch: {}, branch: {:?}, is_snapshot: {}]",
               self.major, self.minor, self.patch, self.branch, self.is_snapshot)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let is = self.is_snapshot.cmp(&other.is_snapshot);
        if is != Ordering:: Equal {
            return is;
        }

        let br = self.branch.cmp(&other.branch);
        if br != Ordering::Equal {
            return br;
        }

        let ma = self.major.cmp(&other.major);
        if ma != Ordering::Equal {
            return ma;
        }

        let mi = self.minor.cmp(&other.minor);
        if mi != Ordering::Equal {
            return mi;
        }

        self.patch.cmp(&other.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_major_minor_patch() {
    let v = parser().parse("1.2.3").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
    assert_eq!(v.is_snapshot, false);
    assert_eq!(v.branch, None);
}

#[test]
fn test_major_minor_patch_snapshot() {
    let v = parser().parse("1.2.3-SNAPSHOT").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
    assert_eq!(v.is_snapshot, true);
    assert_eq!(v.branch, None);
}

#[test]
fn test_major_minor() {
    let v = parser().parse("1.2").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, -1);
    assert_eq!(v.is_snapshot, false);
    assert_eq!(v.branch, None);
}

#[test]
fn test_major() {
    let v = parser().parse("1").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, -1);
    assert_eq!(v.patch, -1);
    assert_eq!(v.is_snapshot, false);
    assert_eq!(v.branch, None);
}

#[test]
fn test_branch() {
    let v = parser().parse("1.2-BR102").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, -1);
    assert_eq!(v.is_snapshot, false);
    assert_eq!(v.branch, Some("BR102".to_string()));
}

#[test]
fn test_branch_snapshot() {
    let v = parser().parse("1.2-BR102-SNAPSHOT").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, -1);
    assert_eq!(v.is_snapshot, true);
    assert_eq!(v.branch, Some("BR102".to_string()));
}

#[test]
fn latest() {
    assert_eq!(parser().parse("latest").is_none(), true);
}

#[test]
fn v() {
    let v = parser().parse("v0.31.2").unwrap();
    assert_eq!(v.major, 0);
    assert_eq!(v.minor, 31);
    assert_eq!(v.patch, 2);
    assert_eq!(v.is_snapshot, false);
    assert_eq!(v.branch, None);
    assert_eq!(format!("{}", v), "v0.31.2");
}

#[test]
fn sdk8() {
    let v = parser().parse("8-sdk").unwrap();
    assert_eq!(v.major, 8);
    assert_eq!(v.minor, -1);
    assert_eq!(v.patch, -1);
    assert_eq!(v.is_snapshot, false);
    assert_eq!(v.branch, Some("sdk".to_string()));
}


#[test]
fn major_minor_patch_cmp() {
    let parser = parser();
    assert_eq!(parser.parse("1.2.3").unwrap().cmp(&parser.parse("1.2.3").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2.2").unwrap().cmp(&parser.parse("1.2.3").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("1.2.3").unwrap().cmp(&parser.parse("1.2.2").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2.10").unwrap().cmp(&parser.parse("1.2.3").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("1.2").unwrap().cmp(&parser.parse("1.2").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2").unwrap().cmp(&parser.parse("1.1").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2").unwrap().cmp(&parser.parse("1.3").unwrap()), Ordering::Less);

    assert_eq!(parser.parse("1").unwrap().cmp(&parser.parse("1").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1").unwrap().cmp(&parser.parse("2").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("2").unwrap().cmp(&parser.parse("1").unwrap()), Ordering::Greater);
}

#[test]
fn major_minor_patch_cmp_snapshot() {
    let parser = parser();
    assert_eq!(parser.parse("1.2.3-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.3-SNAPSHOT").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2.2-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.3-SNAPSHOT").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("1.2.3-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.2-SNAPSHOT").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2.10-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.2-SNAPSHOT").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("1.2-SNAPSHOT").unwrap().cmp(&parser.parse("1.2-SNAPSHOT").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2-SNAPSHOT").unwrap().cmp(&parser.parse("1.1-SNAPSHOT").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2-SNAPSHOT").unwrap().cmp(&parser.parse("1.3-SNAPSHOT").unwrap()), Ordering::Less);

    assert_eq!(parser.parse("1-SNAPSHOT").unwrap().cmp(&parser.parse("1-SNAPSHOT").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1-SNAPSHOT").unwrap().cmp(&parser.parse("2-SNAPSHOT").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("2-SNAPSHOT").unwrap().cmp(&parser.parse("1-SNAPSHOT").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("2").unwrap().cmp(&parser.parse("2-SNAPSHOT").unwrap()) != Ordering::Equal, true);
}

#[test]
fn branch() {
    let parser = parser();
    assert_eq!(parser.parse("1.2.3-BR123").unwrap().cmp(&parser.parse("1.2.3-BR123").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2.2-BR123").unwrap().cmp(&parser.parse("1.2.3-BR123").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("1.2.3-BR123").unwrap().cmp(&parser.parse("1.2.2-BR123").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2.10-BR123").unwrap().cmp(&parser.parse("1.2.2-BR123").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("1.2-BR123").unwrap().cmp(&parser.parse("1.2-BR123").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2-BR123").unwrap().cmp(&parser.parse("1.1-BR123").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2-BR123").unwrap().cmp(&parser.parse("1.3-BR123").unwrap()), Ordering::Less);

    assert_eq!(parser.parse("1-BR123").unwrap().cmp(&parser.parse("1-BR123").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1-BR123").unwrap().cmp(&parser.parse("2-BR123").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("2-BR123").unwrap().cmp(&parser.parse("1-BR123").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("2").unwrap().cmp(&parser.parse("2-BR123").unwrap()) != Ordering::Equal, true);
}

#[test]
fn branch_snapshot() {
    let parser = parser();
    assert_eq!(parser.parse("1.2.3-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.3-BR123-SNAPSHOT").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2.2-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.3-BR123-SNAPSHOT").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("1.2.3-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.2-BR123-SNAPSHOT").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2.10-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.2.2-BR123-SNAPSHOT").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("1.2-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.2-BR123-SNAPSHOT").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1.2-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.1-BR123-SNAPSHOT").unwrap()), Ordering::Greater);
    assert_eq!(parser.parse("1.2-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1.3-BR123-SNAPSHOT").unwrap()), Ordering::Less);

    assert_eq!(parser.parse("1-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1-BR123-SNAPSHOT").unwrap()), Ordering::Equal);
    assert_eq!(parser.parse("1-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("2-BR123-SNAPSHOT").unwrap()), Ordering::Less);
    assert_eq!(parser.parse("2-BR123-SNAPSHOT").unwrap().cmp(&parser.parse("1-BR123-SNAPSHOT").unwrap()), Ordering::Greater);

    assert_eq!(parser.parse("2-BR123").unwrap().cmp(&parser.parse("2-BR123-SNAPSHOT").unwrap()) != Ordering::Equal, true);
}
