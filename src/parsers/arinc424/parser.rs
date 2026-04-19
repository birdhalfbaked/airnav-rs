pub struct Arinc424Parser;

impl Arinc424Parser {
    pub fn parse(input: &[u8]) -> Option<()> {
        if input[0] != b'A' {
            return None;
        }
        Some(())
    }
}
