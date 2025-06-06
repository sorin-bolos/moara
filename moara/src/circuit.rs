extern crate serde;
extern crate serde_with;

use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use serde_with::PickFirst;

#[derive(Deserialize)]
pub struct Circuit
{
    #[serde(default)]
    pub steps:Vec<Step>
}

#[derive(Deserialize)]
pub struct Step
{
    #[serde(default)]
    pub index:u16,
    
    #[serde(default)]
    pub gates:Vec<Gate>
}

#[serde_as]
#[derive(Deserialize)]
pub struct Gate
{
    pub name:String,

    #[serde(default)]
    pub targets:Vec<u8>,

    #[serde(default)]
    pub controls:Vec<Control>,

    #[serde(default)]
    pub gates:Vec<AggregatedGate>,

    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    #[serde(default)]
    pub bit:Option<u8>,

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

#[serde_as]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Control 
{
    pub target:u8,
    pub state:String,
}

#[serde_as]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct AggregatedGate 
{
    pub name:String,
    
    #[serde(default)]
    pub targets:Vec<u8>,

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
        let mut min_index = u8::MAX;

        for i in 0..self.targets.len() {
            if self.targets[i] < min_index {
                min_index = self.targets[i];
            }
        }

        for control in &self.controls {
            if control.target < min_index {
                min_index = control.target;
            }
        }

        for gate in &self.gates {
            for i in 0..gate.targets.len() {
                if gate.targets[i] < min_index {
                    min_index = gate.targets[i];
                }
            }
        }

        min_index
    }

    pub fn get_max_qubit_index(&self) -> u8 {
        let mut max_index = u8::MIN;

        for i in 0..self.targets.len() {
            if self.targets[i] > max_index {
                max_index = self.targets[i];
            }
        }

        for control in &self.controls {
            if control.target > max_index {
                max_index = control.target;
            }
        }

        for gate in &self.gates {
          for i in 0..gate.targets.len() {
              if gate.targets[i] > max_index {
                  max_index = gate.targets[i];
              }
          }
        }

        max_index
    }
}