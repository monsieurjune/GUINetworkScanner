use std::vec::Vec;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct HashNotFound;
pub type Map = HashMap<String, (u16, u16, Option<Vec<u16>>)>;

#[derive(Clone)]
pub struct ScanMode
{
	lower: u16,
	upper: u16,
	portlist: Option<Vec<u16>>,
	partition_size: u16
}

impl ScanMode
{
    pub fn new(mode: &str, map: &Map) -> Result<ScanMode, ()>
    {
		match map.get(mode)
		{
			Some(run) => {
				Ok(
					ScanMode {
						lower: run.0,
						upper: run.1,
						portlist: run.2.clone(),
						partition_size: 16
					}
				)
			}
			None => Err(())
		}
    }
	
	#[allow(dead_code)]
	pub fn get_limit(&self) -> (u16, u16)
	{
		return (self.lower, self.upper);
	}

	#[allow(dead_code)]
	pub fn get_portlist(&self) -> Option<Vec<u16>>
	{
		return self.portlist.clone();
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
		let a_max: usize = usize::from(a); 
		let mut b_max: usize = usize::from(if b > upper { upper } else { b }) + 1;
		let mut subset: Vec<u16> = Vec::new();

		b_max = if b_max > 65536 { 65536 } else { b_max };
		assert!(b_max - a_max <= usize::from(size));
		for port in a_max..b_max
		{
			subset.push(u16::try_from(port).unwrap());
		}
		return subset;
	}

	pub fn get_subset(&self, n:u16) -> Vec<u16>
	{
		let set_n: u16 = ScanMode::subset_len(self);
		let slice: &[u16];
		let a: u16;
		let tmp: usize;
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
				tmp = usize::from(a) + usize::from(self.partition_size) - 1;
				b = if tmp > 65535 { 65535 } else { u16::try_from(tmp).unwrap() };
				return ScanMode::limit_to_subset(a, b, self.upper, self.partition_size);
			}
		}
	}
}

#[cfg(test)]
mod test
{
	use super::*;
	use lazy_static::lazy_static;
	use std::collections::HashMap;

