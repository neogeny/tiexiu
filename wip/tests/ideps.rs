// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/ideps_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::tool::ideps::{findeps, moduledeps, render, Dependency};
use std::path::PathBuf;

#[test]
fn test_moduledeps_collects_absolute_and_relative_imports(tmp_path: PathBuf) {
    let pkg = tmp_path / "pkg";
    std::fs::create_dir(&pkg).ok();
    let mod_file = pkg / "mod.py";
    std::fs::write(
        &mod_file,
        "from __future__ import annotations\nimport os\nfrom . import sibling\n",
    )
    .ok();

    let result = moduledeps("pkg.mod", &mod_file).expect("Failed to get module deps");
    assert_eq!(result.name, "pkg.mod");
    let imports: Vec<_> = result.imports.iter().collect();
    assert!(imports.contains(&Dependency::new("os", false)));
    assert!(imports.contains(&Dependency::new("pkg.sibling", true)));
}

#[test]
fn test_findeps_uses_module_names(tmp_path: PathBuf) {
    let pkg = tmp_path / "pkg";
    std::fs::create_dir(&pkg).ok();
    let a = pkg / "a.py";
    let b = pkg / "b.py";
    std::fs::write(&a, "import os\n").ok();
    std::fs::write(&b, "import sys\n").ok();

    let paths = vec![PathBuf::from("pkg/a.py"), PathBuf::from("pkg/b.py")];
    let results = findeps(&paths).expect("Failed to find dependencies");
    let names: Vec<_> = results.iter().map(|m| m.name.clone()).collect();
    assert_eq!(names, vec!["pkg.a", "pkg.b"]);
}

#[test]
fn test_render_builds_tree(tmp_path: PathBuf) {
    let pkg = tmp_path / "pkg";
    std::fs::create_dir(&pkg).ok();
    let a = pkg / "a.py";
    let b = pkg / "b.py";
    std::fs::write(
        &a,
        "from __future__ import annotations\nimport os\nfrom . import b\n",
    )
    .ok();
    std::fs::write(&b, "import sys\n").ok();

    let paths = vec![PathBuf::from("pkg/a.py"), PathBuf::from("pkg/b.py")];
    let results = findeps(&paths).expect("Failed to find dependencies");
    let tree = render(&results).expect("Failed to render tree");
    assert_eq!(tree.label, "pkg");
}
