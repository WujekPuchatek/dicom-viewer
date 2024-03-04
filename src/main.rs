use std::fs::File;
use memmap2::Mmap;

fn main()  -> std::io::Result<()>
{
    let path = "C:/Users/medapp/Desktop/CT/im001.dcm";
    let file = File::open(path).expect("failed to open the file");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };

    let data = &mmap[..];

    Ok(())
}
