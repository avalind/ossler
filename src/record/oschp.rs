#![allow(non_snake_case)]
use hdf5::{File, H5Type};
use hdf5::types::FixedAscii;
use hdf5::types::VarLenAscii;
use ndarray::Array1;
use anyhow::Result;


#[derive(H5Type, Clone, Debug, Copy)]
#[repr(C)]
pub struct Call {
    Index: u32,
    pub ProbeSetName: hdf5::types::FixedAscii<12>,
    pub Call: u8,
    pub Confidence: f32,
    ForcedCall: u8,
    pub ASignal: f32,
    pub BSignal: f32,
    SignalStrength: f32,
    Contrast: f32,
}

impl Call {
    pub fn from_oschp_file(handle: &hdf5::file::File) -> Result<Array1<Call>, hdf5::Error> {
        return Ok(handle.group("Genotyping")?.dataset("Calls")?.read_1d::<Call>()?);
    }    
}

#[derive(H5Type, Clone, Debug, Copy)]
#[repr(C)]
pub struct CopyNumber {
    pub ProbeSetName: hdf5::types::FixedAscii<12>,
    Chromosome: u8,
    Position: u32,
    pub Log2Ratio: f32,
    pub WeightedLog2Ratio: f32,
    SmoothSignal: f32,
    NormalDiploid: u32,
}

impl CopyNumber {
    pub fn from_oschp_file(handle: &hdf5::file::File) -> Result<Array1<CopyNumber>, hdf5::Error> {
        return Ok(handle.group("ProbeSets")?.dataset("CopyNumber")?.read_1d::<CopyNumber>()?);
    }
}

#[derive(H5Type, Clone, Debug, Copy)]
#[repr(C)]
pub struct AllelicData {
    pub ProbeSetName: hdf5::types::FixedAscii<12>,
    Chromosome: u8,
    Position: u32,
    pub AllelicDifference: f32,
    pub BAF: f32,
}

impl AllelicData {
    pub fn from_oschp_file(handle: &hdf5::file::File) -> Result<Array1<AllelicData>, hdf5::Error> {
        return Ok(handle.group("ProbeSets")?.dataset("AllelicData")?.read_1d::<AllelicData>()?);
    }
}

#[derive(H5Type, Clone, Debug)]
#[repr(C)]
pub struct CopyNumberSegment {
    SegmentID: u32,
    Chromosome: u8,
    StartPosition: u32,
    StopPosition: u32,
    MarkerCount: i32,

    TotalCN: f32,
    BinnedCN: FixedAscii<4>,
    MedianLog2ratio: f32,
    MedianBAF: f32,
}

impl CopyNumberSegment {
    pub fn from_oschp_file(handle: &hdf5::file::File) -> Result<Array1<CopyNumberSegment>, hdf5::Error> {
        return Ok(handle.group("Segments")?.dataset("TuscanCN")?.read_1d::<CopyNumberSegment>()?);
    }
}

#[derive(H5Type, Clone, Debug)]
#[repr(C)]
pub struct LOHSegment {
    SegmentID: u32,
    Chromosome: u8,
    StartPosition: u32,
    StopPosition: u32,
    MarkerCount: i32,

    MeanMarkerDistance: u32,
    LOH: u32,
    Confidence: f32,
}

impl LOHSegment {
    pub fn from_oschp_file(handle: &hdf5::file::File) -> Result<Array1<LOHSegment>, hdf5::Error> {
        return Ok(handle.group("Segments")?.dataset("LOH")?.read_1d::<LOHSegment>()?);
    }
}

#[derive(H5Type, Clone, Debug)]
#[repr(C)]
pub struct ChromosomeSummary {
    pub Chromosome: u8,
    pub Display: FixedAscii<4>,
    StartIndex: u32,
    MarkerCount: u32,
    MinSignal: f32,
    MaxSignal: f32,
    MedianCnState: f32,
    HomFrequency: f32,
    HetFrequency: f32,
    Mosaicism: f32,
    LOH: f32,
    MedianSignal: f32,
}

impl ChromosomeSummary {
    pub fn from_oschp_file(handle: &hdf5::file::File) -> Result<Array1<ChromosomeSummary>, hdf5::Error> {
        return Ok(handle.group("Chromosomes")?.dataset("Summary")?.read_1d::<ChromosomeSummary>()?);
    }
}
