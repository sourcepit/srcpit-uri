use super::*;

use super::char_buf::new_char_buf;

#[test]
fn test_path() -> Result<()> {
    let mut tb = new_char_buf("".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(false, path.absolute);
    assert_eq!(0, path.segments.len());
    assert_eq!("", path.to_string());

    let mut tb = new_char_buf("/".as_bytes());
    let path = parse_path(&mut tb)?;
    assert_eq!(true, path.absolute);
    assert_eq!(0, path.segments.len());
    assert_eq!("/", path.to_string());

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
