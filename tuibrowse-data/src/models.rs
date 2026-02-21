pub enum Strand
{
    FivePrime,
    ThreePrime,
    Unknown,
}

pub enum BedElements
{
    Chrom,
    ChromStart,
    ChromEnd,
    Name,
    Score,
    Strand,
    ThickStart,
    ThickEnd,
    ItemRGB,
    BlockCount,
    BlockSizes,
    BlockStarts
}

pub enum BedType
{
    String(String),
    Uint(u32),
    Int(f32),
    Char(char),
    IntArray(Vec<f32>)
}

pub struct Block
{
    start:u32,
    end:u32
}

pub struct Entry
{
    start:u32,
    end:u32,
    name:String,
    rgb:[u32;3],
    strand:Strand,
    blocks:Vec<Block>,
    score:u32,
}

impl Entry
{
    pub fn new(start:u32, end:u32, other:Vec<String>) -> Result<Entry,String>
    {
        let mut other_values= other.iter();
        //parse other
        let name = match other_values.next()
        {
            Some(name) => name.clone(),
            None => return Err("Missing name".to_string())  
        };
        
        let score:u32 = match other_values.next()
        {
            Some(val) => {
                match val.parse()
                {
                    Ok(v) => v,
                    Err(_) => return Err("Score no a u32".to_string())
                }
            },
            None => return Err("Missing score".to_string())
        };
        
        let strand = match other_values.next()
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