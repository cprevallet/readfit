use std::fs::File;
use fitparser::profile::field_types::MesgNum;
use fitparser::FitDataField;

fn semi_to_degrees(semi : i64) -> f64 {
   let factor : f64 = 2i64.pow(31u32) as f64;
   let deg_val : f64 = semi as f64 * 180f64 / factor;
   return deg_val;
}

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
                     let semi : i64 = pos_lat.value().try_into().expect("conversion failed");  //semicircles
                     let degrees_lat = semi_to_degrees(semi);
                     println!("Name = {}, Number = {}, Value = {}, Units = {}",
                         pos_lat.name(), pos_lat.number(), degrees_lat, pos_lat.units());

                     let pos_lon: &FitDataField = item.fields().get(1).unwrap();
                     let semi : i64 = pos_lon.value().try_into().expect("conversion failed");  //semicircles
                     let degrees_lon = semi_to_degrees(semi);
                     println!("Name = {}, Number = {}, Value = {}, Units = {}",
                         pos_lon.name(), pos_lon.number(), degrees_lon, pos_lon.units());
                },
                _ => print!("{}", "") 
                }
        }
    }
}    
