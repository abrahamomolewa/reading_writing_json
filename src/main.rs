use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
 struct Paragraph {
    name: String,
 }

#[derive(Serialize, Deserialize)]

struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article = Article {
        article: String::from("How to work with json in rust"),
        author: String::from("abraham"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence")
            },
            Paragraph {
                name: String::from("body of the paragraph"),
            },
            Paragraph {
                name: String::from("end of the paragraph"),
            }

        ]
    };
    
    let json =  r#"
    {
      "article": "how to work with json in Rust",
      "author": "abraham",
      "paragraph": [
        {
            "name": "starting sentences"
          },
            {
               "name": "body of the  paragraph"
            },
            {
                "name": "end of the paragraph"
            }



        ]

         
    }"#;

      
    let parsed: Article = serde_json::from_str(json).unwrap();
    println!("\n\n the name of the second paragraph is {} \n", parsed.paragraph[1].name);

    let json1 =  serde_json::to_string(&article).unwrap();
    println!("the josn is: {}", json1);
}