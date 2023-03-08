fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);

    let cwd = std::env::current_dir().unwrap();
    println!("we are in {}", cwd.display());
    println!("list of files:");
    for result in std::fs::read_dir(cwd).unwrap() {
        println!("{}", result.unwrap().path().display());
    }
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value);
    }

    let src = project_root::get_project_root().unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap())
        .join("built.rs");
    built::write_built_file_with_opts(&opts, src.as_ref(), &dst)
        .expect("Failed to acquire build-time information");
}
