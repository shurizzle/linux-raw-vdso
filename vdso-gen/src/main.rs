extern crate vdso_gen;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let dir = {
        let mut args = std::env::args_os();

        if args.len() != 2 {
            bail!("Directory not present");
        }
        _ = args.next().unwrap();
        args.next().unwrap()
    };

    let formatter = vdso_gen::format::Formatter::new()?;

    let base = PathBuf::from(dir);
    let mut def_dir = base.clone();
    def_dir.push("defs");

    for entry in std::fs::read_dir(def_dir).wrap_err("Cannot read definitions directory")? {
        let entry = entry.wrap_err("Cannot read definition file")?;

        {
            let md = entry.metadata().wrap_err("Cannot read definition file")?;
            if md.is_symlink() || !md.is_file() {
                continue;
            }
        }
        let path = entry.path();

        let tokens = {
            let mut content = String::new();
            {
                File::open(&path)
                    .wrap_err("Cannot read definition file")?
                    .read_to_string(&mut content)
                    .wrap_err("Cannot read definition file")?;
            }
            match content.parse::<proc_macro2::TokenStream>() {
                Ok(ts) => ts,
                Err(err) => {
                    bail!("{}: {}", path.display(), err);
                }
            }
        };
        {
            let gen = vdso_gen::vdso(tokens)?;
            let gen = formatter.format(gen)?;

            let mut dest = base.clone();
            dest.push("src");
            dest.push("arch");
            dest.push(path.file_name().expect("unreachable"));

            {
                let mut dest = File::create(dest).wrap_err("Failed to write generated file")?;
                dest.write_all(gen.as_bytes())
                    .wrap_err("Failed to write generated file")?;
            }
        }
    }

    Ok(())
}
