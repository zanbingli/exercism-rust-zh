
const DNA_STR:&str = "ACGT";
const RNA_STR:&str = "CGAU";

#[derive(Debug, PartialEq)]
pub struct DNA{
    data:String,
}

#[derive(Debug, PartialEq)]
pub struct RNA{
    data:String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some((i,_v)) =
        dna.chars().enumerate().find(|(_i,v)|{
           !DNA_STR.contains(*v)
        }){
            return Err(i);
        }
        Ok(DNA{
            data:dna.to_string(),
        })
    }

    pub fn to_rna(self) -> RNA {
        RNA{
            data:self.data.chars().map(|x|{
                match x {
                    'G'=>'C',
                    'C'=>'G',
                    'T'=>'A',
                    'A'=>'U',
                    _ => x,
                }
            }).collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some((i,_v)) =
        rna.chars().enumerate().find(|(_i,v)|{
            !RNA_STR.contains(*v)
        }){
            return Err(i);
        }
        Ok(RNA{
            data:rna.to_string(),
        })
    }
}
