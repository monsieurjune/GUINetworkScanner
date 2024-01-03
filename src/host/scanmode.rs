use std::vec::Vec;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct ScanMode
{
    lower: u16,
    upper: u16,
    portlist: Option<Vec<u16>>,
	partition_size: usize
}

fn create_hashmap() -> HashMap<String, (u16, u16, Option<Vec<u16>>)>
{
	return HashMap::from([
		("fullscan".to_string(), (0, 65535, None)),
		("quickscan".to_string(), (0, 1023, None)),
		("debug_vec_10_port".to_string(), (0, 0, Some(vec![1, 200, 3, 4, 5, 6, 7, 8, 9, 65]))),
		("debug_vec_2_port".to_string(), (0, 0, Some(vec![1, 1000]))),
		("debug_vec_16_port".to_string(), (0, 0, Some(vec![13, 312, 132, 323, 56, 7, 88, 999, 1000, 1100, 1200, 1313, 1441, 15, 160]))),
		("debug_vec_17_port".to_string(), (0, 0, Some(vec![13, 312, 132, 323, 56, 7, 88, 999, 1000, 1100, 1200, 1313, 1441, 15, 160, 17]))),
		("debug_limit_0..10,000_port".to_string(), (0, 10000, None)),
		("debug_limit_60000..65535_port".to_string(), (60000, 65535, None)),
		("debug_limit_50000..50015_port".to_string(), (50000, 50015, None)),
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

	pub fn subset_len(&self) -> usize
	{
		let vec_len: usize;
		let ceil: usize;

		match &self.portlist
		{
			Some(portlist) => {
				vec_len = portlist.len();
			}
			None => {
				vec_len = usize::from(self.upper - self.lower + 1);
			}
		}
		ceil = if vec_len % self.partition_size > 0 { 1 } else { 0 };
		return vec_len / self.partition_size + ceil;
	}

	pub fn get_subset(&self, n:usize) -> Vec<u16>
	{
		let set_n: usize = ScanMode::subset_len(self);

		assert!(n < set_n);
		return Vec::new();
	}
}

#[cfg(test)]
mod test
{
	use super::*;

	#[test]
	fn normal_test1() {
		let mode: ScanMode = ScanMode::new(&"debug1".to_string()).unwrap();
		assert_eq!(mode.get_subset(0), Vec::new());
	}

	#[test]
	#[should_panic]
	fn panic_test1() {
		let mode: ScanMode = ScanMode::new(&"debug2".to_string()).unwrap();
		assert_eq!(mode.get_subset(8), Vec::new());
	}
}