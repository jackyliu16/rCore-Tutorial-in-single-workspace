﻿use crate::*;
use command_ext::{Cargo, CommandExt};
use once_cell::sync::Lazy;
use std::{ffi::OsStr, fs::File, io::Write, path::PathBuf};

const PACKAGE: &str = "user_lib";
static USER: Lazy<PathBuf> = Lazy::new(|| PROJECT.join("user"));

fn build_all(release: bool, base_addr: impl Fn(u64) -> u64) -> Vec<PathBuf> {
    let mut names = USER
        .join("src/bin")
        .read_dir()
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().map_or(false, |t| t.is_file()))
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(OsStr::new("rs")))
        .map(|path| path.file_prefix().unwrap().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    names.sort_unstable();
    names
        .into_iter()
        .enumerate()
        .map(|(i, name)| build_one(name, release, base_addr(i as _)))
        .collect()
}

fn build_one(name: impl AsRef<OsStr>, release: bool, base_address: u64) -> PathBuf {
    let name = name.as_ref();
    println!("build {name:?} at {base_address:#x}");
    Cargo::build()
        .package(PACKAGE)
        .target(TARGET_ARCH)
        .arg("--bin")
        .arg(name)
        .conditional(release, |cargo| {
            cargo.release();
        })
        .env("BASE_ADDRESS", base_address.to_string())
        .invoke();
    let elf = TARGET
        .join(if release { "release" } else { "debug" })
        .join(name);
    strip_all(elf)
}

pub fn build_for(ch: u8, release: bool) {
    let (base, step, bins) = match ch {
        2 => (CH2_APP_BASE, 0, build_all(release, |_| CH2_APP_BASE)),
        3 => (
            CH3_APP_BASE,
            CH3_APP_STEP,
            build_all(release, |i| CH3_APP_BASE + i * CH3_APP_STEP),
        ),
        _ => unreachable!(),
    };
    if bins.is_empty() {
        return;
    }
    let asm = TARGET
        .join(if release { "release" } else { "debug" })
        .join("app.asm");
    let mut ld = File::create(asm).unwrap();
    writeln!(
        ld,
        "\
    .global apps
    .section .data
    .align 3
apps:
    .quad {base:#x}
    .quad {step:#x}
    .quad {}",
        bins.len(),
    )
    .unwrap();

    (0..bins.len()).for_each(|i| {
        writeln!(
            ld,
            "\
    .quad app_{i}_start"
        )
        .unwrap()
    });

    writeln!(
        ld,
        "\
    .quad app_{}_end",
        bins.len() - 1
    )
    .unwrap();

    bins.iter().enumerate().for_each(|(i, path)| {
        writeln!(
            ld,
            "
app_{i}_start:
    .incbin {path:?}
app_{i}_end:",
        )
        .unwrap();
    });
}
