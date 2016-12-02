use DmxOutput;
use error::Error;


pub fn range_on(dmx: &mut DmxOutput, start: u32, end: u32) -> Result<(), Error> {
    let mut first_part = vec![0; start as usize - 1];
    let mut on_part = vec![255; (end - start + 1) as usize];
    let mut last_part = vec![0; 512 - end as usize];

    let mut data = first_part;
    data.append(&mut on_part);
    data.append(&mut last_part);

    dmx.send(&data)
}

pub fn range_off(dmx: &mut DmxOutput, start: u32, end: u32) -> Result<(), Error> {

    let mut first_part = vec![255; start as usize - 1];
    let mut on_part = vec![0; (end - start + 1) as usize];
    let mut last_part = vec![255; 512 - end as usize];

    let mut data = first_part;
    data.append(&mut on_part);
    data.append(&mut last_part);

    dmx.send(&data)
}
