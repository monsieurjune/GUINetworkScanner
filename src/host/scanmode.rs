use std::vec::Vec;
mod fullscan;
mod quickscan;

pub struct ScanMode
{
    lower: u16,
    upper: u16,
    portlist: Option<Vec<u16>>
}

impl ScanMode
{
    pub fn new(mode: usize) -> ScanMode
    {
        static PORTLIST: [fn() -> (u16, u16, Option<Vec<u16>>); 2] = 
        [
            fullscan::fullscan,
            quickscan::quickscan
        ];

        let run: (u16, u16, Option<Vec<u16>>) = PORTLIST[mode]();

        return ScanMode
        {
            lower: run.0,
            upper: run.1,
            portlist: run.2
        }
    }
}