pub struct Circuit
{
    steps:Vec<Step>
}

impl Circuit
{
    pub fn new(steps:Vec<Steps>) -> Self {
        Self {
            data:data
        }
    }
}

pub struct Step
{
    index:u16,
    gates:Vec<Gate>
}

impl Step
{
    pub fn new(index:u16, gates:Vec<Gate>) -> Self {
        Self {
            index:index,
            gates:gates
        }
    }
}

pub struct Gate
{
    name:String,
    target:u8
}

impl Gate
{
    pub fn new(name:String, target:u8) -> Self {
        Self {
            name:name,
            target:target
        }
    }
}