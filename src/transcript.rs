

use genomicregion::GenomicRegion;


pub struct Transcript {
    accession: String
    is_forward_strand: bool,
    coding_regions: Vec<GenomicRegion>
}

impl Transcript {
    /// Create a new transcript from a set of genomic regions that represent 
    /// the coding regions. The genomic regions must not overlap.
    pub fn new_from_cds(accession: String, cdss: Vec<GenomicRegion>) -> Self {
        Transcript { accession: accession, is_forward_strand: true, coding_regions: cdss };
    }
}


