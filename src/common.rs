use nom::IResult;

pub fn dyn_str(i:&[u8]) -> IResult<&[u8],&[u8]> {
    return nom::number::streaming::le_u32(i)
        .and_then(|(i,n)|->IResult<&[u8],&[u8]>{
            return nom::take!(i,n)
        });
}
