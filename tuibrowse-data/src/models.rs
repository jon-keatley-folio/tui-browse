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