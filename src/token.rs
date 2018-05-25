use self::TextTag::*;
use self::Token::*;
use std::fmt;

use std::iter;

#[derive(Clone, Debug)]
pub enum TextTag {
    Bold, // ''' Bold Text '''
    Italic, // '' Italic Text ''
    BoldItalic, // ''' '' Bold Italic Text '' '''
    Strikethrough, // -- Strikethrough -- or ~~ Strikethrough ~~
    Underline, // __ Underline __
    Superscript, // ^^ Superscript ^^ 
    Subscript, // ,, Subscript ,,
}

#[derive(Clone, Debug)]
pub enum Token {
    FunctionTagOpen,
    FunctionTagClose,
    HyperlinkTagOpen,
    HyperlinkTagClose,
    TrippleBracketOpen,
    TrippleBracketClose,

    /*this is text effect but tag is similar to tripple bracket tag*/
    BoxedTextOpen,
    BoxedTextClose,

    TextEffectOpen(TextTag),
    TextEffectClose(TextTag),
    HeaderOpen(usize),
    HeaderClose(usize),

    //TODO: Add checker of Table, Boxed Text, Function in Tag
    BoxedtextTagOpen,
    BoxedtextTagClose,
    InTagFunction,
    TableTag,
    //TODOEND

    Text(String),
    NotParsable(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            FunctionTagOpen => "[".to_owned(),
            FunctionTagClose => "]".to_owned(),
            HyperlinkTagOpen => "[[".to_owned(),
            HyperlinkTagClose => "]]".to_owned(),
            TrippleBracketOpen => "{{{".to_owned(),
            TrippleBracketClose => "}}}".to_owned(),
            BoxedTextOpen => "{{[".to_owned(),
            BoxedTextClose => "]}}".to_owned(),
            HeaderOpen(x) => iter::repeat("=").take(x).collect::<String>(),
            HeaderClose(x) => iter::repeat("=").take(x).collect::<String>(),

            //TextEffectOpen
            TextEffectOpen(Bold) => "'''".to_owned(),
            TextEffectOpen(Italic) => "''".to_owned(),
            TextEffectOpen(BoldItalic) => "''' ''".to_owned(),
            TextEffectOpen(Strikethrough) => "--".to_owned(),
            TextEffectOpen(Underline)  => "__".to_owned(),
            TextEffectOpen(Superscript) => "^^".to_owned(),
            TextEffectOpen(Subscript) => ",,".to_owned(),
            //TextEffectClose
            TextEffectClose(Bold) => "'''".to_owned(),
            TextEffectClose(Italic) => "''".to_owned(),
            TextEffectClose(BoldItalic) => "'' '''".to_owned(),
            TextEffectClose(Strikethrough) => "--".to_owned(),
            TextEffectClose(Underline)  => "__".to_owned(),
            TextEffectClose(Superscript) => "^^".to_owned(),
            TextEffectClose(Subscript) => ",,".to_owned(),

            //TODO: Add checker of Table, Boxed Text, Function in Tag
            BoxedtextTagOpen => "".to_owned(),
            BoxedtextTagClose => "".to_owned(),
            InTagFunction => "".to_owned(),
            TableTag => "".to_owned(),
            //TODOEND

            Text(ref x) => x.clone(),
            NotParsable(ref x) => x.clone(),
        };
        write!(f, "{}", out)
    }
}

