//Prepare VSlamSettings .json file for the vslam-sandbox example
//Install Rust-cv
//In the root folder:
//cargo new vs1
//set the Cargo.toml dependencies:
//[dependencies]
//serde = { version = "1.0.130", features = ["derive"] }
//serde_json = "1.0.69"
//cv-sfm = { version = "0.1.0", path = "../cv-sfm", features = [
//    "serde-serialize",
//] }
//This is: vs1/src/main.rs:
//#23.11.2021

use cv_sfm::{VSlam, VSlamSettings};

use serde::{Deserialize, Serialize};
use serde_json::Result;

fn main() {
 let mut vs: cv_sfm::VSlamSettings; 
 vs = VSlamSettings::default();
 let s = serde_json::to_string(&vs); 
       //Print, write to a file, or send to an HTTP server.
 println!("{:?}", s);
      //Save to file, add to the structop of the vslam-playground etc. 
}
