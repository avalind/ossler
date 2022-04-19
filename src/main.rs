use std::env;
use std::collections::HashMap;

use itertools::izip;

use hdf5::types::{TypeDescriptor as TD, *};
use hdf5::{File, H5Type, Datatype};
use ndarray::Array1;
use csv::ReaderBuilder;
use anyhow::Result;

mod record;
mod transform;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let handle = match hdf5::file::File::open(filename) {
        Ok(h) => h,
        Err(_s) => panic!("boeg"),
    };
 
    let genotype_calls = record::oschp::Call::from_oschp_file(&handle)?;
    let allelic_data = record::oschp::AllelicData::from_oschp_file(&handle)?;
    let copynumber_data = record::oschp::CopyNumber::from_oschp_file(&handle)?;
    let chroms = record::oschp::ChromosomeSummary::from_oschp_file(&handle)?;
    let segments = record::oschp::CopyNumberSegment::from_oschp_file(&handle)?;
    let loh = record::oschp::LOHSegment::from_oschp_file(&handle)?;   
    let mani = record::manifest::Manifest::from_csv(&args[2])?;
    let tab = transform::utils::build_chrom_table(&chroms);

    //let mut genotype_map = HashMap::new();
    //let mut allelic_map = HashMap::new();
    //f.map(|elem| { genotype_map.insert(elem.ProbeSetName, elem) });

    for (gt, allelic, cnv) in izip!(&genotype_calls, &allelic_data, &copynumber_data) {
        let probe_info = match mani.probes.get(gt.ProbeSetName.as_str()) {
            Some(p_info) => p_info,
            None => {
                println!("probeset {} not found in manifest - skipping.", gt.ProbeSetName);
                continue;
            },
        };
        let m = record::snp::Marker::from_hdf5_trio(gt, cnv, allelic, probe_info);
        println!("{:?}", m);
    }
    //println!("{:?}", chroms);
    //println!("{:?}", handle.group("Segments")?.dataset("TuscanCN")?.dtype()?.to_descriptor()?);

    //let mani = record::manifest::Manifest::from_csv(&args[2]);
    //println!("{:?}", mani);
    Ok(())
}
