use std::fs::File;
use fitparser::profile::field_types::MesgNum;

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
                    let x = item.fields().get(0);
                    println!("{:#?}", x);
                    let y = item.fields().get(1);
                    println!("{:#?}", y);
                    let z = item.fields().get(2);
                    println!("{:#?}", z);
                },
                _ => print!("{}", "") 
                }
        }
    }
}    
