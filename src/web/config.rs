use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Web {
    pub static_path: String,

    #[serde(with = "de_listen_addr")]
    pub listen_addr: ListenAddr,
}

pub type ListenAddr = ([u8; 4], u16);

mod de_listen_addr {
    use super::ListenAddr;
    use serde::{self, Deserialize, Deserializer, de::Error};

    /// Convert "a.b.c.d:xxxx" into `ListenAddr`
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<ListenAddr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(":").collect();
        
        if parts.len() == 2 {
            let ip_parts: Vec<&str> = parts[0].split(".").collect();
            if ip_parts.len() == 4 {
                let mut ip: [u8; 4] = Default::default();
                for i in 0..4 {
                    ip[i] = ip_parts[i].parse().map_err(Error::custom)?;                    
                }
                let port = parts[1].parse().map_err(Error::custom)?;

                return Ok( (ip, port) )    
            }
        }
        
        Err(Error::custom(format!("expected listen address like \"1.2.3.4:1234\", but got \"{}\"", s)))
    }
}