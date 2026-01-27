pub mod models;

use std::fs::File;
use bigtools::BigBedRead;

//TODO 
//Create interface
//implement interface for bigbed
//figure out how to access additional values


//--------------------------------------------------------------------------------------

pub fn test_load()
{
    let bed_stream = File::open("/home/jon/Data/homo_sapiens.GRCh38.Regulatory_Build.regulatory_features.bb").unwrap();
    let mut reader = BigBedRead::open(bed_stream).unwrap();
    //let mut full_reader = Reader
    
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
