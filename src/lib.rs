extern crate common_failures;
#[macro_use]
extern crate failure;
extern crate srcpit_token_buf as token_buf;

mod char_buf;

#[cfg(test)]
mod tests;

use common_failures::prelude::*;

use self::char_buf::Char;
use self::char_buf::CharStream;
use std::fmt::Write;
use std::net::Ipv4Addr;
use token_buf::TokenBuffer;
use token_buf::TokenStream;

//    URI           = scheme ":" hier-part [ "?" query ] [ "#" fragment ]

//    hier-part     = "//" authority path-abempty
//                  / path-absolute
//                  / path-rootless
//                  / path-empty

//    URI-reference = URI / relative-ref

//    absolute-URI  = scheme ":" hier-part [ "?" query ]

//    relative-ref  = relative-part [ "?" query ] [ "#" fragment ]

//    relative-part = "//" authority path-abempty
//                  / path-absolute
//                  / path-noscheme
//                  / path-empty

//    scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

//    authority     = [ userinfo "@" ] host [ ":" port ]
//    userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
//    host          = IP-literal / IPv4address / reg-name
//    port          = *DIGIT

//    reg-name      = *( unreserved / pct-encoded / sub-delims )

//    IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"

//    IPvFuture     = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )

//    IPv6address   =                            6( h16 ":" ) ls32
//                  /                       "::" 5( h16 ":" ) ls32
//                  / [               h16 ] "::" 4( h16 ":" ) ls32
//                  / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
//                  / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
//                  / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
//                  / [ *4( h16 ":" ) h16 ] "::"              ls32
//                  / [ *5( h16 ":" ) h16 ] "::"              h16
//                  / [ *6( h16 ":" ) h16 ] "::"

//    ls32          = ( h16 ":" h16 ) / IPv4address
enum Ls32 {
    Ipv6Part(H16, H16),
    Ipv4address(Ipv4address),
}

#[derive(Clone, Debug, PartialEq)]
struct H16(Char, Char, Char, Char);

impl std::fmt::Display for H16 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}{}{}{}", &self.0, &self.1, &self.2, &self.3)
    }
}
fn parse_h16<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<H16>>
where
    T: TokenStream<Char>,
{
    let mut tokens: Vec<Char> = Vec::new();
    for _ in 0..4 {
        if let Some(token) = tb.pop()? {
            if token.is_hex() {
                tokens.push(token);
                continue;
            }
            tb.push(token);
        }
        break;
    }

    match tokens.len() {
        4 => Ok(Some(H16(
            tokens.remove(0),
            tokens.remove(0),
            tokens.remove(0),
            tokens.remove(0),
        ))),
        _ => {
            tb.push_tokens(tokens);
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Ipv4address(DecOctet, DecOctet, DecOctet, DecOctet);

impl std::fmt::Display for Ipv4address {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}.{}.{}.{}", &self.0, &self.1, &self.2, &self.3)
    }
}

fn parse_ipv4_address<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Ipv4address>>
where
    T: TokenStream<Char>,
{
    let d1 = match parse_dec_octet(tb)? {
        Some(dec_octet) => dec_octet,
        None => return Ok(None),
    };

    let dot1 = match tb.pop()? {
        Some(c) => {
            if c.is(b'.') {
                Some(c)
            } else {
                tb.push(c);
                None
            }
        }
        None => None,
    };

    let dot1 = match dot1 {
        Some(c) => c,
        None => {
            tb.push_tokens(d1.0);
            return Ok(None);
        }
    };

    let d2 = match parse_dec_octet(tb)? {
        Some(dec_octet) => dec_octet,
        None => {
            tb.push(dot1);
            tb.push_tokens(d1.0);
            return Ok(None);
        }
    };

    let dot2 = match tb.pop()? {
        Some(c) => {
            if c.is(b'.') {
                Some(c)
            } else {
                tb.push(c);
                None
            }
        }
        None => None,
    };

    let dot2 = match dot2 {
        Some(c) => c,
        None => {
            tb.push_tokens(d2.0);
            tb.push(dot1);
            tb.push_tokens(d1.0);
            return Ok(None);
        }
    };

    let d3 = match parse_dec_octet(tb)? {
        Some(dec_octet) => dec_octet,
        None => {
            tb.push(dot2);
            tb.push_tokens(d2.0);
            tb.push(dot1);
            tb.push_tokens(d1.0);
            return Ok(None);
        }
    };

    let dot3 = match tb.pop()? {
        Some(c) => {
            if c.is(b'.') {
                Some(c)
            } else {
                tb.push(c);
                None
            }
        }
        None => None,
    };

    let dot3 = match dot3 {
        Some(c) => c,
        None => {
            tb.push_tokens(d3.0);
            tb.push(dot2);
            tb.push_tokens(d2.0);
            tb.push(dot1);
            tb.push_tokens(d1.0);
            return Ok(None);
        }
    };

    let d4 = match parse_dec_octet(tb)? {
        Some(dec_octet) => dec_octet,
        None => {
            tb.push(dot3);
            tb.push_tokens(d3.0);
            tb.push(dot2);
            tb.push_tokens(d2.0);
            tb.push(dot1);
            tb.push_tokens(d1.0);
            return Ok(None);
        }
    };

    Ok(Some(Ipv4address(d1, d2, d3, d4)))
}

#[derive(Clone, Debug, PartialEq)]
struct DecOctet(Vec<Char>);

impl std::fmt::Display for DecOctet {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in &self.0 {
            fmt.write_str(c.to_string().as_str())?;
        }
        Ok(())
    }
}

