use winnow::error::AddContext;
use winnow::error::ParserError;
use winnow::stream::AsChar;
use winnow::*;

/// Represents the possible tokens that can be found within a [PEP-292] template
///
/// [PEP-292]: https://peps.python.org/pep-0292
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'slice> {
    /// A escape sequence `$$`
    Escape,
    /// A simple placeholder substitution `$identifier`
    SimplePlaceholder(&'slice [u8]),
    /// A bracket placeholder substitution `${identifier}`
    BracketPlaceholder(&'slice [u8]),
    /// Other content present on the slice
    Content(&'slice [u8]),
}

// TODO use partial instead or generalize input
pub type Input<'i> = &'i [u8];

#[inline]
fn identifier_first<'i, E>(input: &mut Input<'i>) -> PResult<u8, E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    token::one_of((AsChar::is_alpha, b'_')).parse_next(input)
}
#[inline]
fn identifier_rest<'i, E>(input: &mut Input<'i>) -> PResult<&'i [u8], E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    token::take_while(0.., (AsChar::is_alphanum, b'_')).parse_next(input)
}

/// Implementation of a Python [identifier]
///
/// [identifier]: https://docs.python.org/release/2.6/reference/lexical_analysis.html#identifiers-and-keywords
#[inline]
fn identifier<'i, E>(input: &mut Input<'i>) -> PResult<&'i [u8], E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    (identifier_first, identifier_rest)
        .recognize()
        .parse_next(input)
}

#[inline]
fn simple_placeholder<'i, E>(input: &mut Input<'i>) -> PResult<&'i [u8], E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    combinator::preceded(b'$', identifier)
        .context("simple placeholder")
        .parse_next(input)
}

#[inline]
fn bracket_placeholder<'i, E>(input: &mut Input<'i>) -> PResult<&'i [u8], E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    combinator::preceded(b'$', combinator::delimited(b'{', identifier, b'}'))
        .context("bracket placeholder")
        .parse_next(input)
}

#[inline]
fn escape<'i, E>(input: &mut Input<'i>) -> PResult<(), E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    "$$".void().parse_next(input)
}

#[inline]
fn content<'i, E>(input: &mut Input<'i>) -> PResult<&'i [u8], E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    if input.contains(&b'$') {
        token::take_till1(b'$').parse_next(input)
    } else {
        combinator::rest.parse_next(input)
    }
}

/// A [winnow] parser for [PEP-292] tokens.
///
/// [winnow]: winnow
/// [PEP-292]: https://peps.python.org/pep-0292
#[inline]
pub fn token<'i, E>(input: &mut Input<'i>) -> PResult<Token<'i>, E>
where
    E: ParserError<Input<'i>> + AddContext<Input<'i>, &'static str>,
{
    combinator::alt((
        content.map(Token::Content),
        escape.value(Token::Escape),
        bracket_placeholder.map(Token::BracketPlaceholder),
        simple_placeholder.map(Token::SimplePlaceholder),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use speculoos::assert_that;
    use winnow::error::ErrorKind;
    use winnow::error::ParseError;

    #[rstest]
    #[case::full(b"$hello_world10", Some(b"hello_world10" as _))]
    #[case::one_letter(b"$h", Some(b"h" as _))]
    #[case::starts_with_digit(b"$1", None)]
    #[case::starts_with_underscore(b"$_", Some(b"_" as _))]
    #[trace]
    fn test_simple_placeholder(#[case] input: &[u8], #[case] expected: Option<&[u8]>) {
        let result = simple_placeholder::<()>.parse(input);

        assert_that!(result.ok()).is_equal_to(expected)
    }

    #[rstest]
    #[case::full(b"${hello_world10}", Some(b"hello_world10" as _))]
    #[case::one_letter(b"${h}", Some(b"h" as _))]
    #[case::starts_with_digit(b"${1}", None)]
    #[case::starts_with_underscore(b"${_}", Some(b"_" as _))]
    #[trace]
    fn test_bracket_placeholder(#[case] input: &[u8], #[case] expected: Option<&[u8]>) {
        let result = bracket_placeholder::<()>.parse(input);
        assert_that!(result.ok()).is_equal_to(expected)
    }

    #[rstest]
    #[case(b"$$", Ok(()))]
    #[trace]
    fn test_scape(
        #[case] input: &[u8],
        #[case] expected: Result<(), ParseError<&[u8], ErrorKind>>,
    ) {
        let res = escape.parse(input);
        assert_that!(res).is_equal_to(expected)
    }
    #[rstest]
    #[case::simple(b"Hello world", Some(b"Hello world" as _))]
    #[case::single(b"$", None)]
    #[case::end(b"end$", None)]
    #[case::start(b"$start", None)]
    #[case::middle(b"mid$dle", None)]
    #[trace]
    fn test_content(#[case] input: &[u8], #[case] expected: Option<&[u8]>) {
        let res = content::<()>.parse(input);
        assert_that!(res.ok()).is_equal_to(expected)
    }

    #[rstest]
    #[case::escape(b"$$", Some(Token::Escape))]
    #[case::simple(b"$hello", Some(Token::SimplePlaceholder(b"hello")))]
    #[case::brackets(b"${hello}", Some(Token::BracketPlaceholder(b"hello")))]
    #[case::content(b"some random text {}", Some(Token::Content(b"some random text {}")))]
    #[trace]
    fn test_token(#[case] input: &[u8], #[case] expected: Option<Token>) {
        let res = token::<()>.parse(input);
        assert_that!(res.ok()).is_equal_to(expected)
    }
}
