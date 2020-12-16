use super::nom::{named, do_parse, take,i32,tag,map,map_res,take_till,take_while,peek,tap,many0};
use super::nom::number::streaming::{le_u32,le_u8};
use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use super::nom::character::{{is_digit,is_alphabetic}};
use super::file_defs::*;
use super::common::*;
use nom::IResult;
use std::str;

named!(pub version_number<&[u8],u32>,
    do_parse!(
        prefix: tag!(VERSION_STRING_PREFIX) >>
        w2 : take_while!(is_alphabetic) >>
        major: map_res!(take_while!(is_digit), |s: &[u8]| {let ds = str::from_utf8(s); ds.unwrap().parse::<u32>()}) >>
        dot: tag!(".") >>
        minjor: map_res!(take_while(is_digit), |s: &[u8]| {let ds = str::from_utf8(s); ds.unwrap().parse::<u32>()}) >>
        (
             (major * 100 + minjor).into()
        )
    )
);




named!(pub header<&[u8],Header>,
  do_parse!(
     slen: le_u8 >> //the version string length
     version: map!(take!(slen),|s| {
        (s,version_number(s).unwrap().1)
     }) >>
     skipped : take!(30 - slen) >> //
     information: information >>

     (
         Header{
            version:version.0,
            version_number : version.1,
            information:information
         }
         )
   )
);

named!(pub information<&[u8],Information>,
    do_parse!(
        title: dyn_str >>
        sub_title: dyn_str >>
        interpret: dyn_str >>
        album: dyn_str >>
        author: dyn_str >>
        copyright : dyn_str >>
        instructional: dyn_str >>
        (
           Information{
            title:title,
            sub_title:sub_title,
            interpret:interpret,
            album:album,
            author:author,
            copyright:copyright,
            instructional: instructional,
           }
        )
    )
);


#[test]
fn test_information() {
    let f = include_bytes!("../examples/that girl.gp5");
    let result = information(&f[31..f.len()-1]);
    let (i,info) = result.unwrap();
    assert_eq!(std::str::from_utf8(info.title).unwrap(),"\tthat girl");
}

#[test]
fn test_header_parser() {
    let f = include_bytes!("../examples/that girl.gp5");
    let result = header(f);
    let (i,h) = result.unwrap();
    assert_eq!(h.version_number,510);
}

#[test]
fn test_parse_version_number() {
    let f = VERSION_STRING_PREFIX.to_owned() + "5.12 ";
    let result = version_number(f.as_bytes());
    let (_,version) = result.unwrap();
    assert_eq!(version,512);
}
