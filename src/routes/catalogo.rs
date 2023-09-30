use actix_web::*;

pub async fn catalogo() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(
            r#"[
              {
                  "nome": "Rust",
                  "criador": "Mozilla",
                  "ano": 2010
              },
              {
                  "nome": "Python",
                  "criador": "Guido van Rossum",
                  "ano": 1991
              },
              {
                  "nome": "JavaScript",
                  "criador": "Brendan Eich",
                  "ano": 1995
              },
              {
                  "nome": "Java",
                  "criador": "James Gosling",
                  "ano": 1995
              },
              {
                  "nome": "C++",
                  "criador": "Bjarne Stroustrup",
                  "ano": 1983
              },
              {
                  "nome": "Go",
                  "criador": "Robert Griesemer, Rob Pike, Ken Thompson",
                  "ano": 2009
              },
              {
                  "nome": "Swift",
                  "criador": "Apple Inc.",
                  "ano": 2014
              },
              {
                  "nome": "C#",
                  "criador": "Microsoft",
                  "ano": 2000
              },
              {
                  "nome": "Ruby",
                  "criador": "Yukihiro Matsumoto",
                  "ano": 1995
              },
              {
                  "nome": "PHP",
                  "criador": "Rasmus Lerdorf",
                  "ano": 1994
              },
              {
                  "nome": "Kotlin",
                  "criador": "JetBrains",
                  "ano": 2011
              },
              {
                  "nome": "Scala",
                  "criador": "Martin Odersky",
                  "ano": 2003
              },
              {
                  "nome": "TypeScript",
                  "criador": "Microsoft",
                  "ano": 2012
              },
              {
                  "nome": "Perl",
                  "criador": "Larry Wall",
                  "ano": 1987
              },
              {
                  "nome": "Haskell",
                  "criador": "Miranda Project",
                  "ano": 1990
              },
              {
                  "nome": "R",
                  "criador": "Ross Ihaka, Robert Gentleman",
                  "ano": 1993
              },
              {
                  "nome": "Dart",
                  "criador": "Google",
                  "ano": 2011
              },
              {
                  "nome": "Lua",
                  "criador": "TeCGraf, PUC-Rio",
                  "ano": 1993
              },
              {
                  "nome": "Swift",
                  "criador": "Apple Inc.",
                  "ano": 2014
              },
              {
                  "nome": "Racket",
                  "criador": "PLT Inc.",
                  "ano": 1995
              }
          ]
          
      "#,
        )
}
