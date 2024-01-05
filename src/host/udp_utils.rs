use std::vec::Vec;
use std::collections::HashMap;

pub fn create_map() -> HashMap<String, (u16, u16, Option<Vec<u16>>)>
{
	HashMap::from([
		("full".to_string(), (0, 65535, None)),
		("full".to_string(), (0, 65535, None)),
	])
}

