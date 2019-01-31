use std::env;
use std::io;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Any(),
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<()> for Error {
    fn from(_: ()) -> Error {
        Error::Any()
    }
}
fn main() -> Result<(), Error> {
    let current = env::current_dir()?;
    let name = current.file_name().ok_or(())?;

    // creates a temporary which is freed while still in use
    // let name = env::current_dir()?.file_name().ok_or(())? ;

    println!("current: {:?}", name);
    Ok(())
}
