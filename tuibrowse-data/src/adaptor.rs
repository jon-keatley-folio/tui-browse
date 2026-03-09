use std::fs::File;
use bigtools::BigBedRead;

use crate::models::{Interval, Entry, Region};

pub trait BrowserFile: Sized {
    fn get_regions(&self) -> Vec<Region>;
    fn open(path:&str) -> Result<Self,String>;
    fn get_interval(&mut self,region:&str, start:u32, end:u32) -> Result<Interval, String>;
}

pub struct BigBed
{
    reader:BigBedRead<File>
}

impl BrowserFile for BigBed
{
    fn open(path:&str) -> Result<BigBed,String>
    {
        let bed_stream = match File::open(path)
        {
            Ok(f) => f,
            Err(_) => return Err("Failed to open file".to_string())
        };
        
        let reader = match BigBedRead::open(bed_stream)
        {
            Ok(r) => r,
            Err(_) => return Err("Unable to read bigbed".to_string())
        };
        
        Ok(BigBed
        {
            reader
        })
    }
    
    fn get_regions(&self) -> Vec<Region>
    {
        let results:Vec<Region> = self.reader.chroms()
        .iter()
        .map(|ci| Region::new(&ci.name, ci.length))
        .collect();  
        results
    }
    
    fn get_interval(&mut self,region:&str, start:u32, end:u32) -> Result<Interval, String>
    {
        let mut interval =Interval::new(
            region.to_string(),
            start,
            end,
            '\n'
        );
        
        let has_entires = self.reader.get_interval(region, start, end);
        if let Ok(entires) = has_entires
        {
            for has_entry in entires
            {
                if let Ok(entry) = has_entry
                {
                    let extra_cols: Vec<String> = entry.rest.split(interval.delimiter())
                    .map(|v| v.to_string())
                    .collect();
                    
                    let entry_created = Entry::new(
                            entry.start,
                            entry.end,
                            extra_cols
                    );
                    
                    if let Ok(new_entry) = entry_created
                    {
                        interval.add_entry(
                            new_entry
                        );
                    }
                }    
            }
            
            Ok(interval)
        }
        else
        {
            Err("Failed to get interval".to_string())
        }
        
    }
}

//TODO add tests
