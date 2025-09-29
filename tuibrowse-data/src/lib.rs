use std::fs::File;
use bigtools::BigBedRead;

pub enum Strand
{
    FivePrime,
    ThreePrime,
    Unknown,
}

pub struct Feature
{
    name:String,
    start:u32,
    end:u32,
    strand:Strand
}

pub struct Chrom
{
    name:String,
    start:u32,
    end:u32,
    features:Vec<Feature>,
}

pub fn test_load()
{
    let bed_stream = File::open("/home/jon/Programming/github/tui-browse/examples/bigBedExample.bb").unwrap();
    let mut reader = BigBedRead::open(bed_stream).unwrap();
    
    let chroms = reader.chroms();
    
    for chrom in chroms
    {
        println!("{},{}",chrom.name.to_string(), chrom.length);
    }
    
    println!("SQL-----");
    let has_sql = reader.autosql().unwrap();
    if let Some(sql) = has_sql
    {
        println!("{}",sql);
    }
    else
    {
        println!("SQL missing!");
    }
    
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_load();
    }
}
