use std::fs::File;
use fitparser::profile::field_types::MesgNum;

fn semi_to_degrees(semi : i64) -> f64 {
   let factor : f64 = 2i64.pow(31u32) as f64;
   let deg_val : f64 = semi as f64 * 180f64 / factor;
   return deg_val;
}

fn main() {
    println!("Parsing FIT files using Profile version: {:?}", fitparser::profile::VERSION);
    let mut fp = File::open("tests/alsoworking.fit").expect("file not found");
    if let Ok(data) = fitparser::from_reader(&mut fp) {
        // print all the data in FIT file
        // println!("{:#?}", data);
        for item in &data {
            match item.kind() {
                MesgNum::Session => {
                    // print all the data records in FIT file
                    // println!("{:#?}", item.fields());

                    println!("################ Session ###############");
                    // Retrieve the FitDataField struct.
                    for fld in item.fields().iter() {
                            match fld.number() {

                                3_u8..=4_u8 | 29_u8..=39_u8 => {
                                     let semi : i64 = fld.value().try_into().expect("conversion failed");  //semicircles
                                     let degrees = semi_to_degrees(semi);
                                     println!("Name = {}, Value = {:.3}°, Units = {}",
                                              fld.name(), degrees, "degrees");
                                },

                                0_u8..=2_u8|5_u8..=28_u8|40_u8..=253_u8 =>{
                                       println!("Name = {}, Value = {:#}, Units = {}",
                                             fld.name(), fld.value(), fld.units());
                                 },

                                _ => print!("{}", "")  // matches other patterns 
                            }
                    }
                    println!("############ End Session ###############");
                },
                // Individual records
                MesgNum::Record => {
                    // print all the data records in FIT file
                    //println!("{:#?}", item.fields());

                    // Retrieve the FitDataField struct.
                    for fld in item.fields().iter() {
                            match fld.number() {

                                0|1 => {
                                     let semi : i64 = fld.value().try_into().expect("conversion failed");  //semicircles
                                     let degrees = semi_to_degrees(semi);
                                     println!("Name = {}, Value = {:.3}°, Units = {}",
                                              fld.name(), degrees, "degrees");
                                },

                                3|4|5|73|78|253 =>{
                                       println!("Name = {}, Value = {:#}, Units = {}",
                                             fld.name(), fld.value(), fld.units());
                                 },

                                _ => print!("{}", "")  // matches other patterns 
                            }
                    }
                    println!("");
                    },
                    
                // Lap records
                MesgNum::Lap => {
                    println!("################ Lap ###################");
                    // Retrieve the FitDataField struct.
                    for fld in item.fields().iter() {
                            match fld.number() {

                                3|4|5|6 => {
                                     let semi : i64 = fld.value().try_into().expect("conversion failed");  //semicircles
                                     let degrees = semi_to_degrees(semi);
                                     println!("Name = {}, Value = {:.3}°, Units = {}",
                                              fld.name(), degrees, "degrees");
                                },

                                7..25 =>{
                                       println!("Name = {}, Value = {:#}, Units = {}",
                                             fld.name(), fld.value(), fld.units());
                                 },

                                _ => print!("{}", "")  // matches other patterns 
                            }
                    }
                    println!("############ End Lap ###################");
                },
                
                _ => print!("{}", "")  // matches other patterns 
                }
        }
    }
}                                            

