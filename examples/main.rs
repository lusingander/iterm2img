use std::{fs::File, io::Read, io::Result};

fn main() -> Result<()> {
    let bytes = read_as_bytes()?;
    let encoded = iterm2img::from_bytes(bytes)
        .width(5)
        .preserve_aspect_ratio(true)
        .inline(true)
        .build();

    println!("image:\n{}", encoded);

    Ok(())
}

fn read_as_bytes() -> Result<Vec<u8>> {
    let mut f = File::open("./examples/image.jpg")?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}
