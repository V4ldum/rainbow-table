use crate::table::hash_reduce::{Clear, Hash};
use crate::table::rainbow_table_config::RainbowTableConfig;
use anyhow::Result;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

pub struct RainbowTable {
    pub(crate) config: RainbowTableConfig,
    pub(crate) chains: Vec<(String, String)>,
}

impl RainbowTable {
    fn find_clear_from_hash(&self, to_crack: &str, chain_index: usize) -> Clear {
        let start = Clear::new(&self.chains[chain_index].0);

        let to_crack = Hash::from_hash_str(to_crack);
        let mut reduced = start;
        let mut hash;

        for i in 0..self.config.chain_length {
            hash = Hash::new(&reduced);
            let reduced_tmp =
                Clear::from_hash(&hash, self.config.password_length, i, &self.config.charset);

            if hash == to_crack {
                return reduced;
            } else {
                reduced = reduced_tmp;
            }
        }
        panic!("A cleartext for {to_crack} should have been found at this point");
    }

    pub fn crack(&self, hash: &str) -> Result<String, String> {
        for (index, chain) in self.chains.iter().enumerate() {
            let end = Clear::new(&chain.1);

            for i in 1..=self.config.chain_length {
                let mut current_hash = Hash::from_hash_str(hash);
                let mut reduced;

                for j in (self.config.chain_length - i)..self.config.chain_length {
                    reduced = Clear::from_hash(
                        &current_hash,
                        self.config.password_length,
                        j,
                        &self.config.charset,
                    );

                    if reduced == end {
                        let clear = self.find_clear_from_hash(hash, index);
                        return Ok(clear.to_string());
                    }

                    current_hash = Hash::new(&reduced);
                }
            }
        }

        Err(format!("{hash}: cleartext not found in the table"))
    }

    pub fn write_to_file(&self, file_name: &str) -> Result<()> {
        if file_name.contains("/") || file_name.contains("\\") {
            panic!("No path allowed");
        }

        let mut file = File::create(format!("./{}", file_name))?;
        write!(file, "{self}")?;

        Ok(())
    }
}

impl Display for RainbowTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MD5")?;
        writeln!(f, "{} {}", self.config.password_length, self.config.charset)?;
        writeln!(
            f,
            "{} {}",
            self.config.chain_number, self.config.chain_length
        )?;
        for chain in &self.chains {
            writeln!(f, "{}:{}", chain.0, chain.1)?;
        }

        Ok(())
    }
}
