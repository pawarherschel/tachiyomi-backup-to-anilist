use std::io::Result;

fn main() -> Result<()> {
    match prost_build::compile_protos(&[r"./assets/schema.proto"], &["assets/"]) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    }
    Ok(())
}
