use std::collections::HashMap;
use ndarray::Array1;

#[derive(Debug)]
pub struct Marker {
    chrom: u8,
    position: u32,
    lrr: f32,
    weighted_lrr: f32,
    allelic_diff: f32,
    baf: f32,
    call: u8,
    conf: f32,
    a_signal: f32,
    b_signal: f32,
    probe_name: String,
    a_allele: u8,
    b_allele: u8,
}

impl Marker {
    pub fn new() -> Marker {
        let m = Marker {
            chrom: 0,
            position: 0,
            lrr: 0.0,
            weighted_lrr: 0.0,
            allelic_diff: 0.0,
            baf: 0.0,
            call: 0,
            conf: 0.0,
            a_signal: 0.0,
            b_signal: 0.0,
            probe_name: "".to_string(),
            a_allele: 0,
            b_allele: 0,
        };

        m
    }

    pub fn from_hdf5_trio(
        call: &super::oschp::Call,
        cnv: &super::oschp::CopyNumber,
        allelic: &super::oschp::AllelicData,
        probe_info: &super::manifest::ProbeInfo) -> Marker
    {
        Marker {
            chrom: probe_info.chrom,
            position: probe_info.pos,
            lrr: cnv.Log2Ratio,
            weighted_lrr: cnv.WeightedLog2Ratio,
            allelic_diff: allelic.AllelicDifference,
            baf: allelic.BAF,
            call: call.Call,
            conf: call.Confidence,
            a_signal: call.ASignal,
            b_signal: call.BSignal,
            probe_name: call.ProbeSetName.as_str().to_owned(),
            a_allele: probe_info.allele_a,
            b_allele: probe_info.allele_b,
        }
    }
    
    pub fn to_vcf_record(&self) {
        
    }
}