fn parse_dec_octet<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<DecOctet>>
where
    T: TokenStream<Char>,
{
    let mut tokens: Vec<Char> = Vec::new();
    for _ in 0..3 {
        if let Some(token) = tb.pop()? {
            if token.is_digit() {
                tokens.push(token);
                continue;
            }
            tb.push(token);
        }
        break;
    }

    let dec_octet_str = tokens
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

    match dec_octet_str.parse::<u8>() {
        Ok(dec_octet) => Ok(Some(DecOctet(tokens))),
        Err(e) => {
            tb.push_tokens(tokens);
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Path {
    segments: Vec<Segment>,
    absolute: bool,
}

impl std::fmt::Display for Path {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.absolute {
            fmt.write_char('/')?;
        }

        for (i, segment) in self.segments.iter().enumerate() {
            if i > 0 {
                fmt.write_char('/')?;
            }
            fmt.write_str(segment.to_string().as_str())?;
        }
        Ok(())
    }
}

fn parse_path<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Path>
where
    T: TokenStream<Char>,
{
    let path: Path;
    if let Some(path_absolute) = parse_path_absolute(tb)? {
        path = path_absolute;
    } else if let Some(path_rootless) = parse_path_rootless(tb)? {
        path = path_rootless;
    } else {
        path = Path {
            segments: vec![Segment(Vec::new())],
            absolute: false,
        };
    }
    Ok(path)
}

fn parse_path_abempty<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Path>>
where
    T: TokenStream<Char>,
{
    let mut segments: Vec<Segment> = Vec::new();
    loop {
        if let Some(token) = tb.pop()? {
            if token.is(b'/') {
                let segment = parse_segment(tb)?;
                segments.push(segment);
                continue;
            }
            tb.push(token);
            break;
        }
    }
    match segments.is_empty() {
        true => Ok(None),
        false => Ok(Some(Path {
            segments: segments,
            absolute: false,
        })),
    }
}

fn parse_path_absolute<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Path>>
where
    T: TokenStream<Char>,
{
    let path: Option<Path>;
    if let Some(token) = tb.pop()? {
        if token.is(b'/') {
            if let Some(path_rootless) = parse_path_rootless(tb)? {
                path = Some(Path {
                    segments: path_rootless.segments,
                    absolute: true,
                });
            } else {
                path = Some(Path {
                    segments: vec![Segment(Vec::new())],
                    absolute: true,
                });
            }
        } else {
            tb.push(token);
            path = None;
        }
    } else {
        path = None;
    }
    Ok(path)
}

fn parse_path_noscheme<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Path>>
where
    T: TokenStream<Char>,
{
    let mut segments: Vec<Segment> = Vec::new();
    if let Some(segment_nz_nc) = parse_segment_nz_nc(tb)? {
        segments.push(segment_nz_nc);
        loop {
            if let Some(token) = tb.pop()? {
                if token.is(b'/') {
                    let segment = parse_segment(tb)?;
                    segments.push(segment);
                    continue;
                }
                tb.push(token);
                break;
            }
        }
    }
    match segments.is_empty() {
        true => Ok(None),
        false => Ok(Some(Path {
            segments: segments,
            absolute: false,
        })),
    }
}

fn parse_path_rootless<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Path>>
where
    T: TokenStream<Char>,
{
    let mut segments: Vec<Segment> = Vec::new();
    if let Some(segment_nz) = parse_segment_nz(tb)? {
        segments.push(segment_nz);
        loop {
            if let Some(token) = tb.pop()? {
                if token.is(b'/') {
                    let segment = parse_segment(tb)?;
                    segments.push(segment);
                    continue;
                }
                tb.push(token);
            }
            break;
        }
    }
    match segments.is_empty() {
        true => Ok(None),
        false => Ok(Some(Path {
            segments: segments,
            absolute: false,
        })),
    }
}

fn parse_path_empty<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Path>>
where
    T: TokenStream<Char>,
{
    let path: Option<Path>;

    if let Some(token) = tb.pop()? {
        if token.is_pchar() {
            path = None;
        } else {
            path = Some(Path {
                segments: Vec::new(),
                absolute: false,
            });
        }
        tb.push(token);
    } else {
        path = Some(Path {
            segments: vec![Segment(Vec::new())],
            absolute: false,
        });
    }
    Ok(path)
}

#[derive(Clone, Debug, PartialEq)]
struct Segment(Vec<Char>);

impl std::fmt::Display for Segment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in &self.0 {
            fmt.write_str(c.to_string().as_str())?;
        }
        Ok(())
    }
}

