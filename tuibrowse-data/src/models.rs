pub enum Strand
{
    FivePrime,
    ThreePrime,
    Unknown,
}


pub struct Block
{
    start:u32,
    end:u32
}

pub struct Region
{
    name:String,
    length:u32
}

impl Region {
    pub fn new(name:&str, length:u32) -> Region
    {
        Region
        {
            name:name.to_string(),
            length
        }
    }
}

//Core model is a direct copy of bigbed - https://en.wikipedia.org/wiki/BED_(file_format)
pub struct Entry
{
    start:u32,
    end:u32,
    name:String,
    rgb:[u32;3],
    strand:Strand,
    blocks:Vec<Block>,
    score:i16,
}

impl Entry
{
    pub fn new(start:u32, end:u32, other:Vec<String>) -> Result<Entry,String>
    {
        let mut other_values= other.iter();
        //parse other
        let name = match other_values.next() //column 4
        {
            Some(name) => name.clone(),
            None => return Err("Missing name".to_string())  
        };
        
        let score:i16 = match other_values.next() //column 5
        {
            Some(val) => {
                match val.parse()
                {
                    Ok(v) => v,
                    Err(_) => return Err("Score no a u32".to_string())
                }
            },
            None => -1
        };
        
        let strand = match other_values.next() //column 6
        {
            Some(v) => {
                match v.as_str()
                {
                    "+" => Strand::FivePrime,
                    "-" => Strand::ThreePrime,
                    _ => Strand::Unknown
                }
            },
            None => return Err("Missing strand".to_string())
        };
        
        //thick start - column 7
        //thick end - column 8
        //RGB (should be byte,byte,byte) - column 9
        //block cound - column 10
        //block sizes (u32,u32,u32...) - column 11
        //block starts (u32,u32,u32...) - column 12
        
        Ok(Entry 
        {
            start,
            end,
            name,
            rgb:[0,0,0],
            strand,
            blocks:Vec::new(),
            score
        })
    }
    
    pub fn length(&self) -> u32
    {
        return self.end - self.start
    }
    
    pub fn start(&self) -> u32
    {
        self.start
    }
    
    pub fn end(&self) -> u32
    {
        self.end
    }
}

pub struct Interval
{
    region:String,
    start:u32,
    end:u32,
    entries:Vec<Entry>,
    delimiter:char
}

impl Interval
{
    pub fn new(region:String, start:u32, end:u32, delimiter:char) -> Interval
    {
        Interval {
            region,
            start,
            end,
            entries:Vec::new(),
            delimiter
        }
    }
    
    pub fn region(&self) -> String
    {
        self.region.clone()
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