use std::vec::Vec;
mod portlist;

const PORTLIST: [fn() -> (Option<Vec<u16>>, u16, u16); 3] = 
[
    portlist::quickscan,
    portlist::fullscan
];

pub struct ScanMode
{
    lowerRange: u16,
    upperRange: u16,
    portList: Option<Vec<u16>>
}

impl ScanMode
{
    pub fn new(mode: u32) -> ScanMode
    {
        let (list, up, low) = PORTLIST[mode]();

        return ScanMode
        {
            lowerRange: low,
            upperRange: up,
            portList: list
        };
    }
}