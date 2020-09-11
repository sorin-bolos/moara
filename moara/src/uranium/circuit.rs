extern crate serde;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Circuit
{
    pub steps:Vec<Step>
}

#[derive(Deserialize)]
pub struct Step
{
    pub index:u16,
    pub gates:Vec<Gate>
}

#[derive(Deserialize)]
pub struct Gate
{
    pub name:String,
    pub target:u8,
    pub target2:Option<u8>,
    pub control:Option<u8>
}