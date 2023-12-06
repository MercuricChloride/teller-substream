use std::{
    fs::{self, File},
    io::Read,
};

fn main() -> Result<(), anyhow::Error> {
    //A list of paths we want to "upgrade"
    let upgrade_paths = vec!["schema"]
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();

    for path in upgrade_paths.iter() {
        let path = format!("src/pb/{}.rs", path);

        let mut file = File::open(&path)?;

        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let new_contents = contents
            .replace(
                "use derive_more::{From, Into};\n use ::substreams_helpers_macros::Map;\n",
                "// @generated",
            )
            .replace(
                "// @generated",
                "use derive_more::{From, Into};\n use ::substreams_helpers_macros::Map;\n",
            )
            .replace("::prost::Message, From, Into", "::prost::Message")
            .replace("::prost::Message", "::prost::Message, From, Into");

        fs::write(path, new_contents)?;
    }

    Ok(())
}
