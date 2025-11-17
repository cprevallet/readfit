use std::fs::File;
use fitparser::profile::field_types::MesgNum;
use fitparser::FitDataField;

fn main() {
    println!("Parsing FIT files using Profile version: {:?}", fitparser::profile::VERSION);
    let mut fp = File::open("tests/alsoworking.fit").expect("file not found");
    if let Ok(data) = fitparser::from_reader(&mut fp) {
        // print the data in FIT file
        //println!("{:#?}", data);
        for item in &data {
            match item.kind() {
                MesgNum::Record => {
                    //println!("{:#?}", item.fields());
                     let pos_lat: &FitDataField = item.fields().get(0).unwrap();
                     println!("Name = {}, Number = {}, Value = {}, Units = {}",
                         pos_lat.name(), pos_lat.number(), pos_lat.value(), pos_lat.units());

                     let pos_lon: &FitDataField = item.fields().get(1).unwrap();
                     println!("Name = {}, Number = {}, Value = {}, Units = {}",
                         pos_lon.name(), pos_lon.number(), pos_lon.value(), pos_lon.units());
                },
                _ => print!("{}", "") 
                }
        }
    }
}    
