use regex::Regex;
use token::Token;
use token::TextTag;
use token::Token::*;
use token::TextTag::*;

use self::Element::*;

#[derive(Clone, Debug)]
pub enum Element {
    Mark(Vec<Token>, String),
    Raw(String),
}

lazy_static! {
    static ref MARKUP : Regex
        = Regex::new(r"'{3} '{2}.*?'{2} '{3}|\{{3}[^{}]+\}{3}|'{2,3}.*?'{2,3}|[-_~,^]{2}[^-~\r\n]+[-_~,^]{2}|\{{1,2}[{\[].*?[}\]]\}{1,2}|={1,}.*?={1,}[\r\n]|\[{1,2}.*?\]{1,2}").unwrap();
static ref MARKUP_CAPS : Regex
        = Regex::new(r"('{3} '{2}.*?'{2} '{3}|\{{3}[^{}]+\}{3}|'{2,3}.*?'{2,3}|[-_~,^]{2}[^-~\r\n]+?[-_~,^]{2}|\{{1,2}[{\[].*?[}\]]\}{1,2}|={1,}.*?={1,}[\r\n]|\[{1,2}.*?\]{1,2})").unwrap();
static ref ANY : Regex
        = Regex::new(r"([\s\S]+)").unwrap();
    //regex rule - http://regexr.com/3dbmo
}

fn split_blocks(text: &str) -> Vec<&str> {
    let mut tokens = vec![];
    let mut current = 0;
    for (begin, end) in MARKUP.find_iter(text) {
        match &text[current..begin] {
            "" => {}
            t => tokens.push(t),
        }
        tokens.push(&text[begin..end]);
        current = end;
    };
    match &text[current..text.len()] {
        "" => {}
        t => tokens.push(t),
    };
    //println!("blocks : {:?}", &tokens);
    tokens
}

pub fn tokenize(text: &str) -> Option<Vec<Element>> {
    let mut blocks = vec![];

    for block in split_blocks(text) {
        if let Some(caps) = MARKUP_CAPS.captures(block) {
            blocks.push(Mark(granularize(caps.at(1).unwrap_or("")).unwrap(),
                             block.to_owned()));
        }
        else if let Some(caps) = ANY.captures(block) {
            blocks.push(Raw(block.to_owned()));
        }
        else {
            blocks.push(Raw(block.to_owned()));
        }
    }

    Some(blocks)
}

lazy_static! {
    static ref SPLIT : Regex = Regex::new("\\#[a-zA-Z0-9]+|\\{{1,2}[{\\[]|[}\\]]\\}{1,2}|\\[{1,2}|\\]{1,2}|''' ''|'' '''|-{2}|~{2}|'{3}|'{2}|,{2}|\\^{2}|_{2}|={1,}|\\+[1-5]{1}|\\w+\\(.*?\\)").unwrap();
}

fn split_atom(block: &str) -> Vec<&str> {
    let mut result = vec![];
    let mut current = 0;

    // input iteration
    for (begin, end) in SPLIT.find_iter(block) {
        //println!("{} {} {}", begin, end, block);
        let atom = &block[current..begin];
        result.push(atom); //insert between ident to ident
        let atom = &block[begin..end];
        result.push(atom); //insert ident
        current = end;
    }

    result.push(&block[current..block.len()]);
    //println!("atoms : {:?}", result);
    result
}

lazy_static! {
    static ref HEADTAG: Regex = Regex::new("^={1,}|={1,}$").unwrap();
    static ref FUNC: Regex = Regex::new(r"\w+\(.*?\)").unwrap();
    static ref PREFILTER: Regex = Regex::new(r".*?").unwrap();//capture any
}

