use std::vec::Vec;
use std::collections::HashMap;
use std::convert::TryFrom;
use lazy_static::lazy_static;

pub struct ScanMode
{
    lower: u16,
    upper: u16,
    portlist: Option<Vec<u16>>,
	partition_size: u16
}

fn create_hashmap() -> HashMap<String, (u16, u16, Option<Vec<u16>>)>
{
	return HashMap::from([
		("full".to_string(), (0, 65535, None)),
		("quick".to_string(), (0, 1023, None)),
		("debug_priority_check".to_string(), (0, 100, Some(vec![1]))),
		("debug_vec_10_port".to_string(), (0, 0, Some(vec![1, 200, 3, 4, 5, 6, 7, 8, 9, 65]))),
		("debug_vec_2_port".to_string(), (0, 0, Some(vec![1, 1000]))),
		("debug_vec_16_port".to_string(), (0, 0, Some(vec![13, 312, 132, 323, 56, 7, 88, 999, 1000, 1100, 1200, 1313, 1441, 15, 160, 0]))),
		("debug_vec_17_port".to_string(), (0, 0, Some(vec![13, 312, 132, 323, 56, 7, 88, 999, 1000, 1100, 1200, 1313, 1441, 15, 160, 17, 0]))),
		("debug_limit_0..0_port".to_string(), (0, 0, None)),
		("debug_limit_0..9999_port".to_string(), (0, 9999, None)),
		("debug_limit_60000..65535_port".to_string(), (60000, 65535, None)),
		("debug_limit_50000..50015_port".to_string(), (50000, 50015, None)),
		("debug_limit_50000..50016_port".to_string(), (50000, 50016, None))
	]);
}

impl ScanMode
{
    pub fn new(mode: &String) -> Result<ScanMode, ()>
    {
		lazy_static! {
			static ref PORTMAP: HashMap<String, (u16, u16, Option<Vec<u16>>)> = create_hashmap();
		}

		match PORTMAP.get(mode)
		{
			Some(run) => {
				return Ok(
					ScanMode {
						lower: run.0,
						upper: run.1,
						portlist: run.2.clone(),
						partition_size: 16
					}
				)
			}
			None => {
				return Err(());
			}
		}
    }

	pub fn subset_len(&self) -> u16
	{
		let vec_len: usize;
		let ceil: usize;
		let len: usize;

		match &self.portlist
		{
			Some(portlist) => {
				vec_len = portlist.len();
			}
			None => {
				vec_len = usize::from(self.upper) - usize::from(self.lower) + 1;
			}
		}
		ceil = if vec_len % usize::from(self.partition_size) > 0 { 1 } else { 0 };
		len = vec_len / usize::from(self.partition_size) + ceil;
		return u16::try_from(len % 65536).unwrap();
	}

	fn portlist_slicer(list: &[u16], size: u16) -> Vec<u16>
	{
		let mut subset: Vec<u16> = Vec::new();

		for port in list
		{
			subset.push(port.clone());
			if subset.len() >= usize::from(size) { break; }
		}
		return subset;
	}

	fn limit_to_subset(a: u16, b: u16, upper: u16, size: u16) -> Vec<u16>
	{
		let b_max: u16 = if b > upper { upper } else { b } + 1;
		let mut subset: Vec<u16> = Vec::new();

		for port in a..b_max
		{
			subset.push(port);
		}
		return subset;
	}

	pub fn get_subset(&self, n:u16) -> Vec<u16>
	{
		let set_n: u16 = ScanMode::subset_len(self);
		let slice: &[u16];
		let a: u16;
		let b: u16;

		assert!(n < set_n);
		match &self.portlist
		{
			Some(list) => {
				slice = &list[usize::from(n * self.partition_size)..];
				return ScanMode::portlist_slicer(slice, self.partition_size);
			}
			None => {
				a = self.lower + u16::from(n * self.partition_size);
				b = a + self.partition_size - 1;
				return ScanMode::limit_to_subset(a, b, self.upper, self.partition_size);
			}
		}
	}
}

#[cfg(test)]
mod test
{
	use super::*;

	lazy_static!(
		static ref FULL: String = "full".to_string();
		static ref QUICK: String = "quick".to_string();
		static ref PRIORITY: String = "debug_priority_check".to_string();
		static ref VEC_10: String = "debug_vec_10_port".to_string();
		static ref VEC_2: String = "debug_vec_2_port".to_string();
		static ref VEC_16: String = "debug_vec_16_port".to_string();
		static ref VEC_17: String = "debug_vec_17_port".to_string();
		static ref LIM_0_0: String = "debug_limit_0..0_port".to_string();
		static ref LIM_0_9999: String = "debug_limit_0..9999_port".to_string();
		static ref LIM_60000_65535: String = "debug_limit_60000..65535_port".to_string();
		static ref LIM_50000_50015: String = "debug_limit_50000..50015_port".to_string();
		static ref LIM_50000_50016: String = "debug_limit_50000..50016_port".to_string();
	);

	#[test]
	fn normal_access() {
		let mode = ScanMode::new(&QUICK);
		assert!(mode.is_ok());
	}

	#[test]
	fn no_key_access() {
		let mode = ScanMode::new(&"sddfd".to_string());
		assert!(mode.is_err());
	}

	#[test]
	fn priority_check() {
		let mode = ScanMode::new(&PRIORITY);
		let res = mode.unwrap().subset_len();
		assert_eq!(res, 1);
	}

	fn cmp_mode(mode: &String, b: u16) {
		let mode = ScanMode::new(mode);
		let a = mode.unwrap().subset_len();
		assert_eq!(a, b);
	}

	#[test]
	fn subset_len_full() {
		cmp_mode(&FULL, 4096);
	}

	#[test]
	fn subset_len_quick() {
		cmp_mode(&QUICK, 64);
	}

	#[test]
	fn subset_len_vec_10() {
		cmp_mode(&VEC_10, 1);
	}

	#[test]
	fn subset_len_vec_2() {
		cmp_mode(&VEC_2, 1);
	}

	#[test]
	fn subset_len_vec_16() {
		cmp_mode(&VEC_16, 1);
	}

	#[test]
	fn subset_len_vec_17() {
		cmp_mode(&VEC_17, 2);
	}

	#[test]
	fn subset_len_lim_0_0() {
		cmp_mode(&LIM_0_0, 1);
	}

	#[test]
	fn subset_len_lim_0_9999() {
		cmp_mode(&LIM_0_9999, 625);
	}

	#[test]
	fn subset_len_lim_60000_65535() {
		cmp_mode(&LIM_60000_65535, 346);
	}

	#[test]
	fn subset_len_lim_50000_50015() {
		cmp_mode(&LIM_50000_50015, 1);
	}

	#[test]
	fn subset_len_lim_50000_50016() {
		cmp_mode(&LIM_50000_50016, 2);
	}


}