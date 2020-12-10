use super::nom::{named, do_parse, take};
use super::file_defs::*;
use super::common::*;

named!(pub header<&[u8],Header>,
  do_parse!(
     version: take!(30) >>
     information : information >>
     (
         Header{
            version:version,
            information:information
         })
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
    println!("{:?}",result.unwrap());
}

#[test]
fn test_header_parser() {
    let f = include_bytes!("../examples/that girl.gp5");
    let result = header(f);
    println!("{:?}",result.unwrap_err())
}
