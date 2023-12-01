extern crate reqwest;
use dotenvy::dotenv;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

use lingua::Language::{English, Japanese};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub struct Translator {
    client: reqwest::blocking::Client,
    auth_key: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lang {
    Japanese,
    English,
    Unknown,
}

impl Translator {
    pub fn new() -> Translator {
        dotenv().ok();
        Translator {
            client: reqwest::blocking::Client::new(),
            auth_key: env::var("DEEPL_AUTH_KEY").expect("DEEPL_AUTH_KEY must be set"),
        }
    }
    pub fn translate(&self, text: String, src_lang: Lang) -> String {
        let glossary_id = if src_lang == Lang::Japanese {
            "42a3f923-c6cd-4a56-83fb-c1c8733d81c8"
        } else {
            "8b7494b3-70c9-4314-a7cd-89b3fc0140b4"
        };
        let target_lang = if src_lang == Lang::Japanese {
            "EN"
        } else {
            "JA"
        };
        let source_lang = if src_lang == Lang::Japanese {
            "JA"
        } else {
            "EN"
        };
        let params = HashMap::from([
            ("text", text.as_str()),
            ("glossary_id", glossary_id),
            ("target_lang", target_lang),
            ("source_lang", source_lang),
        ]);
        let res = self
            .client
            .post("https://api-free.deepl.com/v2/translate")
            .header(AUTHORIZATION, self.auth_key.clone())
            .header(CONTENT_TYPE, "application/json")
            .form(&params)
            .send()
            .unwrap();
        let json: Value = res.json().unwrap();
        let res = json["translations"][0]["text"].clone().to_string();
        res[1..res.len() - 1].to_string()
    }
    #[allow(dead_code)]
    fn create_glossary(
        &self,
        entries: String,
        name: String,
        source_lang: String,
        target_lang: String,
    ) -> String {
        let params = HashMap::from([
            ("name", name),
            ("source_lang", source_lang),
            ("target_lang", target_lang),
            ("entries", entries),
            ("entries_format", "tsv".to_string()),
        ]);
        let res = self
            .client
            .post("https://api-free.deepl.com/v2/glossaries")
            .header(AUTHORIZATION, self.auth_key.clone())
            .header(CONTENT_TYPE, "application/json")
            .form(&params)
            .send()
            .unwrap();
        res.text().unwrap()
    }
    pub fn detect_language(&self, text: String) -> Lang {
        let languages = vec![English, Japanese];
        let detector: LanguageDetector =
            LanguageDetectorBuilder::from_languages(&languages).build();
        let detected_language: Option<Language> = detector.detect_language_of(text);
        match detected_language {
            Some(English) => Lang::English,
            Some(Japanese) => Lang::Japanese,
            _ => Lang::Unknown,
        }
    }
}

#[test]
fn create_japanese_to_english_glossary() {
    let t = Translator::new();
    let glossary: Vec<String> = vec![
        ("道修", "monastery with road"),
        ("みちしゅう", "monastery with road"),
        ("リップ", "city cap"),
        ("草原", "field"),
        ("ゴージャス", "quadruple city"),
        ("妨害する", "to obstruct"),
        ("寝る", "to drop a farmer"),
        ("草原に寝る", "to drop a farmer"),
        ("マナカナ", "diagonal splitter"),
        ("まなかな", "diagonal splitter"),
        ("右折", "right-handed dagger"),
        ("左折", "left-handed dagger"),
        ("引く", "to draw"),
        ("刺す", "to stab"),
        ("差す", "to stab"),
        ("挿す", "to stab"),
        ("さす", "to stab"),
        ("足場", "platform"),
        ("足掛かり", "platform"),
        ("足がかり", "platform"),
        ("アンダーバー", "starting tile"),
        ("アンダー", "starting tile"),
        ("土管", "extender"),
        ("どかん", "extender"),
        ("ドカン", "extender"),
        ("オスピー", "vertical splitter"),
        ("オスピ", "vertical splitter"),
        ("おすぴー", "vertical splitter"),
        ("平行二辺", "vertical splitter"),
        ("分断都市", "splitter"),
        ("T字路", "T-shaped crossroads"),
        ("T字", "T-shaped crossroads"),
        ("ティー", "T"),
        ("道", "road"),
        ("相乗り", "invasion"),
        ("あいのり", "invasion"),
        ("相乗りする", "to invade"),
        ("あいのりする", "to invade"),
        ("相乗る", "to invade"),
        ("あいのる", "to invade"),
        ("相乗り場", "invasion platform"),
        ("布石", "preparation"),
        ("駒", "meeple"),
        ("体", "meeple"),
        ("こま", "meeple"),
        ("コマ", "meeple"),
        ("フリー", "vacant"),
        ("先手", "first player"),
        ("後手", "second player"),
        ("紋章", "coat of arms"),
        ("点", "point"),
        ("固定", "fixing"),
        ("回収する", "to retrieve"),
        ("回収", "retrieval"),
        ("十字路", "quadruple road"),
        ("十字", "quadruple road"),
        ("系", "type"),
        ("ラス", "last"),
        ("被る", "to duplicate"),
        ("かぶる", "to duplicate"),
        ("ダブる", "to duplicate"),
    ]
    .into_iter()
    .map(|pair| format!("{}\t{}", pair.0, pair.1))
    .collect();

    let res = t.create_glossary(
        glossary.join("\n"),
        "japanese to english glossary".to_string(),
        "ja".to_string(),
        "en".to_string(),
    );
    println!("res = {:?}", res);

    assert!(false);
}

#[test]
fn detect_language_test() {
    let t = Translator::new();
    assert_eq!(
        t.detect_language("Depends on who were' playing - if we're playing a higher rated player, it is OK and maybe even better to go for the 50-50% gamble.".to_string()),
        Lang::English,
    );
    assert_eq!(
        t.detect_language("楽しいのでおきます。".to_string()),
        Lang::Japanese,
    );
    assert_eq!(
        t.detect_language("紋章都市最強。".to_string()),
        Lang::Japanese,
    );
    assert_eq!(
        t.detect_language("painful move for me :D".to_string()),
        Lang::English,
    );
    assert_eq!(t.detect_language("(・∀・；)".to_string()), Lang::Unknown);
}

#[test]
fn japanese_translation_test() {
    let t = Translator::new();

    for note in [
        // problem 1
        "土管とか平行二辺を引かれてスタートダッシュ極められるのが一番嫌。でも、この前、某死にイベにこの形をやったらストレートリップで道を持っていかれるというスタートダッシュ極められて、そのまま一度も追いつかなかったわ。",
        "カーブなら道を上に向けるけど、ティーは１点の道に乗る勇気がないので草が上。",
        "都市の完成を少しでも遅らせるためにこの形。でも多重建設の恐れもあるから微妙？",
        "相手の都市完成を妨害しつつ自分は道の需要を作るために終端のある道に乗る",
        "序盤は土管を置かせない",
        "相手の妨害の足掛かりと道でグルリと円を描いて4点ゲットの為の布石として。",
        "ドカンとかオスピ引かれるのが嫌すぎるから道刺したい\n",
        "相手の都市にプレッシャーをかけながら、じぶんの道を伸ばしていきたい。\n左上の道へ伸ばしていくよりも下の道とのループを狙いたいからこの向き",
        "T字リップが怖いけどマナカナよりはマシなので道差しながらフリーの終端付き道を取る。\n\nこの後カーブを引いたら右の道に付けてリップを置く。\n道伸ばしながらスタートタイル固定か、都市カーブ系のやつ引かれたらループ作る。\n\nスタートタイルか道付き三辺引かれたら辛いのと、道の需要が多くなるのが難点。",
        "終点付きなのでストレートの形が強いと信じてる。\nあいのり箇所を増やすより、相手の都市に逃げられる可能性が高そう。",
        "シンプルに道を挿しながら道に置きます。",
        "大して強くない妨害だけど他もパッとしないから仕方なくここ。\n\n土管オスピーまなかなの絶望を緩和。\n四辺で先後交代。\nT字リップは最悪だが2点損する程度。\n道つき三辺は嫌だが相乗りの余地がある。\n自分が道つき都市を引いた時に相乗りできなくて悲しくなるが、そのタイルは上側から破壊の布石としても強く使える。",
        "赤の都市に道を刺したい所だが、序盤はほぼほぼ完成されてしまうため、完成した時に追加で終点のある道2点をあげてしまうリスクを嫌ってこの位置。\n土管・オスピーケアは出来ていないが、土管・オスピー6枚に対し、道付きで引かれたくない都市は16枚(道付き2辺・3辺含む)なので確率的に道ケア。\n\n向きについては4点ループが狙える、次に道付き都市タイルでも道無し都市タイルでも相乗りに行けるため、この向きに配置。",
        "道を刺すと道付の都市系タイルを有効的に使われてしまうため、道は刺さず。かと言って8点都市放置は有り得ないのでこの形に。",
        "道無しの都市、特に分断都市や土管形の都市を引かれて強く使われるのが嫌過ぎるので、とりあえず道は刺しておきたい",
        "序盤は相手の都市に道を刺しても完成されがちで、その場合は道の終端を相手に渡す結果になる。それなら妨害等の足場をもう一つ作るだけに留めておくのも良いかもしれない。",

        /*
        // problem 2
        "これ置くとこないなー。2点取りつつ、相手が修道院を破壊するためには都市を使うことになりつつ、フリーリップを取ると修道院の防御になりつつということで！",
        "フリーのリップが残る、相手の道が帰りやすくなる…悲しいことだらけだけど他がシャバすぎるので。T字リップがここまで悲しい盤面よく作れたなーと思いました。",
        "右下の都市に付けたいところだけどもう少しタイルを吐き出して種類が絞られるまでは手出し出来ないから5点以上の可能性がある道に置いて相手の動きを観る！",
        "消去法的な感じする\n2つ上のところも考えたけど即道5点放置は無いし、道置くならこっちの方が強い多分\nその他は自分にデメリット多すぎるからここかなぁ",
        "0 点行動だけど、割と勝っている状況なので相手に即座に点をとられる地形を残さないようにここでワントップ。相手は別にミープル数きつくないからこれで回収できてもそんなには嬉しくないはず。",
        "赤のリップ完成しないようにケアしたい",
        "他は破壊されやすくなるか相手に終端を使われるかなのでここ。\n都市を取られるよりカーブで6点取られる方が損なので道。\nフリーリップを取った/取られた時に相手の都市を自動的に妨害できる向き。\n二辺三辺で吸収されても相手は残り少なめな二辺待ちが多くなるし、取られたところで2点の損害なので妥協。\n他にも長い道など優先すべき地形が増えることで間接的に自分の都市と修道院を守れている。",
        "先にカーブ引かれたら終端付きの長い道取られるのであまりやりたくないが仕方なく…\n\n一応相手の気持ち的には道需要被ったり、まとめて妨害される可能性あったりで、ここの道取るのは結構怖さはあります…",
        "相乗りの布石に。赤の完成を遅らせることが出来れば。\n",
        "匂いが・・・匂いがするんだよ、草原のな！",
        "21点勝ちで破壊されている駒もないので即４点、５点といった箇所を作ることなく点差を維持。次に道付き修道院を引かれるのが少し怖い。",
        "一番安全そうな場所にリップ辺を置く",
        "道の需要が作りたいのでこの位置。\n赤に終点付きの道2点を取られそうだがT字路で奪うことも出来るので許容。\nカーブで都市に向けられても、自分の都市回収が出来ない左折の使い道ができるので問題無し。相手が道を下に向けてくると相手のワントップが崩れるのもメリット。\n修道院の左下は修道院スポットとして残しておきたい。",
        "相手が単辺都市を引いた場合、自分の都市を完成する方がいいのと、二辺都市結構出たから。",
        "赤の都市への妨害の足場を作りながらワントップという素直な手だけど、黄色が割と勝っている状況なので相手に運良くカーブ系 2 枚とストレート系で効率良く長い道を完成されるというシナリオがかなり嫌で迷った。でも他に良い手もないからこれで良いかな",

        // problem 3
        "相手の都市を破壊したいけど、紋章付き都市あげちゃうことになるし、その後、相手の捨て場として有用になってしまうのでやめとく。\nここは、都市完成を目指すも良し。捨て場にして相手の道を完成しにくくするも良し。",
        "点数あげるのは悔しいけど中途半端に肥大化して完成されるのが1番悔しい。",
        "3辺都市なので、破壊したとしても都市を広げる余地ができてしまう。それよりは自分の都市を広げた方がよいと思うのでこの配置。相手が道を延ばすとこちらにも点が入る形になるのも良き。",
        "下の三角の右において完成目指すのもあるかなって思ったけど、結構相乗りできるしなら破壊の方が優先度高いよなあって感じです\nただでかいゴミ捨て場になるのも嫌だなぁって感じです",
        "右上の都市は破壊しておく。",
        "最低１２点完成はキツイので破壊。と同時に相手のミープル2体を1体のミープルで足止めできる。\n",
        "潰せるときに潰す",
        "12点都市完成阻止が最優先。後手で３点負けだが駒数優位にできればまだまだ逆転可能。",
        "匂いが・・・匂いがするんだよ、都市のな！",
        "一手前で肥大化したからには破壊する。\nどのみち相乗りはしにくい。\n12点都市の完成されるぐらいなら2点あげたほうがリスクも少ない。\n1番最悪なのは完成されて道修スポットからの即8点行動されると目も当てられない。\nこの先はゴミ捨て場としてどんどん大きくしていくようであればタイミング見て相乗り。",
        "赤の7点を取る機会+ミープルを返らせるのを阻止しましょう、下の３点を取るよりはメリットが大きいはず 、そして赤は２４枚の都市タイルが８点修道院のスポットを作れるのて五分五分の運ゲーを避けたいならこの手しかないです。",
        "破壊とすごく迷ったが、このままだと相手が道引いた時がかなり強いのでこれで。\n\nこの後自分の引くタイルが\n道系→上の終端取りながら相乗りの布石\n二辺三辺系→下の都市を作る\nリップ系→たぶんなんかしら使い道ある\n\nと受けが広いのがメリット！\n\nとは言え右上の都市完成はやっぱり怖いからこの後は全力で阻止しよう！",
        "完成確率高めの都市を1手で破壊できるのはうれしい",
        "紋章付き都市を自分の都市につけず相手の都市につけると実質 -4 点で悩ましいのだけど右上の都市は早めにリップ引かれて左から閉じられて比較的安全に完成を狙われると厳しいのでここは破壊しておいて中盤以降ミープル差を活かせる試合展開に持ち込みたい。",

        // problem 4
        "基本は完全を目指すムーブ。ミープル差があるので、引き次第では相手のリップを巻き込んで死ぬという選択肢もある。",
        "ここのミープルちゃんがリップで帰ってくる気配がしないので、三角で作って返そうの方針。\n右のカーブの左に置いて土管を守るのも考えたけどちょっと空中戦過ぎるかな。\n後は赤のリップ肥大化も考えたけど、さらに自分の三角が帰って来なくなりそう…",
        "そんなに強くはないけどここ。相手の都市妨害も視野に入れながら二辺三辺の置き場を作り、展開に応じて都市完成を目指す。そんなに強くはない。",
        "都市の相乗りも考えたが完成しないのでそれよりは小さくても完成出来る都市を作る。赤の一辺都市の妨害の可能性も視野に入れて。",
        "匂いが・・・匂いがするんだよ、ここに置いてと！",
        "右下の二辺はリップで完成させようとするとどちらから置いても残り2枚しかない左折リップ固定されそうなので、二辺三辺を使って完成させたくてここに！\n最高の結果は二辺4枚三辺2枚の都市の完成だが、妨害のために相手に何手か使ってもらった上で6点くらいの都市まで成長できればそんなに悪くないし、相乗りされてたとしてもミープル差で相手が厳しくなるはずです！\n",
        "他の手が思い浮かばない",
        "右側の赤のリップ辺とこの黄色の都市をつなぎにかかる。\n赤はミープル2つが破壊されているので、この両都市が繋がって完成しない展開になると残ミープル数の差で自然によくなるはず。",
        "右辺の肥大を行うのは道付きリップで自都市が死ぬため防御のみ",
        "相手の草原牽制、修道院完成に無駄な都市タイルを使わせる、修道院連接阻止、の意味からここ。\nこれ僕の試合なんですが、この3辺の正解は全くわからんです笑",
        "都市の破壊を狙う。もともと完成がもたつきそうな都市なので肥大化してOK。\n道を守るか迷ったが、十字路固定されるタイルはリップ回収との選択になるし、問題は道つき二辺くらいだから放置でいいかなと。",
        "左折される前に早めに置こう、赤の都市を邪魔できるし、アンダーバー T字リップ 左折 リップ 土管 オスピーを引いても意外とブロックされにくくなるよ、４辺都市の置き場を作るという意味でもね",
        "右の自分の都市を建築する素直な手と迷ったけどこちらにしてみた。\n黄色の交差点固定になっているところの下に道付き二辺 4 枚と道付き三辺 2 枚のいずれかを置かれてここの解決に T 字リップを使わされる or ラスト 2 枚が引けないの展開になる確率を下げた。ここはまだ 4 枚ある T 字路で解決して T 字リップは他で強く使いたい。右の自分の都市は二辺都市 3 枚で建築を目指す。",

        // problem 5
        "自分で狭くしてしまっているけど、相乗り防止と完成と同時に道も取れるのが良い。",
        "強いかは知らん！！楽しそう！！！\n上から破壊されるのがいやなのは重々承知だが都市当てないと行けないので割と守れる可能性も高い多分\n序盤だしリスク取ってもよし！！",
        "相手の相乗りがこちらの道の得点になるとは言え12点都市完成の確率を少しでも上げたい。一つ下に置いてアンダーバー＆T字リップ待ちにする手は二手で破壊されそうで少し怖い。これなら道付き三辺で肥大化されても悪くない形。",
        "相乗りリスクを最低に",
        "確実に2点を取りながら黄色の道に終端をつける。",
        "都市の完成目指したい。バー付き、Т字固定にすると、右に置かれて2体トラップの可能性があるのでこの形。",
        "2点取って道の終点もつける。序盤の都市防御はタイルの残りを考えると強くない",
        "1.相乗りされないために（道付き２辺、道付き３辺、アンダーバー、T字リップ、右折合計15枚タイル）2.上のカーブを回収できる 3.都市完成に付き道ボーナス（3枚だけど) ",
        "損するのは左折を引いたときだけ。両サイドが道の都市はかなり破壊されづらいと思っているけど、どうだろう。",
        "1番辛そうなのが、赤に道付き3辺都市を右折の右に付けられること。なのでそれを防ぐ形。\n草原を右に向けて、草原がないタイルでは上からでも下からでも破壊の足がかりに出来ないようにする。\n右折は1枚引かれてはいるけど、この状況で都市を2辺都市が何かで上に向けてくることはないと思うので、下の都市完成を待ってから道の回収がしたい。\n土管の下はストレートの置き場として残しておく。",
        "匂いが・・・匂いがするんだよ、大量点のな！",
        "左折リップ固定回避、ループ狙いでここ！\n下の道はストレート受けに！\nスタートタイルやT字リップ引けたら道の点数ももらえて嬉しい！",
        "都市が破壊されるリスクに対してある程度上手く引けたときのメリットが大きいと思う。相手は草原無しのタイル 12 枚ではこの周辺は触れないので破壊が遅れることもある。左折を引いて都市完成できないパターンは悲しいけど左折は右上の道完成や左の道伸ばしながらの都市予約などで十分強いと思う。",

        // problem 6
        "相乗りを防いでおけば完成するやろ！",
        "ちょっと悩むが素直に都市完成への受け入れが広いほうを選択",
        "相乗りより破壊の方がいや🥹\nこっちの方が道付き三角受けやすいし何とかなんべ\n相乗りされるのはしゃーなし",
        "1/3くらいで相手の手を無効にできるのであればあり？",
        "道無し都市タイル(リップ、土管、オスピー、マナカナ)を全部カツアゲする一手。修道院引かれて除外されると先手有利無くなるけど。\n\nと、いうのが狙いなんじゃないか？って深読みしました。\n多分しっかり勝ちに行くならストレートの上だと思います。笑",
        "この都市が破壊される場合は相手は都市タイルを浪費するので、破壊はあまり恐れなくて良い。一手でアドバンテージが消える相乗りの方をケアする一手。\n本当はストレートの下に置いて土管オスピーでドヤ顔したかった。",
        "昔で遭遇した盤面、あの時は真ん中の下で都市を置いた。理由としては２１枚の道無し都市を相手に強制置かせるのと４枚の修道院を置かせないのは狙いでしたけど、でも逆にこの21枚以外を引かれると破壊されやすい+上下の都市が同時完成しにくい+先手から後手になる可能性などのデメリット。\n\n３種類の置け方のパターンを全部比較してみたんだけど、残り６６枚タイルの中で感覚的に自分側が有利の展開になるのは→２1枚（下置き）、7枚（右置き）と３8枚（上置き）という自分なりの結論に至りました。上の都市の完成率を少しでも上げた方が強いかなと",
        "先手なので相乗りされたとしてもイーブンくらい。完成しないと勝ち目がない状況でないのなら破壊されにくい形で都市完成を目指したい。また道付きの相乗りタイルはやや少ない。",
        "相乗り防止を優先してこちらに！\n\n赤に道伸ばしながら妨害の布石を作られるので破壊されやすいが、完全に破壊しきるためにはタイルの中では価値の高めなリップを1つ献上してもらうことになるか、二辺三辺での破壊だと、余った二辺三辺の受け皿になって点数伸ばせるかなのでまあ許容できるかなと…\n\n完成できたら大きい！！",
        "匂いが・・・匂いがするんだよ、できない都市のな！",
        "いつもなら四辺の上に都市を右に向けて置くけど、今回はこちらも強そうと思いこちらで。\n左上からの相乗りは、左折を除く任意の道付き都市 16 枚でできるが、割と少なめなので先に四辺の上の建築を狙いたい。左上からの妨害は、草原を向けられた場合はそもそも最初に四辺の上に都市を右に向けて置いた場合と似た形に帰着されるので構わなくて、ミープル置かず道を向けられた場合は終端つきの道をとれる可能性があるしミープル置きの道向けは都市相乗りをされにくくなって OK。\n右側に草原で相乗り場を作られた場合が結構嫌で、だいぶ建築が難しくなるのだが、それがまともにできるタイルはカーブと修道院系で、修道院をここに置いて相乗り場とするのはあまり強くない気がする。右側に都市で相乗り場を作られて相乗りされる場合もあるかもしれないがわざわざ都市の二手かけて相乗りしてくれるならあまり気にならないし、下の赤の道も狭くなるとなお嬉しい。",

        // problem 7
        "通常、現状6点しかない都市に相乗りすることはしないのだけれど、他にすることがない。仕方なし。",
        "相乗りに行ってフリーのリップ辺を取られるのが嫌。\nこの配置なら、もし赤にリップ辺を取られても道が1点伸びる。\n",
        "自分の道がキツくならないように、妨害もしやすい場所からの相乗りにしました。\nまああまり美味しい都市では無いのが辛いですが…",
        "ワントップ！ 左の都市の邪魔もできるし、放置してる方のリップは取られても今相手が置いてるリップの妨害になる！！強い！！！",
        "８点都市を妨害したいが有効打が無い。相乗りは右の都市完成や上のリップから二辺で乗っ取りもある。次に８点都市完成なら右の都市を妨害しながら草原寝で草原戦を目指す。",
        "乗っ取りを狙ってまず1体目の相乗り。\n相手がリップを引いて上の都市をとった場合、手番順的に乗っ取り確率は自分のほうが高い。相手が二辺か三辺を引いた場合は自分も二辺か三辺で逃げればよく、分の良い勝負になる。運悪く都市を乗っ取られても、損したのは4点（自分のリップ2点が相手の2点に）程度であり許容できるリスク。",
        "ワントップつくりなが、もうひとつの余ったリップは道で予約！\n道の左上に置くのも考えたが、スタートタイルに何かタイル置かれると、片方の道を狭めた上でスタートタイル固定の布石を作られるので、妨害の布石打とうとすると赤自身のリップの邪魔になってやり辛いこちらを選ぶました！\n\n下のリップより右のリップ＆道を優先して置きたいところ…",
        "匂いが・・・匂いがするんだよ、つらい匂いがな！",
        "パッと見て全然分からないけど、実際4種類の置け方の全パターンを並べてみたら、意外と答えが見つかった。自分の結論としては→２1枚道路+６枚修道院タイルはこの局面に特に影響しない。残された40枚の都市タイル全部こっちの置け方が強いと思う、主な理由は３つあるけど、「相手に都市完成+寝るという行動をさせてはいけない」と「相手が単辺リップを引いた時、自分の都市を回収する方が得→自分がマナカナに都市を置く必要性あんまりない」と「相手が２辺都市もしくは3辺都市の引いた時、リップに繋がらせない置き方」ってのはポイントだと思う、この盤面を学んだわ。",
        "どこに置いてもパッとしないのでここ。\nここに置く理由というよりも他の手と比べた時の消去法。\n出来れば2辺都市を先に引いてフリーのリップ辺を取って乗っ取り狙いたい。",
        "回収 5 点行動をさせないという手。弱気めではあるけどどう置いてもどこかに空きリップは残るので仕方がなくて、これを相手が予約してくる場合は右の予約済みの都市の破壊を狙いたい。",

        // problem 8
        "この形になってしまったなら、12〜14点都市は仕方ない。せめて防御される時に都市をもらう。",
        "相手の防御が先かこちらの封殺が先か……実に楽しみ！よしんば相手の防御が成功し都市が完成したとしても12〜14点分の得点を他で作れば良し！どれだけ相手のミープルを釘付けに出来るか？少しでも相手の気を都市の防御・完成に割かす事ができるか。の為にコチラからワントップで仕掛けるッ!!!!",
        "たぶん防御されるけど一応ワンチャンにかけて破壊の布石とワントップつくる…\n\n下向けるのも考えたけど、弱防御された後、破壊できる前に三辺引かれると、逆に自分のリップが破壊される危険があって目も当てられないからまだこっちの方が安全かなと…",
        "ワントップ欲しい あそこにちょっかいかけて帰ってくるの遅らせたい ゴージャスは今じゃなくてもなんとかなる！",
        "相手に防御で一手使ってくれればラッキー程度に触りに行きたい",
        "匂いが・・・匂いがするんだよ、死ぬけど－1点で済むな！",
        "肥大化させた都市はまだ完成まで遠いので相乗りはもっといらないタイルで行いたい。\n左の都市は、次手で草原無しタイルやオスピー・土管を相手が引いた時にゴージャス固定で破壊できるのが強いのでこの位置。\nまあここは完成されても仕方ないかぐらいの気概でいきたい。",
        "後手で10点以上の点差をつけられるのは辛い。ここは共倒れでもいいので阻止したい。武は悪いがこの向き以外だと容易に守られる。修道院で守られたら最悪。ここは勝負手でいきたい。",
        "一応破壊チャレンジはする。守らせて完成タイルを半減させ回収速度を遅らせるだけでも十分。",
        "４辺都市出たので破壊される圧力は大きい、ポイントを取りながらブロックできる足場を作る。修道院と道で守られるなら得点ミスとミープルの回転率が下がる、都市リップなら先手で邪魔できる、２辺都市とマナカナなら下の道を巻き込まれる、道付き都市（13枚）、土管、オスピーと十字路なら破壊できるチャンスがあるくらいかな。上置きだと守られるタイル１枚少ない＆破壊できるタイル1枚多いのでこっちでいいと思う",
        "中都市をcccc待ちで破壊する布石のワントップ",
        "相手に3辺とか2辺刺されて破壊されやすくなるので念の為守りたい気がします！",
        "中都市の完成を防ぎたい。斜めに分断する都市で自分だけ逃げながら破壊か三辺系で自分だけ死んだ都市を伸ばせる形が理想だけど、二辺系で 2 体ずつの心中でも構わない。相手の欲しいタイルが 6 枚で自分の欲しいタイルが 17 枚、向かい側からミープル置き都市を向けられることは相手としては割とリスクなので多分なくて、道刺しか草原刺しをされがちで、その場合こちらのタイルは実質 9 枚か 8 枚にされるがそれでも先に引ける確率が十分ある。この中都市完成でゲームが決まるわけではないので場合によってはこの中都市は諦めて回収 3 点行動でラスト 3 枚などにして相手の駒回転を下げる方針に切り替える。左折が一枚出ているのでこちらの向きから、上からだとストレートを向かい側に置かれて下の道のミープルも巻き込まれて左折固定で死にかねないので、道が狭くなるとしてもこちらからで良いと思う。",

        // problem 9
        "三差路、枯れるの早いー",
        "例えT字路の形で阻まれようとも残りのT字リップを全て引き切る気概を持って臨むべしッ!!",
        "T字系が残り少ないのでループ狙わずに即2点で！\n一応相手のミープル1つなのでカーブ置き即完成の場所はつくらないように",
        "道回収や道修建築時に寝られないようにケア。相手が草原に入る手間が0手番分のところが1,2手番余計にかかるようになり、自分の手間は1手番浮く。差し引き2,3手番分得するなら道2点よりも価値がありそう。他にも道つき三辺を将来的に強く使えるようになる。6点ループも自分にとって有利な地形(相手が取るならカーブ待ちがかぶるし道修回収が遅れる)。",
        "駒数有利だが得点は２点負け。次に赤の道を回収されながら草原に寝る手が強い。見えているだけで15点見込みで更に伸びる。ここは先手を打ちたい。",
        "匂いが・・・匂いがするんだよ、安いwifiじゃつながらない道の匂いがな！",
        "交差点タイルが残り2枚だが、相手もコマ1個で妨害にあまり手数を使えないはずなのでここ。\nこちらはコマが結構あるので色々と高得点のポイントは押さえておきたいかな。\n相手の、カーブで回収+スリープが強いけど、結局コマが1個のままだからリードできそう。",
        "赤残りミープル１個、3点よりは自分の道を回収するほうが得な気がする、右下だと６点の得点差ちょっと大きいかな、完成寝はされるか、寝るスポットは左上２個と右上１個と右下１個（T字リップならなし）って結構多いので右上のストレートの上で寝るのはまだしなくていいんじゃないかね。",
        "実際のゲームでこのように置いてみたという手。相手がカーブ系を引いて道のミープルを回収しながら 9 点草原に先に着地するという手がかなり強いと思ったので今のうちに寝ておいた。強めの空いた道を建築しているが、 T 字路はもうなくて道付き修道院は相手は置き場が既に左にあるので相手にこの道で即点数をとられる可能性は低く、ミープル差もあり、相手は既にカーブの需要がある、の理由からここは自分が予約して 6 点をとれる確率の方が高いと考えた。",

        // problem 10
        "ミープルなんて手元になんぼあっても困りませんから！",
        "駒が少ないうえに戻りにくい駒が多い！加えて盤面下の都市と修道院を巻き込んで殺されたら目も当てられない！故に巻き込まれ防止として下の都市に付けて完成を目指すべし！",
        "ややミープル数が怪しいので回収します。\n\nオスピーや土管は下の都市を受けにします。",
        "相手が2辺や3辺で、黄色の都市を上向きに曲げて巻き込まれると厳しいのでこの形。\n修道院が破壊されるのは仕方ないが、赤の道も回収に時間がかかれば実質コマ数差無しなのでよしとする。",
        "紋章付き三辺都市の右側に二辺三辺などで都市上向きに付けられるのが怖い。道付き修道院と合わせて２個駒が帰ってこなくなる可能性がある。",
        "匂いが・・・匂いがするんだよ、たくさんの死のな！",
        "ミープル2個しかないし、こっちは一般的な手。下の都市に置きたいなら一番下の方がいいと思う、相乗りされる確率低いので。道付き３辺の右に置くと、相手が１３枚の道付き都市タイルが相乗りできる、相手がカーブなら道路脱出+相乗り場作る+修道院を破壊するチャンス、ストレートだと土管の下に置いて邪魔する+相乗り場作る+左折にする。",
        "道修はいいけど都市が死ぬのは避ける",
        "道付き修道院の右のマスに右から道を刺されたあと下の都市を肥大化されてミープルが 3:1 で死ぬという展開を避ける手。この道付き修道院の右のマスは何で埋めてもあまり強くないしもう死んだものと見て、ここを一手かけて破壊してくるならそれはそれで良いと思う。でも素直にワントップ作ったり回収したりする手の方が強いんじゃないかとも思う。",

        // problem 11 - 20
        "めっちゃ勝ってる。勢いで相手の都市破壊！",
        "相手の都市完成を阻む手掛かりに置いて様子見！相手が防御するも良し、上手く相手の都市完成を阻む事が出来れば尚良し！",
        "点数かなり勝ってるので右折リップ引かれてやスタートタイルなど引かれて、修道院完成しながら都市取られる最悪のパターンを阻止する！\n幸い道も都市もまだ塞がってないので道や二辺三辺の受けにできるし、ミープル残り数も多いので、ミープル2対1で損してるけどそんなにきつくはなさそう！\nあわよくば都市完成大量得点ができたら嬉しい！！",
        "大幅にリードしているので、相手の高得点場を潰す方向で。草原侵入ポイントを２つにして、相手が草原に侵入してきたら取り返せるようにする。",
        "次に赤の都市破壊を狙う。これを守るタイルは赤の修道院の右上に置きたいタイルとも被るので効果的。二つ下で弱防御として置けるタイルが少ないのも良い。",
        "点差を考えると相手の得点源を先に取っておく。",
        "これ以上破壊に適したタイルはないのでここ。防御タイルと道修の右上に置けるタイルが被るため10点都市へのケアにもなっている。\nストレートの右で捨てるかかなり迷った。修道院を完成されて道だけ取り残されないように実質的に終端を作りつつ10点をケアしようかと。でもこれはこれで大都市バトルに発展して面倒な展開が見えるのでボツにした。",
        "破壊は絶対早めにやったほうがいい、じゃないと左折の引く確率が3/52...3/50...3/48...という形で後半に行けば行くほどどんどん上がってくる。紋章付き都市の価値は高いので、相手がT字リップ、アンダーバーと道付き２辺で「建築しながら都市を守る」という可能性を消すためだけにミープルを置く。一応置かないメリットもある、例えば相手がカーブで邪魔してくると自分がコマを使わず道付き２辺と道付き３辺で破壊することができる、もう一つとしては相手が道なし２辺、３辺、４辺で巻き込まれる心配もなくなる。なので自分が紋章なし３辺都市を引く場合なら、ミープルを置かないほうがいいまである。",
        "きつい・・・きついんだよ、匂いがな！",
        "トータルで 20 点勝ちなので得点をとにかくとって逃げ切る展開が狙えそう。相手に道付きのリップで修道院完成しながら強い都市を建築されると辛いので先に予約してしまう。道も都市も死ぬ確率高いけどどちらも反対側に伸ばせる余地があり弱くない。",
        "フリーリップは私に有利になるように置きたい、",
        "まだ開始間もないならジャブで様子を見つつ相手がどの様に振る舞うか？を見定めるのも悪くないわよね。",
        "点がとれる匂いがするねぇ。ただ、相手も点をとるだろうねぇ。ここがましってこと。",
        "オフェンススタイル！真ん中の都市は死ぬ前に防御できることも多そうなので、なんとかなるやろの精神で。",
        "相手の都市完成の一手でこちらの都市２つの完成確率が下がるような形を避ける。",
        "先手草原にもってこいの展開。道もカーブで繋いで先の大草原を意識する",
        "なんだかんだでフリーリップを残したくないな、アンダーバーの下に置くのはブロックされにくいってのはあるけど、相手がリップと左折を引くならもちろん取るとして、土管と道なし２、３辺はまあ流石に自分に取らせないために取ったほうがよさそう。相手の次のタイルがカーブ、修道院と４辺なら上には置けないけど、２、３辺は正直まだ分からない。それ以外のタイルならここで置くのはいいと思う。",
        "相手がフリーリップを取ってくれれば、リップ完成＋破壊の布石作られるまでの時間稼ぎできるし、リップへの妨害の布石にもなる。\nリップの完成優先してきたら、その代わり下のリップ取れたらいいなー…",
        "いつも迷いなくこう置いているけど問題にされるとわからなくなってきた。\n破壊される確率は多分半分もないからこれで良いような気がしている。全く自信ない。",
        "序盤なので強気に。\n相手が破壊に道タイルを優先的に使ってくれるなら自分は道の終端も取れそうなので。\n相手のコマをどこかのタイミングで破壊出来れば実質イーブン。",
        "左折の下に置いて右の都市を予約する場合赤が予約済みの都市を建築したときに上下の黄色の都市を辛くされるし、空きリップを残すことで運要素が生まれるので一手で左折固定にされるリスクをとってでもワントップで良いと思う。",
        "草原を制圧できれば勝ち。高確率でミープル帰ってきそうなので全投入OK！修道院を引いてもミープル乗せて置くとないしね。",
        "点差的に草原取りきれば勝てそう？",
        "引いて嬉しくない道タイルを、引いて嬉しいタイルに変えました。ほとんどのタイルで大草原か大都市か駒回収かの高価値な行動ができそうです。",
        "二辺都市もまだ十分残っているうえ、カーブもある！\n草原が繋がる可能性もミープルが戻ってくる可能性も十二分ッ！！\n盤面左側の右向きカーブに寝ている赤ミープルから察するに黄色が寝たのに反応して寝ているはず！つまり相手は追う姿勢！\nだったらミープルを全て草原に投げうってでも圧力をかけるべし！\nおとこ度胸の全投入で相手の肝を震え上がらせろ！\n……………ッ!?\nこれってもしかして相手はAI？\n……そもそも震える肝を持ち併せていない？",
        "草原を乗っ取られる以外に負け筋がないのでダメ押しのスリープ。",
        "草原で負ける可能性は全然あるのて、唯一草原に繋がる確率の高いスポットを相手に取らせない。草原繋げて+22になったら相手も草原を取らないと勝ち目はない、そうすると相手は草原に相乗りするために必ず道路タイルを使うってなると自分のミープルを返らせる確率はほぼ100%と言ってもいいと思う。",
        "ミープルはどうせ帰ってくるけれど、ここで点を取られたら終わりな匂いがするのよね",
        "自分の盤面だけど、改めて残りタイル確認しながら考えたらやっぱり自分で打ったところでいいと思った。\n草原戦になりそうな盤面で、コマ数有利に持って行くのは定石だと思う。\n自分はまだ草原にもう一コマ入れる状態なので草原相乗り場を作るのは問題なさそう。",
        "これでよし。",
        "相手が草原に相乗る場所を潰しながら下側とは別のタイルでも草原に入れるようにして草原戦を勝ちにいく手。残りタイル的にミープルを持っててもそんなに使い道がないので全駒してよい。右上の自分の都市をもし完成できた場合に 6 点草原に寝れないデメリットはあるけどそもそも相手も欲しがっているリップを引いて都市回収できる時点で嬉しいし相手はこの 6 点草原をとっててもおそらく勝てないので寝られる心配も少ない。",
        "次順、道が付いてる都市が引けたらノリノリ。",
        "これに赤が相乗りしてきたらヤクザやで。このあと道付きリップで相乗り行きます。",
        "とりあえず大きくなりそうな大都市に相乗りの牽制。完成させる必要はないが、駒数イーブン以上にしておけば修道院の点もあるので十分戦える。",
        "修道院置き場を維持しつつ、唯一の道の終端を取らせないように道予約。相手にT字路で右折に固定されるパターンが少し面倒で、実際の試合も相手に上側の終端に道修を置かれて修道院左側の扱いに困った。\nフリーの都市は修道院に絡む1点分自分に有利だが、紋章つきなので相手が取っても普通に強いのが気になるところではある。\n色々考えたが破壊も相乗りもそんなに強くない気がするのでやっぱりここで良いのかな。",
        "保留",
        "マナマカ若しくは右折リップ以外には相乗りさせんぞ！という固い意志表明。むしろコッチがマナカナor右折リップで乗っ取りブチかます気持ちで！",
        "3:1破壊のリスクが大きいので、相手はかなり相乗りし辛いし、こちらが相乗りできれば完成したら大量得点か、頑張って破壊しても同じ数のミープル消費でこちらのがたくさん点数取れそう！",
        "赤の道をholdし、道は右に向けて自分の得点源に。",
        "都市の相乗りが一番点数稼げる。相手の道を右折にする択もあるか、感覚的に３枚も出尽くさない限りやる価値は少ないと思う。その他左下の修道院と道のシェアはちょっと効いてる、普通は自分の得点だけ優先して８点取ってから最後の一枚０点行動で修道院を回収するのは一般的、例え破壊されても一緒なので、都市タイルを使うまで破壊する価値はない。だから右の道を右折にするということは修道院を完成しないと行けないことなので結構めんどいと思う。最初ストレートを引けるなら、０点行動は１手で済むか、カーブなら２手も必要となる。得点を優先するなら修道院が完成しつらいし、相手にも破壊されやすい。",
        "ここで足止めしとけば、その後いける匂いがするのだよ、匂いがな！",
        "相手のコマと自分のコマを2:1で巻き込む形。\n相手は右折タイルを無駄に使う必要がある。\n3辺都市の道は置かなくても相手は破壊を恐れて置けないので置かなくて良さそうだけど、序盤だし、T字リップ引いた時に2点追加でもらえるので置く。\n相手が都市に相乗りしてきたら道を伸ばしつつ相乗り場を作り、右折固定で守られたら相乗り狙う、でも遅くなさそう。",
        "追加の修道院スポットを右に残しつつ、修道院の周りで道や都市を建築できそうな形で嬉しい。",
        "ワントップを作りたいけど良い場所がない。修道院が見えてないからミープル温存！ゴージャスつけたことを後悔させてやるぜ！",
        "終端付きの5点道を優先！\nリップは取り合いになるが、二辺都市系のタイルが残り少ないので、相手の左上の都市にリップが必要になる可能性が高いので、取られてもリップを使わせたのでまあ良しとする。\nオスピー受けは上の四辺都市で、土管は上の四辺都市と下の中規模都市をつなげられたら美味しい！",
        "道欲しいよね？欲しくない？",
        "確定5点をもらいます。フリーの都市ももらう予定です。",
        "二辺都市や三辺都市を左下に置いて３点行動取っても相手に相乗りされないために置く、二、三辺は結構出てるか、道なしと道ありは各３枚以上あるのでちょっと不安はあるけどギリギリ十分かな。と考えてる途中にミープルが少ない事に気づいて、しかもその中で３つも都市共有系でカーブと同じ回収しやすいものではないのでミープルを節約したほうがよさそう。道付き二辺を引く時は四辺の右に置いてマナカナの運ゲーを下げたいしな、左下の相乗りを試みると右上の都市の使うタイルがちょっとダブる。今ミープル２個も余裕あるので修道院か右の５点道か左上の都市相乗りでも出来る。",
        "未だ道の終点も複数残ってるしこのまま放置して取られるには惜しい道だから先にツバつけておきたいお年頃。",
        "リスクに見合ったリターンがあると考えてここ。二辺を引いたとき本来は再相乗りが怖くて繋げないがこの形なら繋げる。3手で+19~23点。もともと道付き二辺なら5点道を取るのに使えるが、相手に先に取られたり自分がカーブ等で予約する展開も多く、そうした状況で相乗りで使えるのは強い。三辺は右上の都市の合体に使えるが、そこは土管で繋いでもいい箇所。残り二辺系4枚三辺系3枚は少なくないし、十分賭ける価値がある。",
        "どこも一長一短なんだよね。ならどうするんだい？終点の匂いがするんだよね、匂いが！",
        "赤のマナカナ分断ケア。\n相乗りに行ける形で、相手が恐れて3辺なりを自分の年に繋げてくれればラッキー。\n\n修道院のスポットが出来るが、相手が修道院を置いた場合、左折固定で都市か修道院どちらかは破壊しやすい形。",
        "相手に置かれたくないので先に置く。二辺三辺が減ってきているのでリップを引いたら４点取って回収で良し。",
        "土管型の都市やオスピー (垂直に分断する都市) を強く使えるように右でワントップの都市を作るなどしたいが空いた道を強化し過ぎることになりあまり良くなさそう。四辺都市をつけられた上の都市は二辺系と三辺系で建築するのも難しそうなので、ここの建築に使うのも最悪ではないがあまり強くはなさそう (右から建築すればマナカナ (斜めに分断する都市) + 二辺系で都市乗っ取りパターンのケアに少しなるが、相手は同じタイルをどうせ左下にも置けるのでそこまで嬉しくもない) 。結局、道の終端、特に T 字リップと道付き修道院、がたくさん残っていることを考えると、この道を予約するのが強そう。空きリップを取られたら相手の都市だけ右下に伸ばしていける展開を少し牽制することにも一応なる。",
        "壊せる時に壊すが信条だけど、めっちゃ負けてるので点数を取りに行く",
        "スタートタイルがラスト1枚になったので、破壊されないように上のリップを完成させる…\nと言いたいところだが、かなり点差が負けてるのでそんな消極的なことを言ってる余裕はなし！\n\n残りミープルの数を考えてもリップ使ってまで修道院破壊するのはそんなに強くなさそうなので、取るか取られるかで8点違うフリーリップを取ります！",
        "リップ放置はじりじり離されていくだけと判断。相手がみーぷるをダボつかせてることを考えると、リップで殺すのは早計と判断。",
        "修道院を回収しながらの道修や、同じくカーブで草原取られると30点弱の点差になってしまって、さすがに挽回が難しいと判断しました。\nこの後、相手が三辺都市を3連続で引いてる隙にストレート→修道院→修道院で逆転する予定です。",
        "中盤なので我慢。コマを残しておいて、修道院でまくる展開にしたい。",
        "既に点差があるので、点差を埋めるために修道院破壊にして相手のミープル効率を下げることにしました。\n上の都市完成させながら寝ることも考えたんですが、実質6点行動で相手の草原に入ろうとしても妨害にあってなかなか点数がのびないきがしました。",
        "ミープル数がカギになる匂いがするのよ、匂いがな！",
        "盤面上でフリーリップがないなら迷いなく破壊するけど、それでも流石にこれしないかな、道修２枚もあるしカーブも多く残ってる。ミープルの価値ってどれくらいあるだろう？９点草原あるから、９点以上の価値があるのかな、少なくとも適当なところに置いても３点以上の得点は確定か。もし自分の都市が完成したら６点行動があるとしたらあっちのほうが優先したい気持ちはあるって考えると４点都市を取ってもそこまでは悪くないか",
        "相手の道修スポット→リップを使うのは勿体無い。\n上の都市完成→相手の草原内であまり点が伸びない→3辺や2辺で大きく完成狙いたい。\n\n得点差で負けている以上、フリーで4点取れるところを残すべきでないと考えて即4点行動。",
        "負けているので点差を縮めておく。",
        "点差が酷く得点行動をしたいが、道修建築やカーブで9点草原に入られるのは許容できないので破壊。\n今後の展開として道や都市回収と上下の草原を取ることで20点くらい楽に稼げるため、今はまだ焦らず堅実なプレイをする。",
        "カーブや道付き修道院をひかれちゃぁたまったもんじゃぁないので相手修道院のミープルを戻させないようにロック！まだまだリップ片はたくさんあるから小さく稼ぎつつ草原に寝て逆転ブチかます絵図を描くッ!!!!",
        "19 点負けでとても厳しいけどまずは一体破壊して、せめて長期的なミープル数で優位に立ちたい。リップと修道院をたくさん引いて点差を縮めていこう。",
        "草原戦は嫌い",
        "1番道の邪魔になりにくそうなとこで建設。いうてこれも相乗りやワントップからプレッシャーかけられそうで嫌だけど。",
        "これ普通によくある盤面なのでみんなの意見気になる…\n\n取りあえず相乗りするなら道をくれって感じで…",
        "天に祈る",
        "なるべく自分の置いた道に干渉したくない。且つ、都市でも極力干渉されたくない。But!相手に都市をとられるのは避けたい！……ならばここがいいかしらね。",
        "ここで都市右向き以外に選択肢があるのかとしばらく考えて、都市左向きも一応アリだと気づいた。\nカーブの右に三辺、ストレート、リップなどを置かれたときのダメージが少ないのが利点。\n固定されたとしても道は左折、都市は右折で被らないのも良い。\nでも自分がカーブ/ストレート/十字路を引いた時使いにくい。1/4で引く道系が微妙になるのが少し嫌。\n右向きのほうがやっぱり強い気がするが、今度この場面になったら左向きを試してみたくなった。",
        "相手が道か都市タイルを引いた場合は右の破壊を狙うのが最善手、それを踏まえて自分の都市は邪魔されない位置で置く。",
        "結局南極こつぶっこ。おいしい楽しい匂いがするね",
        "やや狭いが相手に都合の良い修道院置き場を与えたくない。妨害されやすそうだが左の道を繋ぐタイミングは自分で選べる。先手の有利も考慮するとこれで悪くないと思う。",
        "よくみる盤面な気がするけどここが1番良い気がしてる。\n・相手は都市に妨害に行くには自分に1点あげないといけない。\n・道付き都市と道無し都市の使い分けができる。",
        "道と都市をまとめて破壊される展開に一番なりにくそうと思った手にした。カーブの右に何かを置かれるだけで両方狭くされるけど、しょうがない。",
        "道ついてない都市を引いて困ってください",
        "悔しいけどミープルが少ない方だから仕方がない",
        "終端がまだたくさん残ってるので5点道を取りに行く\n\nワントップがくずれるのがちょっと嫌だが、ミープル数的に相手は道無し二辺で牽制相乗りもかけ辛いし、一番引かれたくない道無し三辺も残り1枚で確立が低いので…",
        "劣勢だがこれといって良い手がないのでここ。相手の駒が少ないので完成まで数手遅らせるだけでも点数効率を落とせる。",
        "兎に角相手にミープルを戻さぬ事を考えて戻りやすそうなリップの隣に草原を向けてセット。\n",
        "ぶちかます準備をしておきました！笑\n左上の道も気持ちは予約してます。",
        "相手の草原を封鎖しつつカーブで楽にミープル回収させない手。\n次に相手の都市を相乗りで止めれば相手はほとんど何もできなくなる。",
        "暇やな、暇な匂いがするな",
        "全日本素点を稼ごう協会",
        "10点負け(土管も引かれたら16点負け)なので、相手のコマの回転をなるべく下げて、点を伸ばしていく。\n右折で回収できるところは回収しても±0なのと、相手のコマを返してしまう可能性があるので、右の修道院回収に使いたい。コマ数有利をキープしつつ、別で草原作ればまだ勝てそう。",
        "自分と相手の次のタイルが三叉路と仮定して自分が何も置かない場合は0-5+2=-3、一方ミープルで道を取る場合は+5-2+1=3って６点差がある。道の得点率低いので普通は都市に道刺してのが一番いいか、４点道は見逃しかたい。その後オスピー３枚とマナカナ１枚も出た上で相手は土管の置き場があるので、この盤面での道刺しは利点多くない。",
        "草原の広がる余地を無くす一手。相手の方がミープルきつめなので道のミープルが戻りにくくなるのも嬉しい。右の修道院の破壊の足場はできるけど、ここを完成するには強めのタイル 2 枚を費やさないといけなくてどの道しんどいので完成しなくても許容できるかな。",
        "リップを完成しながら紋章土管の都市を左折固定されると面倒なのでその牽制への布石と、中央下の赤の都市への左折固定の布石に。\nミープル置くか置かないか迷ったけど、リップを完成されながら両方に妨害かけられるのは嫌なので…",
        "この二辺、置くとこない！\nそんな時は次また同じようなのを引くことを想定しておこう！",
        "どこも置きたくないからここ。ストレートの置き場に困ったら一応固定に使えるし。",
        "自分が提出した盤面だけど、元々は相手のプレーンリップの下に、都市右向きで置いていた。\nけど、左の相手の都市はまだ左折3枚残っているし、相手が道付きのリップを引いた時に自分の都市が2枚攻撃を喰らうので、ここでは相手の都市を曲げて、コマ数有利を考えてみた。道無し2辺も3枚出てるし、ここを破壊できればコマ数有利、相手は右からさらにコマ追加で乗ってくるのは勇気がいるのでここの方が良かったと思う。",
        "左右の都市を妨害する足場を作る。カーブとストレートは右の都市に刺し、２辺引いたら左で点数を稼ぎながら破壊の足場を作ることができる。自分の土管付き都市は相手の修道院とシェアなので正直左折にされてもあんまり痛くない。ミープルを置く必要性はこのタイルは相手に取られたくないから置くって思考でいいと思う、この盤面は相手がどんな都市タイルを引いても自分の都市建築を優先するので、都市が完成されるリスクを考えるとミープルを置かないほうがいい。",
        "よってらっしゃい、みてらっしゃい！これがうまい手と見える匂いだよ！",
        "次に左折リップなどで赤の都市を完成されても残りタイル的に全然痛くないので、赤の８点都市完成の阻止を狙う。",
        "相手のリップへの妨害の土台に",
        "受けの広い形でfix",
        "ベーシックに。土管の方は刺されても左折なのでデュプリケーションにもならない。",
        "都市建築と同時に破壊を狙う。道付き二辺で逃げようとしてきたら相乗りできるので下側からの妨害よりも強い。今後リップに道を刺せば相手の待ちが被って良い形。",
        "修道院の上に更に修道院を重ねられちゃぁたまったもんじゃないんでミープルが戻りにくい形にするのが上策と踏んだわ！何より自身の修道院下に自らオスピー入れて左の都市を封じる代わりに修道院を犠牲にしている時点でその覚悟アリ！とあたしゃ受け取ったわ。",
        "相手の都市両方にプレッシャーをかける手。都市を右向きにすると相手の右の都市完成でその上下の自分の都市が辛くなりそうだったのでこちら向きにした。",
        "分断も考えたが、結局もう一回相乗りしにいくことになって二度手間になりそうなので、リスクを取らずにここで！\n下のリップ取る時に道が刺さると2分の1で都市が繋がらずにリップを1点止まりにできるかも！",
        "私の方が置きやすいフリーリップ大好き！",
        "こういう展開好きじゃないけど仕方なく。",
        "大都市にせず回収するつもりだけど・・・。上と迷う・・・。",
        "右の都市はこう着状態なので、あまり触りたくない。相手がフリーのリップを道付きで取ると、自分の相乗り場をラスイチの道付き3辺固定になるので置きにくいはず。道付き3辺を自分が引けてから、上の土管をゴミ捨て場として伸ばしていく。",
        "外反打ちをする気満々で置いた盤面一番下の二辺都市だが、マナカナを引いたとあれば話は別ッ!!!!ここで分断させ盤面一番下の黄色ミープルを戻す事を考える。と、同時に大きくなりそうな都市にはなるべく赤ミープル2つを戻せなくする様に動べしッ!!!!",
        "泥試合の匂いがするぜ",
        "盤面を悪くするよりはフリーリップを許容して安全に行く。\n三辺接続の部分に左折などで道を刺せたら50%で破壊できて自分の都市だけ上に伸ばしていけるし、下の自分の都市を分離して救う展開もありうる。手番を使う価値は感じないので、リップを取ったときに自動で道を刺せたらラッキーくらいの気分で、草原ではなく都市下向き。大都市バトルに発展したとしても相手は左側の都市回収が遅れるため、やや自分に有利な展開。\n",
        "*３辺都市を引いたら置き場ある \n*2辺都市はこの盤面だと大都市に繋げにくい、相手が都市の右で2、３辺を置くとブロックされやすい、たぶん普通にリップの建築をする\n*リップを完成する時は自分の邪魔をするか、そこに道で破壊されでも相手が２、３辺を引く時はゴミ捨て場に置くより左の都市建築を優先する、まあでも後半に行くと２、３枚くらいは捨てるかな。\n*マナカナを右上に置く手もあるか、相手に道なし２、３辺とアンダーバーを引かれるとマナカナ+オスピー+土管って３つのボーナスタイルが置けなくなる、後自分が先手側なので十字路を引いたら後手になる。",
        "あまり強くはないが他に良いところもない。下のリップは三辺で大都市に相乗りも見据えたいが状況によって単独で完成でも良い。しかしあまり強くはない。",
        "妨害しにくいところで都市を作り始める。",
        */
        "相手に防御の一手を使わせる。",
    ] {
        println!("original note: {:?}", note.to_string().replace("\n", ""));
        let lang = t.detect_language(note.to_string().replace("\n", ""));
        let translation = t.translate(note.to_string().replace("\n", ""), lang);
        println!("translated note: {:?}", translation);
        println!("");
    }
    assert!(false);
}

#[test]
fn create_english_to_japanese_glossary() {
    let t = Translator::new();
    let glossary: Vec<String> = vec![
        ("city", "都市"),
        ("road", "道"),
        ("field", "草原"),
        ("temple", "修道院"),
        ("church", "修道院"),
        ("city cap", "リップ"),
        ("platform", "足場"),
        ("to steal", "乗っ取る"),
        ("triple city", "三辺"),
        ("triangle", "二辺"),
        ("to hinder", "妨害する"),
        ("to block", "妨害する"),
        ("blockable", "妨害できる"),
        ("unblockable", "妨害できない"),
        ("hinderance", "妨害"),
        ("square", "マス"),
        ("to grab", "とる"),
        ("behind", "負けている"),
        ("ahead", "勝っている"),
        ("crossroad", "交差点"),
        ("crossroads", "交差点"),
        ("ruin", "ゴミ捨て場"),
        ("to equalize", "相乗りをする"),
        ("equalization", "相乗り"),
        ("to invade", "相乗りする"),
        ("feature", "地形"),
        ("with tempo", "テンポ良く"),
        ("to meeple", "ミープルを置く"),
        ("left-handed dagger", "左折"),
        ("right-handed dagger", "右折"),
    ]
    .into_iter()
    .map(|pair| format!("{}\t{}", pair.0, pair.1))
    .collect();

    let res = t.create_glossary(
        glossary.join("\n"),
        "english to japanese glossary".to_string(),
        "en".to_string(),
        "ja".to_string(),
    );
    println!("res = {:?}", res);

    assert!(false);
}

#[test]
fn english_translation_test() {
    let t = Translator::new();
    for note in [
        /*
        // problem 1
        "Small hinderance to the city + future threat of starting a road to the right, creating an attacking square",

        // problem 2
        "Well, it's a good road + opponent already needs 1 out of the 5 triangular cities left, so if they want to use the top open cap they would ideally need to get 2 more triangles... They can allways use triple cities, but then we get chances to equalize",
        "All options are bad so... I'll get this one",

        // problem 3
        "Really unsure about this one, but getting maximum point value from the tile can't be that bad and if the opponent drives the road into our city we have more squares do grow it / opportunity to defend",
        "It is an awkward tile to block with and it would fit perfectly for our city, but still - a permanent block is a permanent block so we have to finish what we started. No city caps have been drawn, so we often will be able to create a field and use our meeple advantage to outscore Red's ruin.",

        // problem 4
        "Even if it is a ruin, we need to attempt a connection to a 8-point + growable ruin",

        // problem 5
        "We create good value from a curvy tile, having already a use for straight roads, and we setup a 4 point road for ourselves when closing the city (5 tiles for that, 3 give full road value)",
        "Taking 2 points, adding one point to your road and creating a 6-point field upon finishing your city gives a multipurpose use for the crossroad tile. Finishing your ring road is not important as of right now as it is not restricted in any way",

        // problem 6
        "Prevents an easy entry to the city, and it still is quite secure from attacks",
        "Triangle is able to be used effectively to keep yellow's city safe from any attacks coming from the opponent except with the splitter. At the beginning of the game it is not as important to have completely unrestricted squares to be built later as it is to secure your big features from attacks",

        // problem 7
        "The road restriction is not very problematic for the time beimg, but giving away open caps to the opponent is :)",
        "It is unfortunate that we leave an empty city cap but allowing Red to complete an 8-point city is worse, so we have to play this move. Also, the empty city cap can sometimes benefit us. If Red takes the empty city cap and scores 4 points, this will give us a platform to try and steal the whole city with a city cap  + triple city tile.",
        "Red has an immediate threat of finishing an 8-point city, which is the most lucrative feature on the table as of right now. Yellow's best bet to counter it is to attack the said city. Taking the field is not important yet as there are plenty of connection spots one can use later on",

        // problem 8
        "We need to prevent the city from closing, even if we end up with a losing minibattle it would be for 1 point only, and with a triple city, we can grow a ruin for ourselves",

        // problem 9
        "Good road + road end into our city might yield some points in the future if we get a city + road end tile",

        // problem 11 - 20
        "Having a 20-point lead, it feels like preventing the opponent from having threats is a good move",
        "The opponent might block to a 75% chance, but there were already scenarios in which they close their city and constrict ours anyway. Besides, leaving open caps this early on is not ideal, and now we have a threat of closing an 8-point city",
        "It's a full block, and given we are 15 points ahead with the majority on the field (by getting 1 out of the 4 triangles left) our priority should be to prevent our opponent from scoring lots of points and to preserve the field majority",
        "If the opponent tries to attack the city after us, they create a nice road to grab and we now have a better incentive to drive the city into their road",
        "We could maybe attack the city, but leaving a potential 8+ points road up for grabs is not ideal. Plus, if the opponent does not get a useful road tile, we may even get the road all for ourselves",
        "We are so far behind that we need to grab all possible points, and hope to block our opponent later with less useful tiles ",
        "Seems better to trade the safety of the road for that of the city",
        "Attacks opponent's left city while creating a platform to block the right one",
        "Triple down on the city! Why not?",
        */

        // problem 21 - 46
        /*
        "This seems to me to be the most greedy move, and this is probably the 2nd best tile to finish the monastery and start a city to have some future threats on the board",
        "We don't really want to build up the city next to the opponent's monastery, so closing it soon is probably best",
        "Symmetrical setup, so both of us have a change to grab half of the big ring road",
        "There are still plenty of tiles that return us a meeple, and even if we have to wait for 2/3 moves we can either build our features up or restrict the opponent's ones",
        "We need to wait for a tile to connect ourselves to the opponent's city, so in the meantime getting some points and maybe getting a meeple back should be our objective",
        "Monasteries work great together + we want to keep the 50% chance of getting our monastery meeples back + the other 7-point place for the monastery also creates a 9-point field for our opponent to take",
        "Builds a nice road arround our monastery and restricts by a bit the opponent's city, while leaving chances to connect to it later",
        "It's a good field to grab since if we have the need to close the shared city we can even meeple another 6-point field while adding 3 points to our initial field. This play also comes with 2 monastery points and keeps our open city nicely unattackable",
        "I recognize this game, you are playing against RabbitRain! It is tough to solve the problem unbiasedly given that I remember what you played, but I think your move was actually the best move in that situation. The game is balanced in terms of points and meeples. You will be entering a winning mini-battle with 2 pts vs 1 point with chances to get a meeple back elsewhere. Completing the monastery is also a reasonable option.",
        "We get a nice field and prevent our opponent from joining another city to his through our monastery. Also if they get the 50% for connecting the top fields, we have some tiles to win it back, forcing the opponent to give us a monastery point to tie everything",
        "Delays Red's city completion (takes away almost all city caps) and continues the strategy of exploiting Red's temporary meeple disadvatange.",
        "The field mighr grow quite big, so we better get on it, plus we have chances to get the 1st city cap and to restrict by a lot our opponent's road",
        "The move with the biggest immediate point value + a chance to prevent Red from completing their city. It is the opening, so no reason to be afraid of getting Yellow's meeples blocked.",
        "Just dump it somewhere.",
        "painful move for me :D",
        "Depends on who were' playing - if we're playing a higher rated player, it is OK and maybe even better to go for the 50-50% gamble, if we are playing a lower-rated player, it is better to try scoring elsewhere.",
        "Occupying a triple city alone is not a good idea. Trying to break your opponent city is more profitable.",
        "Pre-build the road and attack the potential 12-pt field.",
        "The northwestern city cap should be left to create the chance of equalizing the 5-pt city.",
        "A triple city is easy to be attacked.",
        "Greed is good. The newly-occupied city will barely affect the structure of another yellow city.",
        "Cleaning off an empty city cap + opening a second city threat + defending the main city from attack.",
        "The correct sequence of retrieving the meeples of the southwestern part is the city first and then road, because finishing the road will create a blocking platform of the city, and there is no left-handed dagger remaining.",
        "Not adding monastery points to my opponent.",
        "do not let him meeple the road with tempo",
        "The southern road is unblockable, and the upward city should be blocked with a multi-city tile placing on the triple city, and facing leftwards. This move pre-build a road which may belong to us, and create a blocking platform of the blocking platform.",
        "prepare for blocking red city",
        "There are too many tiles that can save the red meeple while block 2 yellow meeples at the same time. And this tile can't be used as an air-defence. Consequently, blocking the meeples together will reduce the cost to the lowest degree.",
        "if we burn, they burn with us",
        "It controls the opponent's road, and there's a possibility to turn the city upwards using a triple city and block the opponent's 2 meeples with our 1 monastery meeple.",
        "control red road meeple",
        "Invading Red with a high chance of success, there's only one city splitter remaining and even if our opponent blocks the city ln the right we will have one more point than their monastery at most.",
        "Our city is worth more points, and it will be a complete protect. On the other hand, attacking my opponent's city will not always work, it might be blocked or re-attacked.",
        "The equalization of red's city seems more urgent to me because if my city get's blocked it's still equal to red's temple. Also, The save is only to an 88%. These considerations are because I'm ahead and don't want to leave red chances back by completing a big city.",
        "other city can be blocked but still makes a bit more points than the opponent's church",
        "Preventing Red from invading the city in upcoming moves, plus leaving an invading platform on the bottom part",
        "This move denies productive use for the opponent for most road tiles. Attacking the red's city later on from the right side will also provide an additional point for yellow's road. Being greedy in the beginning and taking advantage on as many features as possible whilst it isn't risky yet will provide you flexibility and limits your opponent's choices",
        */
        "Create a mini-battle to prevent the city from finishing. We don't mind losing a road-to-city mini-battle, because blocking the shared square and enlarging the ruin requires our opponent many moves.",
    ] {
        println!("original note: {:?}", note.to_string().replace("\n", ""));
        let lang = t.detect_language(note.to_string().replace("\n", ""));
        let translation = t.translate(note.to_string().replace("\n", ""), lang);
        println!("translated note: {:?}", translation);
        println!("");
    }
    assert!(false);
}
