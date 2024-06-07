
use crate::query_type::*;
use crate::byte_packet_buffer::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}
impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion {
            name: name,
            qtype: qtype,
        }
    }
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<&mut Self, String> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_word()?); // qtype
        let _ = buffer.read_word()?; // class

        Ok(self)
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), String> {
        buffer.write_qname(&self.name)?;

        let typenum = self.qtype.to_num();
        buffer.write_word(typenum)?;
        buffer.write_word(1)?;

        Ok(())
    }
}
