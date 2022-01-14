use memmap::MmapOptions;
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let file = File::open("/tmp/mmap-example.db")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    println!("{:?}", &mmap[10..80]);

    Ok(())
}
