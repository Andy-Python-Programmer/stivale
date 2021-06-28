pub(crate) fn string_from_slice(slice: &[u8]) -> &str {
	let mut length = 0;

	while length < slice.len() && slice[length] != 0 {
		length += 1;
	}

	unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(slice.as_ptr(), length)) }
}
