pub struct SegmentationResult {
    pub mask: Vec<u8>,
}

pub struct SegmentationInput(pub &[u8]);
