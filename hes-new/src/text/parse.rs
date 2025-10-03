use super::{Node, Tag};

use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::map,
    multi::many0,
    sequence::delimited,
};

fn parse_tag(input: &str) -> IResult<&str, Tag> {
    alt((
        map(tag("b"), |_| Tag::Bold),
        map(tag("i"), |_| Tag::Image),
        map(tag("u"), |_| Tag::UnknownParam),
        map(tag("t"), |_| Tag::TypeTotal),
        map(tag("e"), |_| Tag::EffectFeature),
        map(tag("w"), |_| Tag::TipWarn),
        map(tag("g"), |_| Tag::TipGoal),
        map(tag("c"), |_| Tag::CardTag),
    ))
    .parse(input)
}

fn open_tag(input: &str) -> IResult<&str, Tag> {
    delimited(char('['), parse_tag, char(']')).parse(input)
}

fn close_tag(input: &str) -> IResult<&str, Tag> {
    delimited(tag("[/"), parse_tag, char(']')).parse(input)
}

fn text<'a>(input: &'a str) -> IResult<&'a str, Node<'a>> {
    map(take_while1(|c| c != '['), Node::Text).parse(input)
}

fn tagged<'a>(input: &'a str) -> IResult<&'a str, Node<'a>> {
    let (input, tag) = open_tag(input)?;
    let (input, children) =
        many0(alt((tagged, text))).parse(input)?;
    let (input, close) = close_tag(input)?;

    if tag == close {
        Ok((input, Node::Tagged { tag, children }))
    } else {
        Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

pub fn parse_bbcode(input: &str) -> IResult<&str, Vec<Node>> {
    many0(alt((tagged, text))).parse(input)
}
