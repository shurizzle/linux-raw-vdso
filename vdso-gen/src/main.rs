extern crate vdso_gen;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = {
        let mut args = std::env::args_os();

        if args.len() != 2 {
            eprintln!("Directory not present");
            std::process::exit(1);
        }
        _ = args.next().unwrap();
        args.next().unwrap()
    };

    let formatter = vdso_gen::format::Formatter::new();

    let base = PathBuf::from(dir);
    let mut def_dir = base.clone();
    def_dir.push("defs");

    for entry in std::fs::read_dir(def_dir)? {
        let entry = entry?;

        {
            let md = entry.metadata()?;
            if md.is_symlink() || !md.is_file() {
                continue;
            }
        }
        let path = entry.path();

        let tokens = {
            let mut content = String::new();
            {
                File::open(&path)?.read_to_string(&mut content)?;
            }
            content.parse::<proc_macro2::TokenStream>()?
        };
        {
            let gen = vdso_gen::vdso(tokens).to_string();
            let gen = formatter.format(gen);

            let mut dest = base.clone();
            dest.push("src");
            dest.push("arch");
            dest.push(path.file_name().unwrap());

            {
                let mut dest = File::create(dest)?;
                dest.write_all(gen.as_bytes())?;
            }
        }
    }

    Ok(())
}
