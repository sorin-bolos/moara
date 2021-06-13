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
    pub control:Option<u8>,
    pub phi:Option<f32>,
    pub theta:Option<f32>,
    pub lambda:Option<f32>,
    pub root:Option<String>,
}

impl Gate {
    pub fn get_min_qubit_index(&self) -> u8 {
        let mut min_index = self.target;

        match self.target2 {
            Some(index) => {
                if index < min_index {
                    min_index = index;
                }
            },
            None => {}
        }

        match self.control {
            Some(index) => {
                if index < min_index {
                    min_index = index;
                }
            },
            None => {}
        }

        min_index
    }

    pub fn get_max_qubit_index(&self) -> u8 {
        let mut max_index = self.target;

        match self.target2 {
            Some(index) => {
                if index > max_index {
                    max_index = index;
                }
            },
            None => {}
        }

        match self.control {
            Some(index) => {
                if index > max_index {
                    max_index = index;
                }
            },
            None => {}
        }

        max_index
    }
}