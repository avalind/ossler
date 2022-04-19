use std::collections::HashMap;

use csv::ReaderBuilder;
use serde::Deserialize;

use anyhow::Result;

#[derive(Debug)]
pub struct ProbeInfo {
    pub chrom: u8,
    pub pos: u32,
    pub allele_a: u8,
    pub allele_b: u8,
}

impl ProbeInfo {
    pub fn new(chrom: u8, pos: u32, allele_a: u8, allele_b: u8)
    -> ProbeInfo {
        ProbeInfo{ chrom, pos, allele_a, allele_b }
    }
}

#[derive(Debug)]
pub struct Manifest {
    pub probes: HashMap<String, ProbeInfo>,
}

impl Manifest {
    pub fn new() -> Manifest {
        let m = Manifest{ probes: HashMap::new() };
        m
    }

    pub fn from_csv(path: &str) -> Result<Manifest> {
        let mut manifest = Manifest::new();
        let mut rdr = ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        /* should we just use serde? */
        println!("Created reader!");
        for res in rdr.records() {
            let record = res?;
            let tag = &record[0];
            
            /* @see https://media.affymetrix.com/support/developer/powertools/changelog/oschp.html */
            let chrom: u8 = match &record[2] {
                "X" => 24,
                "Y" => 25,
                val @ _ => val.parse()?, 
            };

            let pos: u32 = record[3].parse()?;
            let allele_a: u8 = record[11].as_bytes()[0];
            let allele_b: u8 = record[12].as_bytes()[0];
            
            manifest.add_probe(tag.to_owned(), ProbeInfo::new(
                chrom,
                pos,
                allele_a,
                allele_b
            ));
        }
        return Ok(manifest);
    }

    pub fn add_probe(&mut self, key: String, probe: ProbeInfo) {
        self.probes.insert(key, probe);
    }
}
