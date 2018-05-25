use token::Token;
use token::TextTag;
use token::Token::*;
use token::TextTag::*;

//ONLY HTML, simpl parser
fn parse(tokens: &Vec<Token>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for t in tokens {
        let out: String = match *t {
            FunctionTagOpen => "[".to_owned(),
            FunctionTagClose => "]".to_owned(),
            HyperlinkTagOpen => "[[".to_owned(),
            HyperlinkTagClose => "]]".to_owned(),
            TrippleBracketOpen => "{{{".to_owned(),
            TrippleBracketClose => "}}}".to_owned(),
            BoxedTextOpen => "{{[".to_owned(),
            BoxedTextClose => "]}}".to_owned(),
            HeaderOpen(x) => format!("<h{}>", x),
            HeaderClose(x) => format!("</h{}>", x),

            //TextEffectOpen
            TextEffectOpen(Bold) => "<b>".to_owned(),
            TextEffectOpen(Italic) => "<i>".to_owned(),
            TextEffectOpen(BoldItalic) => "<b><i>".to_owned(),
            TextEffectOpen(Strikethrough) => "<del>".to_owned(),
            TextEffectOpen(Underline)  => "<u>".to_owned(),
            TextEffectOpen(Superscript) => "<sup>".to_owned(),
            TextEffectOpen(Subscript) => "<sub>".to_owned(),
            //TextEffectClose
            TextEffectClose(Bold) => "</b>".to_owned(),
            TextEffectClose(Italic) => "</i>".to_owned(),
            TextEffectClose(BoldItalic) => "</i></b>".to_owned(),
            TextEffectClose(Strikethrough) => "</del>".to_owned(),
            TextEffectClose(Underline)  => "</u>".to_owned(),
            TextEffectClose(Superscript) => "</sup>".to_owned(),
            TextEffectClose(Subscript) => "</sub>".to_owned(),

            //TODO: Add checker of Table, Boxed Text, Function in Tag
            BoxedtextTagOpen => "".to_owned(),
            BoxedtextTagClose => "".to_owned(),
            InTagFunction => "".to_owned(),
            TableTag => "".to_owned(),
            //TODOEND

            Text(ref x) => x.clone(),
            NotParsable(ref x) => x.clone(),
        };
        v.push(out);
    }
    v
}

#[test]
fn _test_must_panic() {

    use lexer::*;

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
                    for sub in parse(&V) {
                        println!(">> sub token : {} {:?}", sub, sub);
                    }
                },
                Element::Raw(T) =>
                    println!("token(raw) : {}", T),
            };
        }
    }

    assert_eq!(true, false); //make unequal assert function for displaying log
}