fn granularize(block: &str) -> Option<Vec<Token>> {
    let mut result = vec![];

    let mut bold_opened = false;
    let mut italic_opened = false;
    let mut strike_opened = false;
    let mut strike2_opened = false;
    let mut sup_opened = false;
    let mut sub_opened = false;
    let mut del_opened = false;
    let mut under_opened = false;
    let mut header_opened = false;
    let mut header_level = 0;

    for atom in split_atom(block) {
        let tok: Token = match &*atom {
            ""        => continue,

            "{{{"     => TrippleBracketOpen,
            "}}}"     => TrippleBracketClose,
            "{{["     => BoxedTextOpen,
            "]}}"     => BoxedTextClose,
            "[["      => HyperlinkTagOpen,
            "]]"      => HyperlinkTagClose,
            "["       => FunctionTagOpen,
            "]"       => FunctionTagClose,
    
            //TextEffectOpen&Close
            "''' ''"  => TextEffectOpen(BoldItalic),
            "'' '''"  => TextEffectClose(BoldItalic),
            "'''"     => {
                if bold_opened == false {
                    bold_opened = !bold_opened;
                    TextEffectOpen(Bold)
                }
                else {
                    bold_opened = !bold_opened;
                    TextEffectClose(Bold)
                }
            },
            "''"      => {
                if italic_opened == false {
                    italic_opened = !italic_opened;
                    TextEffectOpen(Italic)
                }
                else {
                    italic_opened = !italic_opened;
                    TextEffectClose(Italic)
                }
            },
            "--"      => {
                if strike_opened == false {
                    strike_opened = !strike_opened;
                    TextEffectOpen(Strikethrough)
                }
                else {
                    strike_opened = !strike_opened;
                    TextEffectClose(Strikethrough)
                }
            },
            "~~"      => {
                if strike2_opened == false {
                    strike2_opened = !strike2_opened;
                    TextEffectOpen(Strikethrough)
                }
                else {
                    strike2_opened = !strike2_opened;
                    TextEffectClose(Strikethrough)
                }
            },
            "__"      => {
                if under_opened == false {
                    under_opened = !under_opened;
                    TextEffectOpen(Underline)
                }
                else {
                    under_opened = !under_opened;
                    TextEffectClose(Underline)
                }
            },
            "^^"      => {
                if sup_opened == false {
                    sup_opened = !sup_opened;
                    TextEffectOpen(Superscript)
                }
                else {
                    sup_opened = !sup_opened;
                    TextEffectClose(Superscript)
                }
            },
            ",,"      => {
                if sub_opened == false {
                    sub_opened = !sub_opened;
                    TextEffectOpen(Subscript)
                }
                else {
                    sub_opened = !sub_opened;
                    TextEffectClose(Subscript)
                }
            },
    
            //TODO: Add checker of Table, Boxed Text, Function in Tag
            //""        => BoxedtextTagOpen,
            //""        => BoxedtextTagClose,
            //""        => InTagFunction,
            //""        => TableTag,
            //TODOEND

            x if FUNC.is_match(x) => {
                Text(x.to_owned())
            },
            x if HEADTAG.is_match(x) => {
                let level = x.matches("=").count();

                if level > 6 {
                    Text(x.to_owned()) // lexing disapproval
                }
                else {
                    if header_opened == false {
                        header_opened = !header_opened;
                        HeaderOpen(level)
                    }
                    else {
                        header_opened = !header_opened;
                        header_level = 0;
                        HeaderClose(level)
                    }
                }
            },
            x         => Text(x.to_owned()),
        };
        result.push(tok);
    }

    Some(result)
}

//TODO: Remake test code
/*#[test]
fn _test_must_panic() {

    let test_text = 
"= 문단 - Man 1 =
== 문단 2 ==
=== 문단 3 ===
==== bimundan ====z
======= 문단 7 =======

'''굵게'''
''기울임''

''''비문법''''

~~~~취소선 1~~~~
----취소선 2----
--~~취소선 3~~-- 
~~-- 취소선 4 --~~

-- chui -- so --

-- i -- er -- san --

,,아래첨자,,
^^위첨자^^

__밑줄__
''' ''굵게 기울임'' '''
[include(틀:루비,글자=text,루비=upper ruby text)]
[[http://www.google.co.kr/search?q=%20%4D%3E%2D|출력]]
{{{+3 텍스트}}}
{{[ Boxed Text ]}}
{{{#green __밑줄 포함 녹색__}}}
__{{{#green 밑줄 제외 녹색}}}__";
 
    for tok in tokenize(test_text) {
        for t in tok {
            match t {
                Element::Mark(V, T) => {
                    println!("token(mark) : {}", T);
                    for sub in V {
                        println!(">> sub token : {} {:?}", sub, sub);
                    }
                },
                Element::Raw(T) =>
                    println!("token(raw) : {}", T),
            };
        }
    }

    assert_eq!(true, false);
}*/
