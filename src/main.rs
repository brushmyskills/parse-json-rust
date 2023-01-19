use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Paragraph{
    name: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Article{
    name:String,
    author:String,
    paragraphs : Vec<Paragraph>
}

fn main() {

   let json = r#"{
          "name": "My Article Details",
          "author": "AKS",
          "paragraphs": [
              {
                  "name" : "paragraph heading"
              },
              {
                "name" : "paragraph Body"
            },
            {
                "name" : "paragraph End"
            }
          ]
   }"#;

   let parsed_articles = json_data_parse_article(json);

    println!("Final Parse Data Body is: {}",parsed_articles.paragraphs[1].name);
}

fn json_data_parse_article(raw_json: &str) -> Article{

   let parsed : Article = serde_json::from_str(raw_json).unwrap();
   return parsed

}
