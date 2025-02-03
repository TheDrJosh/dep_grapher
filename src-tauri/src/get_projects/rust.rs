use std::path::Path;

pub async fn read_rust_project_info(path: &Path) -> Vec<String> {
    let mut names = vec![];

    let Ok(manifest) = cargo_toml::Manifest::from_path(path.join("Cargo.toml")) else {
        return names;
    };

    // manifest.complete_from_path(path).ok()?;

    if let Some(package) = manifest.package {
        names.push(package.name);
    }

    if let Some(workspace) = manifest.workspace {
        let exclude_matchers = workspace
            .exclude
            .iter()
            .filter_map(|exclude| globmatch::Builder::new(&exclude).build(path).ok())
            .collect::<Vec<_>>();

        for member in workspace.members {
            let Ok(member_matcher) = globmatch::Builder::new(&member).build(path) else {
                //TODO - add log
                continue;
            };

            'member_loop: for member_path in member_matcher.into_iter() {
                let Ok(member_path) = member_path else {
                    //TODO - add log
                    continue;
                };

                for exclude_matcher in &exclude_matchers {
                    if exclude_matcher.is_match(member_path.clone()) {
                        continue 'member_loop;
                    }
                }

                names.append(&mut Box::pin(read_rust_project_info(&member_path)).await);
            }
        }
    }

    todo!()
}