	lazy_static! (
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

	lazy_static! {
		static ref DEBUG_MAP: HashMap<String, (u16, u16, Option<Vec<u16>>)> = HashMap::from([
			(FULL.clone(), (0, 65535, None)),
			(QUICK.clone(), (0, 1023, None)),
			(PRIORITY.clone(), (0, 100, Some(vec![1]))),
			(VEC_10.clone(), (0, 0, Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]))),
			(VEC_2.clone(), (0, 0, Some(vec![1, 1000]))),
			(VEC_16.clone(), (0, 0, Some(vec![0, 1, 12, 123, 1234, 12345, 6, 67, 678, 6789, 1010, 1111, 1212, 1313, 1414, 1515]))),
			(VEC_17.clone(), (0, 0, Some(vec![0, 1, 12, 123, 1234, 12345, 6, 67, 678, 6789, 1010, 1111, 1212, 1313, 1414, 1515, 1616]))),
			(LIM_0_0.clone(), (0, 0, None)),
			(LIM_0_9999.clone(), (0, 9999, None)),
			(LIM_60000_65535.clone(), (60000, 65535, None)),
			(LIM_50000_50015.clone(), (50000, 50015, None)),
			(LIM_50000_50016.clone(), (50000, 50016, None)),
		]);
	}

	fn cmp_len_mode(mode: &String, b: u16) {
		let mode = ScanMode::new(mode, &DEBUG_MAP);
		let a = mode.unwrap().subset_len();
		assert_eq!(a, b);
	}

	fn cmp_vec_mode(mode: &String, i: u16, ex: Vec<u16>) {
		let modes = ScanMode::new(mode, &DEBUG_MAP).unwrap();
		let sub = modes.get_subset(i);
		assert_eq!(sub, ex);
	}

	fn range_to_vec(a: usize, b:usize) -> Vec<u16>
	{
		let mut res: Vec<u16> = Vec::new();

		for i in a..(b+1)
		{
			if i >= 65536 { break; }
			res.push(u16::try_from(i).unwrap())
		}
		return res;
	}

	#[test]
	fn normal_access() {
		let mode = ScanMode::new(&QUICK, &DEBUG_MAP);
		assert!(mode.is_ok());
	}

	#[test]
	fn no_key_access() {
		let mode = ScanMode::new(&"sddfd".to_string(), &DEBUG_MAP);
		assert!(mode.is_err());
	}

	#[test]
	fn priority_check() {
		let mode = ScanMode::new(&PRIORITY, &DEBUG_MAP);
		let res = mode.unwrap().subset_len();
		assert_eq!(res, 1);
	}

	#[test]
	fn subset_len_full() {
		cmp_len_mode(&FULL, 4096);
	}

	#[test]
	fn subset_len_quick() {
		cmp_len_mode(&QUICK, 64);
	}

	#[test]
	fn subset_len_vec_10() {
		cmp_len_mode(&VEC_10, 1);
	}

	#[test]
	fn subset_len_vec_2() {
		cmp_len_mode(&VEC_2, 1);
	}

	#[test]
	fn subset_len_vec_16() {
		cmp_len_mode(&VEC_16, 1);
	}

	#[test]
	fn subset_len_vec_17() {
		cmp_len_mode(&VEC_17, 2);
	}

	#[test]
	fn subset_len_lim_0_0() {
		cmp_len_mode(&LIM_0_0, 1);
	}

	#[test]
	fn subset_len_lim_0_9999() {
		cmp_len_mode(&LIM_0_9999, 625);
	}

	#[test]
	fn subset_len_lim_60000_65535() {
		cmp_len_mode(&LIM_60000_65535, 346);
	}

	#[test]
	fn subset_len_lim_50000_50015() {
		cmp_len_mode(&LIM_50000_50015, 1);
	}

	#[test]
	fn subset_len_lim_50000_50016() {
		cmp_len_mode(&LIM_50000_50016, 2);
	}

	#[test]
	#[should_panic]
	fn get_subset_vec_10_error() {
		cmp_vec_mode(&VEC_10, 312, vec![]);
	}

	#[test]
	#[should_panic]
	fn get_subset_fullscan_error() {
		cmp_vec_mode(&FULL, 65535, vec![]);
	}

	#[test]
	fn get_subset_vec17_0() {
		cmp_vec_mode(&VEC_17, 0, vec![0, 1, 12, 123, 1234, 12345, 6, 67, 678, 6789, 1010, 1111, 1212, 1313, 1414, 1515]);
	}

	#[test]
	fn get_subset_vec17_1() {
		cmp_vec_mode(&VEC_17, 1, vec![1616]);
	}

	#[test]
	fn get_subset_vec10_0() {
		cmp_vec_mode(&VEC_10, 0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
	}

	#[test]
	fn get_subset_lim_0_0() {
		cmp_vec_mode(&LIM_0_0, 0, vec![0]);
	}

	#[test]
	fn get_subset_lim_0_9999() {
		let range = range_to_vec(9984, 9999);
		cmp_vec_mode(&LIM_0_9999, 624, range);
	}

	#[test]
	fn get_subset_lim_50000_50016_0() {
		let range = range_to_vec(50000, 50015);
		cmp_vec_mode(&LIM_50000_50016, 0, range);
	}

	#[test]
	fn get_subset_lim_50000_50016_1() {
		cmp_vec_mode(&LIM_50000_50016, 1, vec![50016]);
	}

	#[test]
	fn get_subset_fullscan_head() {
		let range = range_to_vec(16, 31);
		cmp_vec_mode(&FULL, 1, range);
	}

	#[test]
	fn get_subset_fullscan_middle() {
		let mid:u16 = 1653;
		let range = range_to_vec(usize::from(mid) * 16, usize::from(mid) * 16 + 15);
		cmp_vec_mode(&FULL, mid, range)
	}

	#[test]
	fn get_subset_fullscan_end() {
		let range = range_to_vec(65520, 65535);
		cmp_vec_mode(&FULL, 4095, range);
	}

	#[test]
	fn get_subset_lim_60000_65535() {
		let range = range_to_vec(65520, 65535);
		cmp_vec_mode(&LIM_60000_65535, 345, range)
	}
}