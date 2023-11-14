extern crate reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;
use serde_json::Value;

struct Translator {
    client: reqwest::blocking::Client,
}

impl Translator {
    fn new() -> Translator {
        Translator {
            client: reqwest::blocking::Client::new(),
        }
    }
    fn from_japanese_to_english(&self, text: String) -> String {
        let params = HashMap::from([
            ("text", text.as_str()),
            ("glossary_id", "77192bc9-d2f7-4791-8788-75f603c362b9"),
            ("target_lang", "EN"),
            ("source_lang", "JA"),
        ]);
        let res = self.client
            .post("https://api-free.deepl.com/v2/translate")
            .header(
                AUTHORIZATION,
                "DeepL-Auth-Key ",
            )
            .header(
                CONTENT_TYPE,
                "application/json",
            )
            .form(&params)
            .send()
            .unwrap();
        let json: Value = res.json().unwrap();
        let res = json["translations"][0]["text"].clone().to_string();
        res[1..res.len() - 1].to_string()
    }
    fn from_english_to_japanese(&self, text: String) -> String {
        let params = HashMap::from([
            ("text", text.as_str()),
            ("glossary_id", "7986fe04-832d-48d9-bf03-0ec65356d55d"),
            ("target_lang", "JA"),
            ("source_lang", "EN"),
        ]);
        let res = self.client
            .post("https://api-free.deepl.com/v2/translate")
            .header(
                AUTHORIZATION,
                "DeepL-Auth-Key ",
            )
            .header(
                CONTENT_TYPE,
                "application/json",
            )
            .form(&params)
            .send()
            .unwrap();
        let json: Value = res.json().unwrap();
        let res = json["translations"][0]["text"].clone().to_string();
        res[1..res.len() - 1].to_string()
    }
    fn create_glossary(&self, entries: String, name: String, source_lang: String, target_lang: String) -> String {
        let params = HashMap::from([
            ("name", name),
            ("source_lang", source_lang),
            ("target_lang", target_lang),
            ("entries", entries),
            ("entries_format", "tsv".to_string()),
        ]);
        let res = self.client
            .post("https://api-free.deepl.com/v2/glossaries")
            .header(
                AUTHORIZATION,
                "DeepL-Auth-Key ",
            )
            .header(
                CONTENT_TYPE,
                "application/json",
            )
            .form(&params)
            .send()
            .unwrap();
        res.text().unwrap()
    }
}

#[test]
fn translate_test() {
    let t = Translator::new();
    // let res = t.from_japanese_to_english("道相乗りは間に合うことの方が多いし、間に合わなかったとしてもそれはつまり修道院や都市を引けているということなので、左上でいいかな".to_string());
    // let text = "If Red takes the empty city cap and scores 4 points, this will give us a platform to try and steal the whole city with a city cap + triple city tile.".to_string();
    let text = "修道院を返すことを優先させる。 右折引いた時のメリットがかなり大きいのと、次手でカーブで9点草原が見える。 もしも先に相手に6点草原寝られたら、T字路で草原を分離狙いたい。".to_string();

    println!("Original Sentences\n\"{:}\"", text);

    let res = t.from_japanese_to_english(text);

    println!("Translation\n\"{:}\"", res);
    assert!(false);
}

#[test]
fn create_glossary() {
    let t = Translator::new();
    let res = t.create_glossary("道修\tmonastery with road\nリップ\tcity cap\n道付三辺\ttriple city with road\n草原\tfield\n三辺\ttriple city\n四辺\tquadruple city\n妨害する\tto obstruct\n寝る\tto drop a farmer\n相乗り\tinvasion\n相乗りする\tto invade\n二辺\ttriangle\n草原に寝る\tto drop a farmer\nマナカナ\tdiagonal splitter\n右折\tright dagger\nT字\tT-shaped crossroads\n引く\tto draw".to_string(), "new glossary".to_string(), "ja".to_string(), "en".to_string());
    // let res = t.create_glossary("city cap\tリップ\nplatform\t足場\nto steal\t乗っ取る\ntriple city\t三辺".to_string(), "english to japanese glossary".to_string(), "en".to_string(), "ja".to_string());
    println!("res = {:?}", res);
    assert!(false);
}
