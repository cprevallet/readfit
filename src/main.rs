use std::fs::File;
use fitparser::profile::field_types::MesgNum;
use fitparser::FitDataField;

/*
Basic stuff silicon sneaker brings in:

		RecTimestamps[idx] = C.long(item.Timestamp.Unix())
		RecDistances[idx] = C.float(item.GetDistanceScaled())
		RecSpeeds[idx] = C.float(item.GetSpeedScaled())
		RecAltitudes[idx] = C.float(item.GetAltitudeScaled())
		RecCadences[idx] = C.float(item.Cadence)
		RecHeartRates[idx] = C.float(item.HeartRate)
		RecLats[idx] = C.float(item.PositionLat.Degrees())
		RecLongs[idx] = C.float(item.PositionLong.Degrees())

Other stuff silicon sneaker brings in:
		LapTimestamps[idx] = C.long(item.Timestamp.Unix())
		LapTotalDistances[idx] = C.float(item.GetTotalDistanceScaled())
		LapStartPositionLats[idx] = C.float(item.StartPositionLat.Degrees())
		LapStartPositionLongs[idx] = C.float(item.StartPositionLong.Degrees())
		LapEndPositionLats[idx] = C.float(item.EndPositionLat.Degrees())
		LapEndPositionLongs[idx] = C.float(item.EndPositionLong.Degrees())
		LapTotalCaloriess[idx] = C.float(item.TotalCalories)
		LapTotalElapsedTimes[idx] = C.float(item.GetTotalElapsedTimeScaled())
		LapTotalTimerTimes[idx] = C.float(item.GetTotalTimerTimeScaled())
		nLaps = nLaps + 1
*/

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
                MesgNum::Record => {
                    // print all the data records in FIT file
                    //println!("{:#?}", item.fields());

                    // Retrieve the FitDataField struct.
                    for fld in item.fields().iter() {
                            match fld.number() {

                                0 => {
                                     let semi : i64 = fld.value().try_into().expect("conversion failed");  //semicircles
                                     let degrees_lat = semi_to_degrees(semi);
                                     println!("Name = {}, Value = {}, Units = {}",
                                              fld.name(), degrees_lat, fld.units());
                                },

                                1 => {
                                     let semi : i64 = fld.value().try_into().expect("conversion failed");  //semicircles
                                     let degrees_lng = semi_to_degrees(semi);
                                     println!("Name = {}, Value = {}, Units = {}",
                                              fld.name(), degrees_lng, fld.units());
                                },

                                3|4|5|73|78|253 =>{
                                       println!("Name = {}, Value = {}, Units = {}",
                                             fld.name(), fld.value(), fld.units());
                                 },

                                _ => print!("{}", "")  // matches other patterns 
                            }
                    }
                    },
                _ => print!("{}", "")  // matches other patterns 
                }
        }
    }
}                                            

