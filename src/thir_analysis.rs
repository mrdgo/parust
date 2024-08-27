use std::env;
use std::fs::File;
use std::io::prelude::*;
use toml::Table;
use toml::Value;
use walkdir::WalkDir;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::{
    collections::{BTreeMap, BTreeSet},
    path, process, str,
    sync::Arc,
};

use rustc_errors::registry;
use rustc_session::config;

struct FnCallCollector<'a, 'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    thir: &'a rustc_middle::thir::Thir<'tcx>,
}

impl<'thir, 'tcx: 'thir> rustc_middle::thir::visit::Visitor<'thir, 'tcx>
    for FnCallCollector<'thir, 'tcx>
{
    fn thir(&self) -> &'thir rustc_middle::thir::Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, ex: &'thir rustc_middle::thir::Expr<'tcx>) {
        rustc_middle::thir::visit::walk_expr(self, ex);
        match &ex.kind {
            rustc_middle::thir::ExprKind::Call {
                ty,
                fun: _,
                args: _,
                from_hir_call: _,
                fn_span: _,
            } => {
                // https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/struct.Ty.html
                match ty.kind() {
                    rustc_middle::ty::TyKind::FnDef(def_id, _args) => {
                        let Some(name) = self.tcx.opt_item_name(*def_id) else {
                            return;
                        };
                        println!("fname: {}", name);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn dependencies_to_extern_entries(
    cwd: String,
) -> BTreeMap<String, rustc_session::config::ExternEntry> {
    let mut externs = BTreeMap::new();
    if let Ok(mut file) = File::open(format!("{}/Cargo.toml", cwd)) {
        let mut cargo_toml: String = String::new();
        file.read_to_string(&mut cargo_toml)
            .expect("Could not read Cargo.toml");
        let value = cargo_toml.parse::<Table>().unwrap();
        match &value["dependencies"] {
            Value::Table(deps) => {
                deps.into_iter().for_each(|dep| {
                    if let Some(dir_ent) = WalkDir::new(format!("{}/target/debug/deps", cwd))
                        .into_iter()
                        .find(|e| match e {
                            Ok(ent) => {
                                let file_name = ent
                                    .file_name()
                                    .to_str()
                                    .unwrap_or("Not found.1")
                                    .to_string();
                                let pat = format!("lib{}", dep.0);
                                file_name.starts_with(&pat) && file_name.ends_with(".rlib")
                            }
                            Err(_) => false,
                        })
                    {
                        if let Ok(de) = dir_ent {
                            // println!("EXTERN dep: {}, path: {}", dep.0, de.file_name().to_str().unwrap_or("Not found.3"));
                            externs.insert(
                                dep.0.to_string(),
                                rustc_session::config::ExternEntry {
                                    location: rustc_session::config::ExternLocation::ExactPaths(
                                        BTreeSet::from([
                                            rustc_session::utils::CanonicalizedPath::new(de.path()),
                                        ]),
                                    ),
                                    is_private_dep: false,
                                    add_prelude: true,
                                    nounused_dep: false,
                                    force: false,
                                },
                            );
                        }
                    }
                })
            }
            _ => {
                // NOTE: there are no dependencies! yay.
            }
        }
    }

    externs
}

fn infer_search_paths(cwd: String) -> Vec<rustc_session::search_paths::SearchPathFile> {
    WalkDir::new("target/debug/deps")
        .into_iter()
        .filter_map(|entry| {
            let expected_entry = entry.expect("Expected entry");
            let name = expected_entry.file_name().to_str().unwrap();
            if name.contains(".") {
                // println!("SPF: {}", name);
                Some(rustc_session::search_paths::SearchPathFile {
                    path: format!("{}/target/debug/deps/{}", cwd, name).into(),
                    file_name_str: name.to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<rustc_session::search_paths::SearchPathFile>>()
}

pub fn thir_analysis(file_name: &String, input: String) {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();

    let cwd_path = env::current_dir().expect("Expected to read CWD.");
    let cwd = cwd_path.display();

    let externs = dependencies_to_extern_entries(cwd.to_string());
    let search_paths = infer_search_paths(cwd.to_string());

    // TODO: fix find libs, externs, and mods
    let config = rustc_interface::Config {
        opts: config::Options {
            crate_name: Some(String::from("parust")), // TODO: derive from input?
            crate_types: vec![rustc_session::config::CrateType::Executable],
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            incremental: None,
            edition: rustc_span::edition::Edition::Edition2021,
            unstable_features: rustc_feature::UnstableFeatures::Allow,
            diagnostic_width: Some(251),
            debuginfo: rustc_session::config::DebugInfo::LineTablesOnly,
            search_paths: vec![rustc_session::search_paths::SearchPath {
                kind: rustc_session::search_paths::PathKind::ExternFlag,
                dir: format!("{}/target/debug/deps", cwd).into(),
                files: search_paths,
            }],
            externs: rustc_session::config::Externs::new(externs),
            json_future_incompat: true,
            ..config::Options::default()
        },
        // TODO: Compiler uses File, is that the Error?
        input: config::Input::Str {
            name: rustc_span::FileName::Custom(file_name.to_string()),
            input,
        },
        crate_cfg: Vec::new(),
        crate_check_cfg: vec![
            "cfg(docsrs)".to_string(),
            "cfg(feature, values())".to_string(),
        ],
        output_dir: None,
        output_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
        lint_caps: rustc_hash::FxHashMap::default(),
        psess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(rustc_errors::codes::DIAGNOSTICS),
        expanded_args: Vec::new(),
        ice_file: None,
        hash_untracked_state: None,
        using_internal_features: Arc::default(),
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // let ast_krate = queries.parse().unwrap().get_mut().clone();
            // for item in ast_krate.items {
            //     println!("{}", item_to_string(&item));
            // }
            queries
                .global_ctxt()
                .unwrap()
                .enter(|tcx| tcx.resolver_for_lowering());

            queries.global_ctxt().unwrap().enter(|tcx| {
                rustc_interface::passes::write_dep_info(tcx);
            });

            queries.global_ctxt().unwrap().enter(|tcx| {
                let hir_krate = tcx.hir();
                for id in hir_krate.items() {
                    let item = hir_krate.item(id);
                    match item.kind {
                        // one of https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/hir/enum.ItemKind.html
                        rustc_hir::ItemKind::Fn(_signature, _generics, body_id) => {
                            // https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/hir/struct.Body.html
                            let func_hir = &tcx.hir().body(body_id);
                            println!("\nFunction definition: {}", item.ident);

                            let body_expr = func_hir.value;

                            match body_expr.kind {
                                rustc_hir::ExprKind::Block(_, _) => {
                                    let Ok((thir_ro, expr_id)) =
                                        &tcx.thir_body(body_expr.hir_id.owner)
                                    else {
                                        continue;
                                    };

                                    let thir = thir_ro.steal();

                                    // Visit by expression
                                    let mut fn_call_collector =
                                        FnCallCollector { tcx, thir: &thir };
                                    rustc_middle::thir::visit::walk_expr(
                                        &mut fn_call_collector,
                                        &thir[*expr_id],
                                    );

                                    // Visit by statements
                                    // for stmt in &*thir.stmts {
                                    //     rustc_middle::thir::visit::walk_stmt(
                                    //         &mut fn_call_collector,
                                    //         &stmt,
                                    //     );
                                    // }

                                    // Visit by blocks
                                    // for block in &thir.blocks {
                                    //     rustc_middle::thir::visit::walk_block(
                                    //         &mut fn_call_collector,
                                    //         &block,
                                    //     );
                                    // }
                                }
                                _ => {
                                    dbg!("Function body, that is no block..?");
                                }
                            }
                        }
                        _ => (),
                    }
                }
            });
        });
    });
}
