use std::process::{Command, Output};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

use super::version;
use super::image;

pub trait ImageRegistry {
    fn list(&self) -> image::Images;
    fn remove(&self, image_name: &str, ver: &version::Version, is_dryrun: bool) -> ();
}

pub struct DockerImageRegistry {
}

pub struct IbmCloudRegistry {
}

pub const DOCKER_IMAGE_REGISTRY: DockerImageRegistry = DockerImageRegistry {};

pub const IBM_CLOUD_REGISTRY: IbmCloudRegistry = IbmCloudRegistry {};

impl ImageRegistry for DockerImageRegistry {
    fn list(&self) -> image::Images {
        let out = Command::new("docker")
            .arg("images")
            .arg("--format")
            .arg("{{.ID}}\t{{.Repository}}\t{{.Tag}}")
            .output()
            .expect("Cannot run 'docker images'. Please check docker installation.");

        parse_docker_image_list(out)
    }
    
    fn remove(&self, image_name: &str, ver: &version::Version, is_dryrun: bool) -> () {
        let img = format!("{}:{}", image_name, ver);

        if is_dryrun {
            println!("docker rmi {}", img);
        } else {
            Command::new("docker")
                .arg("rmi")
                .arg(img)
                .output()
                .expect("Cannot run 'docker rmi {}'. Please check docker installation.");
        }
    }
}

impl ImageRegistry for IbmCloudRegistry {
    fn list(&self) -> image::Images {
        let out = Command::new("ibmcloud")
            .arg("cr")
            .arg("images")
            .arg("--format")
            .arg("{{.Digest}}\t{{.Repository}}\t{{.Tag}}")
            .output()
            .expect("Cannot run 'ibmcloud cr images'. Please check ibmcloud CLI installation.");

        parse_docker_image_list(out)
    }

    fn remove(&self, image_name: &str, ver: &version::Version, is_dryrun: bool) -> () {
        let img = format!("{}:{}", image_name, ver);

        if is_dryrun {
            println!("ibmcloud cr image-rm {}", img);
        } else {
            Command::new("ibmcloud")
                .arg("cr")
                .arg("image-rm")
                .arg(img)
                .output()
                .expect("Cannot run 'ibmcloud cr image-rm'. Please check ibmcloud CLI installation.");
        }
    }
}

fn parse_docker_image_list(out: Output) -> image::Images {
    let br = BufReader::new(out.stdout.as_slice());
    let mut hash: HashMap<String, HashSet<image::ImageEntry>> = HashMap::new();
    let ver_parser = version::parser();
        
    for (_, line) in br.lines().enumerate() {
        let l = line.unwrap();
        let mut z = l.split('\t');
        let id = z.next().unwrap();
        let repository = z.next().unwrap();
        let tag = z.next().unwrap();
        match ver_parser.parse(tag) {
            None => println!("Version(={}) is unrecognized ignored: {}", tag, l),
            Some(ver) => {
                match hash.get_mut(repository) {
                    Some(entry) => {
                        entry.insert(image::ImageEntry {id: id.to_string(), ver: ver});
                    },
                    None => {
                        let mut set = HashSet::new();
                        set.insert(image::ImageEntry {id: id.to_string(), ver: ver});
                        hash.insert(repository.to_string(), set);
                    }
                }
            }
        }
    }
        
    image::Images {entries: hash}
}
