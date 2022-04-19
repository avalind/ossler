use std::collections::HashMap;
use ndarray::Array1;

/*
* Build a lookup table so that we can retrieve the correct chromosome names from the byte keys.
*/
pub fn build_chrom_table(chrom_list: &Array1<crate::record::oschp::ChromosomeSummary>) -> HashMap<u8, String> {
    let mut m = HashMap::new();
    for elem in chrom_list {
        m.insert(elem.Chromosome, elem.Display.as_str().to_owned());
    }
    return m;
}
