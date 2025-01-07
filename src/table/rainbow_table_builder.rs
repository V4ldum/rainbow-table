use crate::table::hash_reduce::{Clear, Hash};
use crate::{RainbowTable, RainbowTableConfig};
use anyhow::{anyhow, Result};
use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;
use std::fs;

pub struct RainbowTableBuilder(RainbowTableConfig);

impl RainbowTableBuilder {
    pub fn new(config: RainbowTableConfig) -> Self {
        if !config.charset.is_ascii() {
            panic!("Charset should be ASCII")
        }

        let mut config = config;
        if config.charset.is_empty() {
            // default to lowercase
            config.charset = "abcdefghijklmnopqrstuvwxyz".to_string();
        }

        // Sort charset to have same behavior for the same charset in a different order
        config.sort_charset();

        RainbowTableBuilder(config)
    }
    pub fn generate(self) -> RainbowTable {
        let config = &self.0;
        let mut rng = rand::rng();
        let mut chains = Vec::with_capacity(config.chain_number);

        for i in 0..self.0.chain_number {
            #[cfg(debug_assertions)]
            let seed = if config.debug {
                if i == 0 {
                    Clear::new("ObdcZGEh")
                } else {
                    Clear::new("2WA9Pfo5")
                }
            } else {
                self.generate_random_seed(&mut rng)
            };
            #[cfg(not(debug_assertions))]
            let seed = self.generate_random_seed(&mut rng);

            #[cfg(debug_assertions)]
            if config.debug {
                print!("{seed}");
            }

            let mut hash;
            let mut reduced = seed.clone();
            for i in 0..config.chain_length {
                hash = Hash::new(&reduced);
                reduced = Clear::from_hash(&hash, config.password_length, i, &config.charset);

                #[cfg(debug_assertions)]
                if config.debug {
                    print!("-> {hash} -> {reduced}")
                }
            }
            chains.push((seed.to_string(), reduced.to_string()));

            #[cfg(debug_assertions)]
            if config.debug {
                println!();
            }
        }

        RainbowTable { config: self.0, chains }
    }

    fn generate_random_seed(&self, rng: &mut ThreadRng) -> Clear {
        Clear::new(
            &(0..self.0.password_length)
                .map(|_| self.0.charset.chars().choose(rng).expect("charset should not be empty"))
                .collect::<String>(),
        )
    }

    pub fn from_file(path: &str) -> Result<RainbowTable> {
        let file_content = fs::read_to_string(path)?;
        let mut lines = file_content.lines();

        // First line is the hashing algorithm, we skip it
        lines.next().ok_or(anyhow!("invalid file format"))?;

        // Second line is password length followed by charset
        let mut line = lines.next().ok_or(anyhow!("invalid file format"))?.split(' ');
        let (password_length, charset) = (
            line.next().ok_or(anyhow!("invalid format"))?.parse()?,
            line.next().ok_or(anyhow!("invalid format"))?.to_owned(),
        );

        // Third line is number of chains followed by chain length
        let mut line = lines.next().ok_or(anyhow!("invalid file format"))?.split(' ');
        let (chain_number, chain_length) = (
            line.next().ok_or(anyhow!("invalid format"))?.parse()?,
            line.next().ok_or(anyhow!("invalid format"))?.parse()?,
        );

        let config = RainbowTableConfig {
            charset,
            chain_length,
            chain_number,
            password_length,
            #[cfg(debug_assertions)]
            debug: false,
        };

        // The rest is start and end of a chain (one per line)
        let mut chains = Vec::with_capacity(chain_number);
        for _ in 0..chain_number {
            let mut line = lines.next().ok_or(anyhow!("invalid format"))?.split(':');
            let chain = (
                line.next().ok_or(anyhow!("invalid format"))?.to_owned(),
                line.next().ok_or(anyhow!("invalid format"))?.to_owned(),
            );

            chains.push(chain)
        }

        Ok(RainbowTable { config, chains })
    }
}
