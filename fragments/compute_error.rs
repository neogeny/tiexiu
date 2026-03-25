impl<'a> Ctx<'a> {
    pub fn report_error(&self) -> String {
        let offset = self.max_offset;
        let expected = self.compute_ll1_at(offset); // The TatSu magic
        format!("Expected one of {:?} at offset {}", expected, offset)
    }
}
