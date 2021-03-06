extern crate serde;
extern crate serde_with;

use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use serde_with::PickFirst;
//use serde_with::Option;

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

#[serde_as]
#[derive(Deserialize)]
pub struct Gate
{
    pub name:String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub target:u8,

    #[serde_as(as = "PickFirst<(_, Option<DisplayFromStr>)>")]
    #[serde(default)]
    pub target2:Option<u8>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub control:Option<u8>,
    
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub controlstate:Option<u8>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub control2:Option<u8>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub controlstate2:Option<u8>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub phi:Option<f32>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub theta:Option<f32>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub lambda:Option<f32>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
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