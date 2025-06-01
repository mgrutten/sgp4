fn main() -> anyhow::Result<()> {
    let elements = sgp4::Elements::from_tle(
        Some("QZS-3 (MICHIBIKI-3)".to_owned()),
        "1 42917U 17048A   25151.79280885 -.00000353  00000+0  00000+0 0  9994".as_bytes(),
        "2 42917   0.0659 140.0046 0002525 290.7869 231.1818  1.00271916 28433".as_bytes(),
    )?;
    let constants = sgp4::Constants::from_elements(&elements)?;
    let (lon, lonrate) = constants.initial_lon_lonrate()
        .ok_or(anyhow::anyhow!("No longitude"))?;
    println!("{:?} {:?}", lon, lonrate);
    
    Ok(())
}
