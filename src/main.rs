use std::process::Command;

mod arg;
mod image;
mod version;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: arg::Args = arg::parse_arg();

    if args.show_version {
        println!("{}", VERSION);
    } else {
        let images = image::perform();
        images.delete(args.keep_count, args.keep_count_snapshot, |repo, ver| {
            if args.is_dry_run {
                println!("delete {}:{}", repo, ver);
            } else {
                let img = format!("{}:{}", repo, ver);
                Command::new("docker")
                    .arg("rmi")
                    .arg(img)
                    .output()
                    .expect("Cannot run 'docker rmi {}'. Please check docker installation.");
            }
        });
    }
}
