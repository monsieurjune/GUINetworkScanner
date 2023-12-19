use std::vec::Vec;

pub fn quickscan() -> (Option<Vec<u16>>, u16, u16)
{
    return (None, 0, 1023);
}

pub fn fullscan() -> (Option<Vec<u16>>, u16, u16)
{
    return (None, 0, 65535);
}