fn parse_segment<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Segment>
where
    T: TokenStream<Char>,
{
    let mut tokens: Vec<Char> = Vec::new();
    loop {
        if let Some(token) = tb.pop()? {
            if token.is_pchar() {
                tokens.push(token);
                continue;
            }
            tb.push(token);
        }
        break;
    }
    Ok(Segment(tokens))
}

fn parse_segment_nz<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Segment>>
where
    T: TokenStream<Char>,
{
    let mut tokens: Vec<Char> = Vec::new();
    loop {
        if let Some(token) = tb.pop()? {
            if token.is_pchar() {
                tokens.push(token);
                continue;
            }
            tb.push(token);
        }
        break;
    }
    match tokens.is_empty() {
        true => Ok(None),
        false => Ok(Some(Segment(tokens))),
    }
}

fn parse_segment_nz_nc<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Option<Segment>>
where
    T: TokenStream<Char>,
{
    let mut tokens: Vec<Char> = Vec::new();
    loop {
        if let Some(token) = tb.pop()? {
            if token.is_unreserved()
                || token.is_pct_encoded()
                || token.is_sub_delim()
                || token.is(b'@')
            {
                tokens.push(token);
                continue;
            }
            tb.push(token);
        }
        break;
    }
    match tokens.is_empty() {
        true => Ok(None),
        false => Ok(Some(Segment(tokens))),
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Query(Vec<Char>);

impl std::fmt::Display for Query {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in &self.0 {
            fmt.write_str(c.to_string().as_str())?;
        }
        Ok(())
    }
}

fn parse_query<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Query>
where
    T: TokenStream<Char>,
{
    Ok(Query(parse_fragment(tb)?.0))
}

#[derive(Clone, Debug, PartialEq)]
struct Fragment(Vec<Char>);

impl std::fmt::Display for Fragment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in &self.0 {
            fmt.write_str(c.to_string().as_str())?;
        }
        Ok(())
    }
}

fn parse_fragment<T>(tb: &mut TokenBuffer<Char, T>) -> Result<Fragment>
where
    T: TokenStream<Char>,
{
    let mut tokens: Vec<Char> = Vec::new();
    loop {
        if let Some(token) = tb.pop()? {
            if token.is_pchar() || token.is(b'/') || token.is(b'?') {
                tokens.push(token);
                continue;
            }
            tb.push(token);
        }
        break;
    }
    Ok(Fragment(tokens))
}
