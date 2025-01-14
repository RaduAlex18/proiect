
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use crate::query_type::*;
use crate::byte_packet_buffer::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    }, // 0
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }, // 1
    NS {
        domain: String,
        host: String,
        ttl: u32,
    }, // 2
    CNAME {
        domain: String,
        host: String,
        ttl: u32,
    }, // 5
    MX {
        domain: String,
        priority: u16,
        host: String,
        ttl: u32,
    }, // 15
    AAAA {
        domain: String,
        addr: Ipv6Addr,
        ttl: u32,
    }, // 28
}
impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord, String> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;

        let qtype_num = buffer.read_word()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_word()?;
        let ttl = buffer.read_dword()?;
        let data_len = buffer.read_word()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_dword()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                );

                Ok(DnsRecord::A {
                    domain: domain,
                    addr: addr,
                    ttl: ttl,
                })
            }
            QueryType::AAAA => {
                let raw_addr1 = buffer.read_dword()?;
                let raw_addr2 = buffer.read_dword()?;
                let raw_addr3 = buffer.read_dword()?;
                let raw_addr4 = buffer.read_dword()?;
                let addr = Ipv6Addr::new(
                    ((raw_addr1 >> 16) & 0xFFFF) as u16,
                    ((raw_addr1 >> 0) & 0xFFFF) as u16,
                    ((raw_addr2 >> 16) & 0xFFFF) as u16,
                    ((raw_addr2 >> 0) & 0xFFFF) as u16,
                    ((raw_addr3 >> 16) & 0xFFFF) as u16,
                    ((raw_addr3 >> 0) & 0xFFFF) as u16,
                    ((raw_addr4 >> 16) & 0xFFFF) as u16,
                    ((raw_addr4 >> 0) & 0xFFFF) as u16,
                );

                Ok(DnsRecord::AAAA {
                    domain: domain,
                    addr: addr,
                    ttl: ttl,
                })
            }
            QueryType::NS => {
                let mut ns = String::new();
                buffer.read_qname(&mut ns)?;

                Ok(DnsRecord::NS {
                    domain: domain,
                    host: ns,
                    ttl: ttl,
                })
            }
            QueryType::CNAME => {
                let mut cname = String::new();
                buffer.read_qname(&mut cname)?;

                Ok(DnsRecord::CNAME {
                    domain: domain,
                    host: cname,
                    ttl: ttl,
                })
            }
            QueryType::MX => {
                let priority = buffer.read_word()?;
                let mut mx = String::new();
                buffer.read_qname(&mut mx)?;

                Ok(DnsRecord::MX {
                    domain: domain,
                    priority: priority,
                    host: mx,
                    ttl: ttl,
                })
            }
            QueryType::UNKNOWN(_) => {
                buffer.step(data_len as usize)?;

                Ok(DnsRecord::UNKNOWN {
                    domain: domain,
                    qtype: qtype_num,
                    data_len: data_len,
                    ttl: ttl,
                })
            }
        }
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<usize, String> {
        let start_pos = buffer.pos;

        match *self {
            DnsRecord::A {
                ref domain,
                ref addr,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_word(QueryType::A.to_num())?;
                buffer.write_word(1)?;
                buffer.write_dword(ttl)?;
                buffer.write_word(4)?;

                let octets = addr.octets();
                buffer.write_byte(octets[0])?;
                buffer.write_byte(octets[1])?;
                buffer.write_byte(octets[2])?;
                buffer.write_byte(octets[3])?;
            }
            DnsRecord::NS {
                ref domain,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_word(QueryType::NS.to_num())?;
                buffer.write_word(1)?;
                buffer.write_dword(ttl)?;

                let pos = buffer.pos;
                buffer.write_word(0)?;

                buffer.write_qname(host)?;

                let size = buffer.pos - (pos + 2);
                buffer.set_word(pos, size as u16)?;
            }
            DnsRecord::CNAME {
                ref domain,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_word(QueryType::CNAME.to_num())?;
                buffer.write_word(1)?;
                buffer.write_dword(ttl)?;

                let pos = buffer.pos;
                buffer.write_word(0)?;

                buffer.write_qname(host)?;

                let size = buffer.pos - (pos + 2);
                buffer.set_word(pos, size as u16)?;
            }
            DnsRecord::MX {
                ref domain,
                priority,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_word(QueryType::MX.to_num())?;
                buffer.write_word(1)?;
                buffer.write_dword(ttl)?;

                let pos = buffer.pos;
                buffer.write_word(0)?;

                buffer.write_word(priority)?;
                buffer.write_qname(host)?;

                let size = buffer.pos - (pos + 2);
                buffer.set_word(pos, size as u16)?;
            }
            DnsRecord::AAAA {
                ref domain,
                ref addr,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_word(QueryType::AAAA.to_num())?;
                buffer.write_word(1)?;
                buffer.write_dword(ttl)?;
                buffer.write_word(16)?;

                for octet in &addr.segments() {
                    buffer.write_word(*octet)?;
                }
            }
            DnsRecord::UNKNOWN { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }

        Ok(buffer.pos - start_pos)
    }
}
