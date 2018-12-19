use super::*;

use super::char_buf::new_char_buf;

#[test]
    fn test_parse_ipv4_address() {
        let mut tb = new_char_buf("".as_bytes());
        let ip = parse_ipv4_address(&mut tb).unwrap();
        assert_eq!(0, tb.len());
        assert_eq!(None, ip);

        let mut tb = new_char_buf("foo".as_bytes());
        let ip = parse_ipv4_address(&mut tb).unwrap();
        assert_eq!(None, ip);
        assert_eq!(1, tb.len());
        assert_eq!("f", tb.pop().unwrap().unwrap().to_string());

        let mut tb = new_char_buf("12.34.56.foo".as_bytes());
        let ip = parse_ipv4_address(&mut tb).unwrap();
        assert_eq!(None, ip);
        assert_eq!(10, tb.len());
        assert_eq!("1", tb.pop().unwrap().unwrap().to_string());

        let mut tb = new_char_buf("12.34.56.78.foo".as_bytes());
        let ip = parse_ipv4_address(&mut tb).unwrap().unwrap();
        assert_eq!("12.34.56.78", ip.to_string());
        assert_eq!(1, tb.len());
        assert_eq!(".", tb.pop().unwrap().unwrap().to_string());

        let mut tb = new_char_buf("12.34.56.78".as_bytes());
        let ip = parse_ipv4_address(&mut tb).unwrap().unwrap();
        assert_eq!("12.34.56.78", ip.to_string());
        assert_eq!(0, tb.len());
        assert_eq!(None, tb.pop().unwrap());
    }

#[test]
fn test_parse_dec_octet() -> Result<()> {
    let mut tb = new_char_buf("".as_bytes());
    let dec_octet = parse_dec_octet(&mut tb)?;
    assert_eq!(false, dec_octet.is_some());
    assert_eq!(0, tb.len());

    let mut tb = new_char_buf("0".as_bytes());
    let dec_octet = parse_dec_octet(&mut tb)?;
    assert_eq!(true, dec_octet.is_some());
    let dec_octet = dec_octet.unwrap();
    assert_eq!(0, tb.len());
    assert_eq!("0", dec_octet.to_string());

    let mut tb = new_char_buf("1.".as_bytes());
    let dec_octet = parse_dec_octet(&mut tb)?;
    assert_eq!(true, dec_octet.is_some());
    let dec_octet = dec_octet.unwrap();
    assert_eq!(1, tb.len());
    assert_eq!("1", dec_octet.to_string());

    let token = tb.pop()?;
    assert_eq!(true, token.is_some());
    let token = token.unwrap();
    assert_eq!(".", token.to_string());

    let mut tb = new_char_buf("255".as_bytes());
    let dec_octet = parse_dec_octet(&mut tb)?;
    assert_eq!(true, dec_octet.is_some());
    let dec_octet = dec_octet.unwrap();
    assert_eq!(0, tb.len());
    assert_eq!("255", dec_octet.to_string());

    let mut tb = new_char_buf("256".as_bytes());
    let dec_octet = parse_dec_octet(&mut tb)?;
    assert_eq!(false, dec_octet.is_some());
    assert_eq!(3, tb.len());

    let mut tb = new_char_buf("2555".as_bytes());
    let dec_octet = parse_dec_octet(&mut tb)?;
    assert_eq!(true, dec_octet.is_some());
    let dec_octet = dec_octet.unwrap();
    assert_eq!(0, tb.len());
    assert_eq!("255", dec_octet.to_string());

    let token = tb.pop()?;
    assert_eq!(true, token.is_some());
    let token = token.unwrap();
    assert_eq!("5", token.to_string());

    Ok(())
}

#[test]
fn test_path() -> Result<()> {
    let mut tb = new_char_buf("".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(false, path.absolute);
    assert_eq!(1, path.segments.len());
    assert_eq!("", path.to_string());

    let mut tb = new_char_buf("/".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(true, path.absolute);
    assert_eq!(1, path.segments.len());
    assert_eq!("/", path.to_string());

    let mut tb = new_char_buf("/foo/".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(true, path.absolute);
    assert_eq!(2, path.segments.len());
    assert_eq!("/foo/", path.to_string());

    let mut tb = new_char_buf("/foo/bar".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(true, path.absolute);
    assert_eq!(2, path.segments.len());
    assert_eq!("/foo/bar", path.to_string());

    let mut tb = new_char_buf("foo/bar".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(false, path.absolute);
    assert_eq!(2, path.segments.len());
    assert_eq!("foo/bar", path.to_string());

    Ok(())
}

#[test]
fn test_query() -> Result<()> {
    let mut tb = new_char_buf("foo}bar".as_bytes());

    let query = parse_query(&mut tb)?;
    assert_eq!("foo", query.to_string());

    let c = tb.pop()?.unwrap();
    assert_eq!(Char::Ascii(b'}'), c);

    let query = parse_query(&mut tb)?;
    assert_eq!("bar", query.to_string());

    assert_eq!(None, tb.pop()?);

    Ok(())
}

#[test]
fn test_fragment() -> Result<()> {
    let mut tb = new_char_buf("foo}bar".as_bytes());

    let fragment = parse_fragment(&mut tb)?;
    assert_eq!("foo", fragment.to_string());

    let c = tb.pop()?.unwrap();
    assert_eq!(Char::Ascii(b'}'), c);

    let fragment = parse_fragment(&mut tb)?;
    assert_eq!("bar", fragment.to_string());

    assert_eq!(None, tb.pop()?);

    Ok(())
}
