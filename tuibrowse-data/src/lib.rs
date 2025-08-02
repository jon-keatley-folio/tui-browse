
pub enum Strand
{
    FivePrime,
    ThreePrime,
    Unknown,
}

pub struct Feature
{
    name:String,
    start:uint,
    end:uint,
    strand:Strand
}

pub struct Chrom
{
    name:String,
    start:uint,
    end:uint,
    features:Vec<Feature>,
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
