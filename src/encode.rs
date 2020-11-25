use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// from https://cloud.google.com/storage/docs/request-endpoints
// !, #, $, &, ', (, ), *, +, ,, /, :, ;, =, ?, @, [, ]
const NORMAL_SET: &AsciiSet = &CONTROLS // no way to start with an empty set...
    .add(b'!')
    .add(b'#')
    .add(b'$')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b']');

pub(crate) fn normal(input: impl AsRef<str>) -> String {
    utf8_percent_encode(input.as_ref(), NORMAL_SET).to_string()
}

const SLASH_SET: &AsciiSet = &NORMAL_SET.add(b'/');

pub(crate) fn slash(input: impl AsRef<str>) -> String {
    utf8_percent_encode(input.as_ref(), SLASH_SET).to_string()
}
