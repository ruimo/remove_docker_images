mod arg;
mod image;
mod version;
mod docker_registry_type;
mod image_registry;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: arg::Args = arg::parse_arg();

    if args.show_version {
        println!("{}", VERSION);
    } else {
        let registry: &dyn image_registry::ImageRegistry = match args.repository_type {
            docker_registry_type::DockerRegistryType::Docker => &image_registry::DOCKER_IMAGE_REGISTRY,
            docker_registry_type::DockerRegistryType::IbmCloudRegistry => &image_registry::IBM_CLOUD_REGISTRY,
        };

        let images = registry.list();
        images.delete(args.keep_count, args.keep_count_snapshot, |repo, ver| {
            registry.remove(repo, ver, args.is_dry_run);
        });
    }
}
