use criterion::{black_box, criterion_group, criterion_main, Criterion};
use logos::Logos;

#[derive(Logos, PartialEq, Debug, Clone, Copy)]
pub enum KindsRustLogos {
    #[regex(r"[^\S\n]+")]
    Whitespace,
    #[regex(r"//(.*)")]
    CommentLine,
    #[regex(r"/\*([^*]|\*+[^*/])*\*/")]
    CommentBlock,
    #[regex(r"\n")]
    Newline,

    #[token("mut")]
    KeywordMut,
    #[token("let")]
    KeywordLet,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("fn")]
    KeywordFn,
    #[token("struct")]
    KeywordStruct,
    #[token("enum")]
    KeywordEnum,
    #[token("match")]
    KeywordMatch,
    #[token("use")]
    KeywordUse,
    #[token("mod")]
    KeywordMod,
    #[token("pub")]
    KeywordPub,
    #[token("crate")]
    KeywordCrate,
    #[token("impl")]
    KeywordImpl,
    #[token("trait")]
    KeywordTrait,
    #[token("for")]
    KeywordFor,
    #[token("while")]
    KeywordWhile,
    #[token("loop")]
    KeywordLoop,
    #[token("break")]
    KeywordBreak,
    #[token("continue")]
    KeywordContinue,
    #[token("return")]
    KeywordReturn,
    #[token("as")]
    KeywordAs,
    #[token("const")]
    KeywordConst,
    #[token("static")]
    KeywordStatic,
    #[token("type")]
    KeywordType,
    #[token("where")]
    KeywordWhere,
    #[token("unsafe")]
    KeywordUnsafe,
    #[token("extern")]
    KeywordExtern,
    #[token("ref")]
    KeywordRef,
    #[token("self")]
    KeywordSelf,
    #[token("super")]
    KeywordSuper,
    #[token("in")]
    KeywordIn,
    #[token("move")]
    KeywordMove,
    #[token("dyn")]
    KeywordDyn,
    #[token("abstract")]
    KeywordAbstract,
    #[token("async")]
    KeywordAsync,
    #[token("await")]
    KeywordAwait,
    #[token("become")]
    KeywordBecome,
    #[token("box")]
    KeywordBox,
    #[token("do")]
    KeywordDo,
    #[token("final")]
    KeywordFinal,
    #[token("macro")]
    KeywordMacro,
    #[token("override")]
    KeywordOverride,
    #[token("priv")]
    KeywordPriv,
    #[token("typeof")]
    KeywordTypeof,
    #[token("unsized")]
    KeywordUnsized,
    #[token("virtual")]
    KeywordVirtual,
    #[token("yield")]
    KeywordYield,

    #[regex(r#""[^"]*""#)]
    String,
    #[regex(r"'([^'\\]|\\.)*'")]
    Char,
    #[regex(r"'")]
    Lifetime,
    #[regex(r"(=|\+|-|\*|/|%)")]
    Operator,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r"\d+")]
    Integer,
    #[regex(r"\d+\.\d+")]
    Float,
    #[regex(r"::")]
    DoubleColon,
    #[regex(r";")]
    Semicolon,
    #[regex(r"\{")]
    OpenBrace,
    #[regex(r"\}")]
    CloseBrace,
    #[regex(r"\(")]
    OpenParen,
    #[regex(r"\)")]
    CloseParen,
    #[regex(r"\[")]
    OpenBracket,
    #[regex(r"\]")]
    CloseBracket,
    #[regex(r",")]
    Comma,
    #[regex(r"#")]
    Hash,
    #[regex(r"\.")]
    Dot,
    #[regex(r":")]
    Colon,
    #[regex(r"\|")]
    Pipe,
    #[regex(r"<")]
    OpenAngle,
    #[regex(r">")]
    CloseAngle,
    #[regex(r"\^")]
    Caret,
    #[regex(r"&")]
    TempBorrow,
    #[regex(r"\?")]
    Question,
    #[regex(r"!")]
    MacroExclamation,
}

fn criterion_benchmark(c: &mut Criterion) {
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory")
        .join("data");

    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap_or_default() == "rs" {
                    files.push((
                        file_path.file_name().unwrap().to_str().unwrap().to_owned(),
                        std::fs::read_to_string(&file_path).expect("Unable to read file"),
                    ));
                }
            }
        }
    }

    c.bench_function(
        "Tokenize Yuumi vulkan game engine with logos",
        |b: &mut criterion::Bencher<'_>| {
            b.iter(|| {
                for (_, source) in files.iter() {
                    let mut lex = KindsRustLogos::lexer(source);
                    while let Some(Ok(res)) = lex.next() {
                        black_box(lex.slice());
                        black_box(res);
                    }
                }
            })
        },
    );

    let mut total_logos = 0;
    for (_, source) in files.iter() {
        let mut lex = KindsRustLogos::lexer(source);
        while let Some(token_result) = lex.next() {
            match token_result {
                Ok(KindsRustLogos::Whitespace)
                | Ok(KindsRustLogos::CommentLine)
                | Ok(KindsRustLogos::CommentBlock)
                | Ok(KindsRustLogos::Newline) => {}
                _ => {
                    total_logos += 1;
                }
            }
        }
    }
    println!("Total Logos tokens: {}", total_logos);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
