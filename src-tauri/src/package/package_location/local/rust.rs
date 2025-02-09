use std::path::Path;

use log::error;

use crate::{
    package::{
        package_location::{registry::RegistryPackageLocation, PackageLocation},
        Dependency, Language, Package,
    },
    registry::Registry,
};

use super::LocalPackageLocation;

pub async fn resolve_rust_package_info(path: &Path, location: &PackageLocation) -> Vec<Package> {
    let mut names = vec![];

    let Ok(manifest) = cargo_toml::Manifest::from_path(path.join("Cargo.toml")) else {
        return names;
    };

    if let Some(package) = manifest.package {
        manifest.dependencies.iter().map(|(name, dep)| Dependency {
            location: PackageLocation::Registry(RegistryPackageLocation {
                registry: Registry {
                    registry_type: crate::registry::RegistryType::Cargo,
                    custom_url: None,
                },
                name: dep.package().unwrap().to_string(),
                version: dep.detail().unwrap().version.as_ref().unwrap().to_string(),
            }),
            name: name.clone(),
            stage: crate::package::Stage::Default,
            required_features: vec![],
            platform_requirements: crate::package::PlatformRequirements::Blacklist(vec![]),
            optional: dep.optional(),
            with_features: vec![],
        });
        names.push(Package {
            location: location.clone(),
            name: package.name.clone(),
            version: package.version().to_string(),
            description: package.description().map(|s| s.to_owned()),
            authors: Vec::from(package.authors()),
            language: Language::Rust,
            dependencies: vec![], //TODO - figure out deps
        });
    }

    if let Some(workspace) = manifest.workspace {
        let exclude_matchers = workspace
            .exclude
            .iter()
            .filter_map(|exclude| globmatch::Builder::new(exclude).build(path).ok())
            .collect::<Vec<_>>();

        for member in workspace.members {
            let Ok(member_matcher) = globmatch::Builder::new(&member).build(path) else {
                error!("invalid glob {}", member);
                continue;
            };

            'member_loop: for member_path in member_matcher.into_iter() {
                let member_path = match member_path {
                    Ok(member_path) => member_path,
                    Err(err) => {
                        error!("glob error {}", err);
                        continue;
                    }
                };

                for exclude_matcher in &exclude_matchers {
                    if exclude_matcher.is_match(member_path.clone()) {
                        continue 'member_loop;
                    }
                }

                names.append(
                    &mut Box::pin(resolve_rust_package_info(
                        &member_path,
                        &match location {
                            PackageLocation::Local(_) => {
                                PackageLocation::Local(LocalPackageLocation {
                                    path: member_path.clone(),
                                })
                            }
                            PackageLocation::Git(_) => location.clone(),
                            _ => unreachable!(),
                        },
                    ))
                    .await,
                );
            }
        }
    }

    names
}
