use std::io;

trait Serialize {
    fn size(&self) -> u32;
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}
