extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::{env, fs};
use std::{path, process, str, sync::Arc};

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

pub fn thir_analysis() {
    let args: Vec<String> = env::args().collect();
    let file_name = &String::from(&args[1]);

    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();

    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            ..config::Options::default()
        },
        input: config::Input::Str {
            name: rustc_span::FileName::Custom(file_name.to_string()),
            input: contents,
        },
        crate_cfg: Vec::new(),
        crate_check_cfg: Vec::new(),
        output_dir: None,
        output_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
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
                                rustc_hir::ExprKind::Block(block, _) => {
                                    match block.rules {
                                        rustc_hir::BlockCheckMode::UnsafeBlock(_) => continue,
                                        _ => (),
                                    }

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
            })
        });
    });
}
