use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use anyhow::{bail, Context};
use quote::ToTokens;
use syn::{ItemUse, UseTree};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <source> <filename>", args[0]);
    }

    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).context(format!("Cannot open file {}", filename))?;

    // Парсинг AST файла с помощью `syn`
    let parsed_file = syn::parse_file(&file_content)
        .context(format!("Failed to parse Rust file {}", filename))?;

    // Вычисление полного пути файла в пространстве crate
    let full_path = compute_full_crate_path(filename)?;
    let parent_path = compute_parent_path(&full_path)?;

    println!("  full_path: {}", full_path);
    println!("parent_path: {}", parent_path);

    // Обработка блока `use`
    let mut need_file_update = false;
    let mut modified_file = file_content.clone(); // Создаем копию для модификации
    for item in parsed_file.items {
        if let syn::Item::Use(item_use) = item {
            let old_stream = item_use
                .to_token_stream()
                .to_string()
                .replace("use ", "use|")
                .replace(" ", "")
                .replace("|", " ");
            let mut use_list = flatten_use_tree(&item_use);

            let mut found = false;
            for mut elem in &mut use_list {
                if !elem.starts_with(&parent_path) {
                    continue;
                }

                found = true;
                *elem = elem.replace(&parent_path, "super");
            }

            if !found {
                continue;
            }

            let mut new_stream = String::from("");
            for (ind, elem) in use_list.iter().enumerate() {
                let last_elem = use_list.len() - 1 == ind;
                if last_elem {
                    new_stream.push_str(format!("use {};", elem).as_str());
                } else {
                    new_stream.push_str(format!("use {};\n", elem).as_str());
                }
            }

            println!(" - repl\nfrom=`{}`\n  to=`{}`", old_stream, new_stream);
            modified_file = modified_file.replace(&old_stream, &new_stream);
            need_file_update = true;
        }
    }

    if !need_file_update {
        return Ok(());
    }

    // Сохранение измененного файла
    let new_file_name = filename.clone();
    let mut file = File::create(new_file_name.clone())
        .context(format!("Cannot write to file {}", new_file_name))?;
    file.write_all(modified_file.as_bytes())
        .context("Failed to write modified content")?;
    println!("file updated!: {}", new_file_name);

    Ok(())
}

// Вычисление полного пути файла в пространстве crate
fn compute_full_crate_path(filename: &str) -> anyhow::Result<String> {
    let path = Path::new(filename);
    let mut components = path.components().map(|c| c.as_os_str().to_str().unwrap());

    // Находим индекс `src` в пути
    if let Some(src_index) = components.clone().position(|c| c == "src") {
        // Строим путь начиная от `crate::`
        let relative_path: Vec<_> = components.clone().skip(src_index + 1).collect();
        let crate_path = format!("crate::{}", relative_path.join("::").replace(".rs", ""));
        Ok(crate_path)
    } else {
        bail!("File path must contain `src` directory.");
    }
}

// Вычисление пути к родительскому модулю
fn compute_parent_path(full_path: &str) -> anyhow::Result<String> {
    let repl_path = full_path.replace("::", "/");
    let parent_path = Path::new(repl_path.as_str())
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot find parent path"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid parent path"))?;
    Ok(parent_path.replace("/", "::"))
}

/// Разворачивает `ItemUse` в плоский список строковых импортов.
fn flatten_use_tree(item_use: &ItemUse) -> Vec<String> {
    let mut imports = Vec::new();
    flatten_use_tree_recursively(&item_use.tree, String::new(), &mut imports);
    imports
}

/// Рекурсивно разворачивает дерево `UseTree` в список импортов.
fn flatten_use_tree_recursively(tree: &UseTree, prefix: String, imports: &mut Vec<String>) {
    match tree {
        UseTree::Path(path) => {
            let new_prefix = if prefix.is_empty() {
                path.ident.to_string()
            } else {
                format!("{}::{}", prefix, path.ident)
            };
            flatten_use_tree_recursively(&*path.tree, new_prefix, imports);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                flatten_use_tree_recursively(item, prefix.clone(), imports);
            }
        }
        UseTree::Name(name) => {
            imports.push(if prefix.is_empty() {
                name.ident.to_string()
            } else {
                format!("{}::{}", prefix, name.ident)
            });
        }
        UseTree::Rename(rename) => {
            imports.push(if prefix.is_empty() {
                rename.ident.to_string()
            } else {
                format!("{}::{}", prefix, rename.ident)
            });
        }
        UseTree::Glob(_) => {
            // Не обрабатываем `*`, так как это не конкретный импорт
        }
    }
}
