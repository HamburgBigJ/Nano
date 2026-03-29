use std::fs;
use std::path::Path;

fn main() {
    let src = Path::new("assets");

    let dest = Path::new("../static/assets");

    if src.exists() {
        println!("cargo:warning=Copying assets from {:?} to {:?}", src, dest);
        copy_dir_recursive(src, dest).expect("Failed to copy assets");
    } else {
        println!("cargo:warning=Assets folder not found, skipping copy");
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
        println!("cargo:warning=Copied: {}", path.display());
    }

    Ok(())
}