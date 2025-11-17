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
    let mut fp = File::open("tests/test.fit").expect("file not found");
    if let Ok(data) = fitparser::from_reader(&mut fp) {
        // print the data in FIT file
        //println!("{:#?}", data);
        for item in &data {
            match item.kind() {
                MesgNum::Record => {
//                    println!("{:#?}", item.fields());
                
                    println!();

// TODO - Need to figure out how to  iterate through all the numbered indexes properly.
// TODO - The indexs on .get don't always match - esp for the timestamp.
                    if item.fields().get(11).is_some() {
                         let time_stamp: &FitDataField = item.fields().get(11).unwrap();
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             time_stamp.name(), time_stamp.number(), time_stamp.value(), time_stamp.units());
                      };
                     
                    if item.fields().get(0).is_some() {
                         let pos_lat: &FitDataField = item.fields().get(0).unwrap();
                         let semi : i64 = pos_lat.value().try_into().expect("conversion failed");  //semicircles
                         let degrees_lat = semi_to_degrees(semi);
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             pos_lat.name(), pos_lat.number(), degrees_lat, pos_lat.units());
                      };

                    if item.fields().get(1).is_some() {
                         let pos_lon: &FitDataField = item.fields().get(1).unwrap();
                         let semi : i64 = pos_lon.value().try_into().expect("conversion failed");  //semicircles
                         let degrees_lon = semi_to_degrees(semi);
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             pos_lon.name(), pos_lon.number(), degrees_lon, pos_lon.units());
                      };

                    if item.fields().get(2).is_some() {
                         let heart_rate: &FitDataField = item.fields().get(2).unwrap();
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             heart_rate.name(), heart_rate.number(), heart_rate.value(), heart_rate.units());
                      };

                    if item.fields().get(3).is_some() {
                         let cadence: &FitDataField = item.fields().get(3).unwrap();
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             cadence.name(), cadence.number(), cadence.value(), cadence.units());
                      };

                    if item.fields().get(4).is_some() {
                         let distance: &FitDataField = item.fields().get(4).unwrap();
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             distance.name(), distance.number(), distance.value(), distance.units());
                      };

                    if item.fields().get(7).is_some() {
                         let speed: &FitDataField = item.fields().get(7).unwrap();
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             speed.name(), speed.number(), speed.value(), speed.units());
                      };

                    if item.fields().get(8).is_some() {
                         let cadence: &FitDataField = item.fields().get(8).unwrap();
                         println!("Name = {}, Number = {}, Value = {}, Units = {}",
                             cadence.name(), cadence.number(), cadence.value(), cadence.units());
                      };


                     
                },
                _ => print!("{}", "") 
            }
        }
    }
}    
