use std::fs::File;
use bigtools::BigBedRead;

pub enum Strand
{
    FivePrime,
    ThreePrime,
    Unknown,
}

pub struct Entry
{
    start:u32,
    end:u32,
    other:Vec<String>,
}

impl Entry
{
    pub fn new(start:u32, end:u32, other:Vec<String>) -> Entry
    {
        Entry 
        {
            start,
            end,
            other
        }
    }
    
    pub fn length(&self) -> u32
    {
        return self.end - self.start
    }
    
    pub fn other(&self, index:usize) -> Option<String>
    {
        if index < self.other.len()
        {
            return Some(self.other[index].clone());
        }
        None
    }
}

pub struct Interval
{
    chrom:String,
    start:u32,
    end:u32,
    entries:Vec<Entry>,
    delimiter:char
}

impl Interval
{
    pub fn new(chrom:String, start:u32, end:u32, delimiter:char) -> Interval
    {
        Interval {
            chrom,
            start,
            end,
            entries:Vec::new(),
            delimiter
        }
    }
    
    pub fn chrom(&self) -> String
    {
        self.chrom.clone()
    }
    
    pub fn delimiter(&self) -> char
    {
        self.delimiter
    }
    
    pub fn length(&self) -> u32
    {
        self.end - self.start
    }
    
    pub fn add_entry(&mut self, entry:Entry)
    {
        self.entries.push(entry);
    }
}

//--------------------------------------------------------------------------------------

pub fn test_load()
{
    let bed_stream = File::open("/home/jon/Data/homo_sapiens.GRCh38.Regulatory_Build.regulatory_features.bb").unwrap();
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
    
    let intervals = reader.get_interval("1", 9949600, 10000000).unwrap();
    
    for int_result in intervals
    {
        if let Ok(int) = int_result
        {
            
            println!("{}, {}, [{}]", int.start, int.end, int.rest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_load();
    }
}
