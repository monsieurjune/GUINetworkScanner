use std::vec::Vec;
mod fullscan;
mod quickscan;
mod debug;

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
        static PORTLIST: [fn() -> (u16, u16, Option<Vec<u16>>); 3] = [
            fullscan::fullscan,
            quickscan::quickscan,
            debug::debug1
        ];

        let run: (u16, u16, Option<Vec<u16>>) = PORTLIST[mode]();

        return ScanMode {
            lower: run.0,
            upper: run.1,
            portlist: run.2
        }
    }

    pub fn get_portlist(&self) -> Option<Vec<u16>>
    {
        return self.portlist.clone();
    }

    pub fn get_limit(&self) -> (u16, u16)
    {
        return (self.lower, self.upper);
    }
}