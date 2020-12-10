

#[derive(Debug)]
pub struct Header<'a>{
    pub version: &'a [u8],
    pub information: Information<'a>,
}

#[derive(Debug)]
pub struct Information<'a> {
    pub title: &'a [u8],
    pub sub_title: &'a [u8],
    pub interpret: &'a [u8],
    pub album: &'a [u8],
    pub author: &'a [u8],
    pub copyright: &'a [u8],
    pub instructional:&'a[u8],
    /*
    pub tempo: u32,
    pub key: u8,
    pub octave: u8,
    pub number_of_measures: u32,
    pub number_of_tracks: u32,
    */
  //  pub midi_channels:Vec<MIDIChannel>,
}
/*
pub struct MIDIChannel{
    pub instrument: u32,
    pub volume: u8,
    pub balance: u8,
    pub chorus: u8,
    pub reverb: u8,
    pub phaser: u8,
    pub tremolo: u8,
    pub blank1: u8,
    pub blank2: u8,
}

pub struct Measure{
   pub header: u8, // allowed 0-7
   pub numerator_of_key_signature: u8,
   pub denominator_of_key_signature: u8,
   pub end_of_repeat: u8,
   pub number_of_alternate_ending: u8,
   pub marker:[u8],
   pub tonality_of_measure:u8,
}

pub struct Track {
   pub header:u8,
   pub name:[u8;40],
   pub number_of_strings:i32,
   pub tuning_of_strings:[i32;6],
   pub port:i32,
   pub channel:i32,
   pub channel_e:i32,
   pub number_of_frets:i32,
   pub height_of_capo:i32,
   pub color_of_track:Color
}

pub struct MeasureTrackPair {
   pub number_of_beats: i32,
   pub measure:Measure,
   pub track:Track
}

pub struct Beat {
   pub header:u8,
   pub status:u8,
   pub beat_duration:u8,
   pub n_tuplet:i32,
   pub text: DynamicString,
}


pub struct ChordDiagram {
   pub header:u8,
   pub sharp:i8,
   pub blank1:u8,
   pub blank2:u8,
   pub blank3:u8,
   pub root:u8,
   pub chord_type:u8,


}

pub struct Color {
   pub space:u8, // always 0x0
   pub blue:u8,
   pub green:u8,
   pub red:u8
}

pub struct DynamicString{
   pub length:i32,
   pub str:[u8],
}
*/
