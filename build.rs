use std::fs::read_dir;
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    for file in read_dir("abis")? {
        let file = file?;
        let path = file.path();
        let path = path.to_str().unwrap();
        let name = path.split("/").last().unwrap().split(".").next().unwrap();

        if path.ends_with(".bak") {
            continue;
        }

        println!("Processing {}...", name);

        Abigen::new(name, path)
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(format!("src/abi/{}.rs", name))?;

        println!("Done!");
    }

    Ok(())
}
