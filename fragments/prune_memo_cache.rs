// Assuming your memo cache uses the input offset as a key
// and 'cut_offset' is the position of your PEG cut.
self.memo_cache.retain(|&offset, _| offset >= cut_offset);
