// == File for struct of all registerable vegetables ==

use std::collections::HashMap;
use once_cell::sync::Lazy;



pub fn search_by_name(name: &str) -> Option<Vegetable> {
    VEGETABLE_LOOKUP.get(name).cloned()
}

pub fn search_by_family(target_family: &str) -> Vec<Vegetable> {
    VEGETABLE_LOOKUP
        .values()
        .filter(|veg| veg.get_metadata().family == target_family) 
        .cloned()
        .collect()
}



// Trait for metadata
pub trait VegMetadata {
    fn name(&self) -> &'static str;
    fn genus(&self) -> &'static str;
    fn family(&self) -> &'static str;
}



// Vegetable enum
#[derive(Debug, Clone)]
pub enum Vegetable {
    AoiFamily(AoiFamily),
    SuirenFamily(SuirenFamily),
    SumireFamily(SumireFamily),
    SeriFamily(SeriFamily),
    TadeFamily(TadeFamily),
    NadeshikoFamily(NadeshikoFamily),
    BaraFamily(BaraFamily),
    FuuchousouFamily(FuuchousouFamily),
    FuurosouFamily(FuurosouFamily),
    FutomomoFamily(FutomomoFamily),
    MameFamily(MameFamily),
    MukurojiFamily(MukurojiFamily),
    KikuFamily(KikuFamily),
    GomanohagusaFamily(GomanohagusaFamily),
    ShisoFamily(ShisoFamily),
    NasuFamily(NasuFamily),
    OmodakaFamily(OmodakaFamily),
    KayatsurigusaFamily(KayatsurigusaFamily),
    SatoimoFamily(SatoimoFamily),
    ShougaFamily(ShougaFamily),
    YuriFamily(YuriFamily),
    RanFamily(RanFamily),
    ShidaFamily(ShidaFamily),
    TokusaFamily(TokusaFamily),
    KikurageFamily(KikurageFamily),
    HaratakeFamily(HaratakeFamily),
}


// Family --------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum AoiFamily {
    Aoi(AoiGenus),
    Shinanoki(ShinanokiGenus),
}


#[derive(Debug, Clone)]
pub enum SuirenFamily {
    Suiren(SuirenGenus),
    Hagoromomo(HagoromomoGenus),
}


#[derive(Debug, Clone)]
pub enum SumireFamily {
    Uri(UriGenus),
}


#[derive(Debug, Clone)]
pub enum SeriFamily {
    Ukogi(UkogiGenus),
    Seri(SeriGenus),
}


#[derive(Debug, Clone)]
pub enum TadeFamily {
    Tade(TadeGenus),
}


#[derive(Debug, Clone)]
pub enum NadeshikoFamily {
    Akaza(AkazaGenus),
    Suberihiyu(SuberihiyuGenus),
    Tsuruna(TsurunaGenus),
    Tsurumurasaki(TsurumurasakiGenus),
    Hiyu(HiyuGenus),
}


#[derive(Debug, Clone)]
pub enum BaraFamily {
    Bara(BaraGenus),
}


#[derive(Debug, Clone)]
pub enum FuuchousouFamily {
    Aburana(AburanaGenus),
}


#[derive(Debug, Clone)]
pub enum FuurosouFamily {
    Nouzenharen(NouzenharenGenus),
}


#[derive(Debug, Clone)]
pub enum FutomomoFamily {
    Hishi(HishiGenus),
}


#[derive(Debug, Clone)]
pub enum MameFamily {
    Mame(MameGenus),
}


#[derive(Debug, Clone)]
pub enum MukurojiFamily {
    Mikan(MikanGenus),
}


#[derive(Debug, Clone)]
pub enum KikuFamily {
    Kiku(KikuGenus),
}


#[derive(Debug, Clone)]
pub enum GomanohagusaFamily {
    Goma(GomaGenus),
}


#[derive(Debug, Clone)]
pub enum ShisoFamily {
    Shiso(ShisoGenus),
    Murasaki(MurasakiGenus)
}


#[derive(Debug, Clone)]
pub enum NasuFamily {
    Nasu(NasuGenus),
    Hirugao(HirugaoGenus)
}


#[derive(Debug, Clone)]
pub enum OmodakaFamily {
    Omodaka(OmodakaGenus),
}


#[derive(Debug, Clone)]
pub enum KayatsurigusaFamily { 
    Ine(IneGenus),
    Kayatsurigusa(KayatsurigusaGenus),
}


#[derive(Debug, Clone)]
pub enum SatoimoFamily {
    Satoimo(SatoimoGenus),
}


#[derive(Debug, Clone)]
pub enum YuriFamily {
    Yuri(YuriGenus),
    Yamanoimo(YamanoimoGenus),
    Ayame(AyameGenus),
}


#[derive(Debug, Clone)]
pub enum ShougaFamily {
    Shouga(ShougaGenus),
}


#[derive(Debug, Clone)]
pub enum RanFamily {
    Ran(RanGenus),
}


#[derive(Debug, Clone)]
pub enum ShidaFamily {
    Uraboshi(UraboshiGenus),
    Kijinooshida(KijinooshidaGenus),
    Zenmai(ZenmaiGenus),
}


#[derive(Debug, Clone)]
pub enum TokusaFamily {
    Tokusa(TokusaGenus),
}


#[derive(Debug, Clone)]
pub enum KikurageFamily {
    Kikurage(KikurageGenus),
}


#[derive(Debug, Clone)]
pub enum HaratakeFamily {
    Kishimeji(KishimejiGenus),
    Haratake(HaratakeGenus),
    Benitake(BenitakeGenus),
    Moegitake(MoegitakeGenus),
}


// Genus ----------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum AoiGenus {
    Okura,
    Roselle,
    Okanori,
    Zeniaoi,
    Tororoaoi,
}


#[derive(Debug, Clone)]
pub enum ShinanokiGenus {
    Moroheiya,
}


#[derive(Debug, Clone)]
pub enum SuirenGenus {
    Hasu,
}


#[derive(Debug, Clone)]
pub enum HagoromomoGenus {
    Junsai,
}


#[derive(Debug, Clone)]
pub enum UriGenus {
    Uintameron,
    Kantaropu,
    Kyuri,
    Kurodanekabocha,
    Zasshukabocha,
    Shirouri,
    Suika,
    Zukkiini,
    Seiyoukabocha,
    TsunoMeron,
    Meron,
    Tougan,
    Tokadohechima,
    Nigauri,
    Nihonkabocha,
    Nettomeron,
    Hayatouri,
    Hyoutan,
    Hechima,
    Hebiuri,
    Pepokabocha,
    Makuwauri,
    Yuugao,
}


#[derive(Debug, Clone)]
pub enum UkogiGenus {
    Udo,
    Taranoki,
}


#[derive(Debug, Clone)]
pub enum SeriGenus {
    Ashitaba,
    ItarianPaseri,
    Koriandaa,
    SuupuSerori,  
    SuupuMinto,  
    Seri,  
    Serori,  
    Chaabiru,  
    Diru,  
    Ninjin,  
    Paasunippu,  
    Paseri,  
    Hamaboufuu,  
    Fennneru,  
    Mitsuba,
}


#[derive(Debug, Clone)]
pub enum TadeGenus {
    Yanagitade,
    Aitade,
    Rubaabu,
}


#[derive(Debug, Clone)]
pub enum AkazaGenus {
    Okahijiki,
    Teeburubiito,
    Fudansou,
    Houkigi,
    Hourensou,
    Matsuna,
    Yamahourensou,
    
}


#[derive(Debug, Clone)]
pub enum SuberihiyuGenus {
    Tachisuberihiyu,
}


#[derive(Debug, Clone)]
pub enum TsurunaGenus {
    Tsuruna,
}


#[derive(Debug, Clone)]
pub enum TsurumurasakiGenus {
    Tsurumurasaki,
}


#[derive(Debug, Clone)]
pub enum HiyuGenus {
    Amaransasu,
    Hiyu,
}


#[derive(Debug, Clone)]
pub enum BaraGenus {
    Ichigo,
    Saradabaanetto,
    WairudoSutoroberii,
}


#[derive(Debug, Clone)]
pub enum AburanaGenus {
    Aburana,
    Oosakashirona,
    Kiren,
    Kabu,
    Karashina,
    Karifurawaa,
    Kyabetsu,
    Kureson,
    Keeru,
    Koorurabi,
    Kousaitai,
    Koshousou,
    Komatsuna,
    Sugukina,
    Tasai,
    Daikon,
    Taisai,
    Daishinsai,
    Takana,
    Tanikutakana,
    Chingensai,
    Nozawana,
    Hakusai,
    Hukuran,
    Hatsukadikon,
    Hamana,
    PuchiVeeru,
    Burokkorii,
    Mizuna,
    Mibuna,
    Mekyabetsu,
    Rutabaga,
    Rukkora,
    Wasabi,
    Wasabidakon,
}


#[derive(Debug, Clone)]
pub enum NouzenharenGenus {
    Kinrenka,
}


#[derive(Debug, Clone)]
pub enum HishiGenus {
    Hishi,
}


#[derive(Debug, Clone)]
pub enum MameGenus {
    Azuki,
    Amerikahodoimo,
    Ingenmame,
    Endou,
    Sasage,
    Shikakumame,
    Juurokusasage,
    Soramame,
    Daizu,
    Tachinatamame,
    Choumame,
    Natamame,
    Nantenhagi,
    Hasshoumame,
    Hiyokomame,
    Hiramame,
    Fujimame,
    Benibanaingen,
    Raimabiin,
    Rakkasei,
    Ryokutou,
}


#[derive(Debug, Clone)]
pub enum MikanGenus {
    Sanshou,
    Henruuda,
}


#[derive(Debug, Clone)]
pub enum KikuGenus {
    Aatichooku,
    Endaibu,
    Orandasennichi,
    Kakijisha,
    Kamomiiru,
    Karudon,
    Kareepuranto,
    Kiku,
    Kikuimo,
    Gobou,
    Sarushifai,
    Sanchu,
    Shungiku,
    Shokuyoutanpopo,
    Suizenjina,
    Sutebia,
    Tachijisha,
    Chikori,
    Chisha,
    Tsuwabuki,
    Torebisu,
    Fuki,
    Yamagobou,
    Riifuretasu,
    Retasu,
}


#[derive(Debug, Clone)]
pub enum GomaGenus {
    Goma,
}


#[derive(Debug, Clone)]
pub enum ShisoGenus {
    Appuruminto,
    Uintaasaborii,
    Egoma,
    Oregano,
    Shiso,
    Supearaminto,
    Seiji,
    Taimu,
    Chorogi,
    Numahakka,
    Painappuruminto,
    Hakka,
    Bajiru,
    Pepaminto,
    Majoramu,
    Rabendaa,
    Remontaimu,
    Remonbaamu,
    Roozumarii
}


#[derive(Debug, Clone)]
pub enum MurasakiGenus {
    Konfurii,
}


#[derive(Debug, Clone)]
pub enum NasuGenus {
    Karantotomato,
    Shishitougarashi,
    Jagaimo,
    Shokuyouhoozuki,
    Shironasu,
    Tougarashi,
    Tomato,
    Nasu,
    Piiman,
    Hiranasu,
    Pepiino,

}


#[derive(Debug, Clone)]
pub enum HirugaoGenus {
    Satsumaimo,
    Yousai,
}


#[derive(Debug, Clone)]
pub enum OmodakaGenus {
    Kuwai,
    Suitakuwai,
}


#[derive(Debug, Clone)]
pub enum IneGenus {
    Toumorokoshi,
    Nemagaridake,
    Hachiku,
    Makomo,
    Madake,
    Mousouchiku,
}


#[derive(Debug, Clone)]
pub enum KayatsurigusaGenus {
    Ookuroguwai,
    Kuroguwai,
}


#[derive(Debug, Clone)]
pub enum SatoimoGenus {
    Konnyaku,
    Satoimo,
    Hasuimo,
}


#[derive(Debug, Clone)]
pub enum YuriGenus {
    Asatsuki,
    Asparagus,
    Oniyuri,
    Katakuri,
    Kanzou,
    Giboushi,
    Gyoujaninniku,
    Kooniyuri,
    Shallot,
    Tamanegi,
    Chive,
    Nira,
    Ninniku,
    Negi,
    Hananira,
    Yaguranegi,
    Yamayuri,
    Rakkyo,
    Riiki,
    Wakegi,
}


#[derive(Debug, Clone)]
pub enum YamanoimoGenus {
    Daisho,
    Nagaimo,
    Yamanoimo,
}


#[derive(Debug, Clone)]
pub enum AyameGenus {
    Safuran,
}


#[derive(Debug, Clone)]
pub enum ShougaGenus {
    Shouga,
    Myouga,
}


#[derive(Debug, Clone)]
pub enum RanGenus {
    Shunran,
}


#[derive(Debug, Clone)]
pub enum UraboshiGenus {
    Warabi,
}


#[derive(Debug, Clone)]
pub enum KijinooshidaGenus {
    Yamasotetsu,
}


#[derive(Debug, Clone)]
pub enum ZenmaiGenus {
    Zenmai,
}


#[derive(Debug, Clone)]
pub enum TokusaGenus {
    Tsukushi,
}


#[derive(Debug, Clone)]
pub enum KikurageGenus {
    Kikurage,
}


#[derive(Debug, Clone)]
pub enum KishimejiGenus {
    Enokitake,
    Shiitake,
    Shakashimeji,
    Hiratake,
    Honshimeji,
    Matsutake,
}


#[derive(Debug, Clone)]
pub enum HaratakeGenus {
    Masshuruumu,
}


#[derive(Debug, Clone)]
pub enum BenitakeGenus {
    Hatsutake,
}


#[derive(Debug, Clone)]
pub enum MoegitakeGenus {
    Nameko,
}



impl VegMetadata for AoiGenus {
    fn name(&self) -> &'static str {
        match self {
            AoiGenus::Okura => "オクラ",
            AoiGenus::Roselle => "ローゼル",
            AoiGenus::Okanori => "オカノリ",
            AoiGenus::Zeniaoi => "ゼニアオイ",
            AoiGenus::Tororoaoi => "トロロアオイ",
        }
    }
    fn genus(&self) -> &'static str { "アオイ" }
    fn family(&self) -> &'static str { "アオイ" }
}

impl VegMetadata for ShinanokiGenus {
    fn name(&self) -> &'static str { "モロヘイヤ" }
    fn genus(&self) -> &'static str { "シナノキ" }
    fn family(&self) -> &'static str { "アオイ" }
}


impl VegMetadata for SuirenGenus {
    fn name(&self) -> &'static str { "ハス" }
    fn genus(&self) -> &'static str { "スイレン" }
    fn family(&self) -> &'static str { "スイレン" }
}


impl VegMetadata for HagoromomoGenus {
    fn name(&self) -> &'static str { "ジュンサイ" }
    fn genus(&self) -> &'static str { "ハゴロモモ" }
    fn family(&self) -> &'static str { "スイレン" }
}


impl VegMetadata for UriGenus {
    fn name(&self) -> &'static str {
        match self {
            UriGenus::Uintameron => "ウインターメロン",
            UriGenus::Kantaropu => "カンタロープ",
            UriGenus::Kyuri => "キュウリ",
            UriGenus::Kurodanekabocha => "クロダネカボチャ",
            UriGenus::Zasshukabocha => "ザッシュカボチャ",
            UriGenus::Shirouri => "シロウリ",
            UriGenus::Suika => "スイカ",
            UriGenus::Zukkiini => "ズッキーニ",
            UriGenus::Seiyoukabocha => "セイヨウカボチャ",
            UriGenus::TsunoMeron => "ツノメロン",
            UriGenus::Meron => "メロン",
            UriGenus::Tougan => "トウガン",
            UriGenus::Tokadohechima => "トカドヘチマ",
            UriGenus::Nigauri => "ニガウリ",
            UriGenus::Nihonkabocha => "ニホンカボチャ",
            UriGenus::Nettomeron => "ネットメロン",
            UriGenus::Hayatouri => "ハヤトウリ",
            UriGenus::Hyoutan => "ヒョウタン",
            UriGenus::Hechima => "ヘチマ",
            UriGenus::Hebiuri => "ヘビウリ",
            UriGenus::Pepokabocha => "ペポカボチャ",
            UriGenus::Makuwauri => "マクワウリ",
            UriGenus::Yuugao => "ユウガオ",
        }
    }
    fn genus(&self) -> &'static str { "ウリ" }
    fn family(&self) -> &'static str { "スミレ" }
}


impl VegMetadata for UkogiGenus {
    fn name(&self) -> &'static str {
        match self {
            UkogiGenus::Udo => "ウド",
            UkogiGenus::Taranoki => "タラノキ",
        }
    }
    fn genus(&self) -> &'static str { "ウコギ" }
    fn family(&self) -> &'static str { "セリ" }
}


impl VegMetadata for SeriGenus {
    fn name(&self) -> &'static str {
        match self {
            SeriGenus::Ashitaba => "アシタバ",
            SeriGenus::ItarianPaseri => "イタリアンパセリ",
            SeriGenus::Koriandaa => "コリアンダー",
            SeriGenus::SuupuSerori => "スープセロリ",
            SeriGenus::SuupuMinto => "スープミント",
            SeriGenus::Seri => "セリ",
            SeriGenus::Serori => "セロリ",
            SeriGenus::Chaabiru => "チャービル",
            SeriGenus::Diru => "ディル",
            SeriGenus::Ninjin => "ニンジン",
            SeriGenus::Paasunippu => "パースニップ",
            SeriGenus::Paseri => "パセリ",
            SeriGenus::Hamaboufuu => "ハマボウフウ",
            SeriGenus::Fennneru => "フェンネル",
            SeriGenus::Mitsuba => "ミツバ",
        }
    }
    fn genus(&self) -> &'static str { "セリ" }
    fn family(&self) -> &'static str { "セリ" }
}


impl VegMetadata for TadeGenus {
    fn name(&self) -> &'static str {
        match self {
            TadeGenus::Yanagitade => "ヤナギタデ",
            TadeGenus::Aitade => "アイタデ",
            TadeGenus::Rubaabu => "ルバーブ",
        }
    }
    fn genus(&self) -> &'static str { "タデ" }
    fn family(&self) -> &'static str { "タデ" }
}


impl VegMetadata for AkazaGenus {
    fn name(&self) -> &'static str {
        match self {
            AkazaGenus::Okahijiki => "オカヒジキ",
            AkazaGenus::Teeburubiito => "テーブルビート",
            AkazaGenus::Fudansou  => "フダンソウ",
            AkazaGenus::Houkigi  => "ホウキギ",
            AkazaGenus::Hourensou  => "ホウレンソウ",
            AkazaGenus::Matsuna  => "マツナ",
            AkazaGenus::Yamahourensou  => "ヤマホウレンソウ",
        }
    }
    fn genus(&self) -> &'static str { "アカザ" }
    fn family(&self) -> &'static str { "ナデシコ" }
}


impl VegMetadata for SuberihiyuGenus {
    fn name(&self) -> &'static str {" タチスベリヒユ" }
    fn genus(&self) -> &'static str { "スベリヒユ" }
    fn family(&self) -> &'static str { "ナデシコ" }
}


impl VegMetadata for TsurunaGenus {
    fn name(&self) -> &'static str {" ツルナ" }
    fn genus(&self) -> &'static str { "ツルナ" }
    fn family(&self) -> &'static str { "ナデシコ" }
}


impl VegMetadata for TsurumurasakiGenus {
    fn name(&self) -> &'static str {" ツルムラサキ" }
    fn genus(&self) -> &'static str { "ツルムラサキ" }
    fn family(&self) -> &'static str { "ナデシコ" }
}


impl VegMetadata for HiyuGenus {
    fn name(&self) -> &'static str {
        match self {
            HiyuGenus::Amaransasu => "アマランサス",
            HiyuGenus::Hiyu => "ヒユ",
        }
    }
    fn genus(&self) -> &'static str { "ヒユ" }
    fn family(&self) -> &'static str { "ナデシコ" }
}


impl VegMetadata for BaraGenus {
    fn name(&self) -> &'static str {
        match self {
            BaraGenus::Ichigo => "イチゴ",
            BaraGenus::Saradabaanetto => "サラダバーネット",
            BaraGenus::WairudoSutoroberii => "ワイルドストロベリー",
        }
    }
    fn genus(&self) -> &'static str { "バラ" }
    fn family(&self) -> &'static str { "バラ" }
}


impl VegMetadata for AburanaGenus {
    fn name(&self) -> &'static str {
        match self {
            AburanaGenus::Aburana => "アブラナ",
            AburanaGenus::Oosakashirona => "オーサカシロナ",
            AburanaGenus::Kiren => "キレン",
            AburanaGenus::Kabu => "カブ",
            AburanaGenus::Karashina => "カラシナ",
            AburanaGenus::Karifurawaa => "カリフラワー",
            AburanaGenus::Kyabetsu => "キャベツ",
            AburanaGenus::Kureson => "クレソン",
            AburanaGenus::Keeru => "ケール",
            AburanaGenus::Koorurabi => "コールラビ",
            AburanaGenus::Kousaitai => "コウサイタイ",
            AburanaGenus::Koshousou => "コーショウソウ",
            AburanaGenus::Komatsuna => "コマツナ",
            AburanaGenus::Sugukina => "スグキナ",
            AburanaGenus::Tasai => "ターサイ",
            AburanaGenus::Daikon => "ダイコン",
            AburanaGenus::Taisai => "タイサイ",
            AburanaGenus::Daishinsai => "ダイシンサイ",
            AburanaGenus::Takana => "タカナ",
            AburanaGenus::Tanikutakana => "タニクタカナ",
            AburanaGenus::Chingensai => "チンゲンサイ",
            AburanaGenus::Nozawana => "ノザワナ",
            AburanaGenus::Hakusai => "ハクサイ",
            AburanaGenus::Hukuran => "ハクラン",
            AburanaGenus::Hatsukadikon => "ハツカダイコン",
            AburanaGenus::Hamana => "ハマナ",
            AburanaGenus::PuchiVeeru => "プチヴェール",
            AburanaGenus::Burokkorii => "ブロッコリー",
            AburanaGenus::Mizuna => "ミズナ",
            AburanaGenus::Mibuna => "ミブナ",
            AburanaGenus::Mekyabetsu => "メキャベツ",
            AburanaGenus::Rutabaga => "ルタバガ",
            AburanaGenus::Rukkora => "ルッコラ",
            AburanaGenus::Wasabi => "ワサビ",
            AburanaGenus::Wasabidakon => "ワサビダイコン",
        }
    }
    fn genus(&self) -> &'static str { "アブラナ" }
    fn family(&self) -> &'static str { "フウチョウソウ" }
}


impl VegMetadata for NouzenharenGenus {
    fn name(&self) -> &'static str { "キンレンカ" }
    fn genus(&self) -> &'static str { "ノウゼンハレン" }
    fn family(&self) -> &'static str { "フウロソウ" }
}


impl VegMetadata for HishiGenus {
    fn name(&self) -> &'static str { "ヒシ" }
    fn genus(&self) -> &'static str { "ヒシ" }
    fn family(&self) -> &'static str { "フトモモ" }
}


impl VegMetadata for MameGenus {
    fn name(&self) -> &'static str {
        match self {
            MameGenus::Azuki => "アズキ",
            MameGenus::Amerikahodoimo => "アメリカホドイモ",
            MameGenus::Ingenmame => "インゲンマメ",
            MameGenus::Endou => "エンドウ",
            MameGenus::Sasage => "ササゲ",
            MameGenus::Shikakumame => "シカクマメ",
            MameGenus::Juurokusasage => "ジュウロクササゲ",
            MameGenus::Soramame => "ソラマメ",
            MameGenus::Daizu => "ダイズ",
            MameGenus::Tachinatamame => "タチナタマメ",
            MameGenus::Choumame => "チョウマメ",
            MameGenus::Natamame => "ナタマメ",
            MameGenus::Nantenhagi => "ナンテンハギ",
            MameGenus::Hasshoumame => "ハッショウマメ",
            MameGenus::Hiyokomame => "ヒヨコマメ",
            MameGenus::Hiramame => "ヒラマメ",
            MameGenus::Fujimame => "フジマメ",
            MameGenus::Benibanaingen => "ベニバナインゲン",
            MameGenus::Raimabiin => "ライマビーン",
            MameGenus::Rakkasei => "ラッカセイ",
            MameGenus::Ryokutou => "リョクトウ",
        }
    }
    fn genus(&self) -> &'static str { "マメ" }
    fn family(&self) -> &'static str { "マメ" }
}


impl VegMetadata for MikanGenus {
    fn name(&self) -> &'static str {
        match self {
            MikanGenus::Sanshou => "サンショウ",
            MikanGenus::Henruuda => "ヘンルーダ",
        }
    }
    fn genus(&self) -> &'static str { "ミカン" }
    fn family(&self) -> &'static str { "ムクロジ" }
}


impl VegMetadata for KikuGenus {
    fn name(&self) -> &'static str {
        match self {
            KikuGenus::Aatichooku => "アーティチョーク",
            KikuGenus::Endaibu => "エンダイブ",
            KikuGenus::Orandasennichi => "オランダセンニチ",
            KikuGenus::Kakijisha => "カキヂシャ",
            KikuGenus::Kamomiiru => "カモミール",
            KikuGenus::Karudon => "カルドン",
            KikuGenus::Kareepuranto => "カレープラント",
            KikuGenus::Kiku => "キク",
            KikuGenus::Kikuimo => "キクイモ",
            KikuGenus::Gobou => "ゴボウ",
            KikuGenus::Sarushifai => "サルシファイ",
            KikuGenus::Sanchu => "サンチュ",
            KikuGenus::Shungiku => "シュンギク",
            KikuGenus::Shokuyoutanpopo => "ショクヨウタンポポ",
            KikuGenus::Suizenjina => "スイゼンジナ",
            KikuGenus::Sutebia => "ステビア",
            KikuGenus::Tachijisha => "タチヂシャ",
            KikuGenus::Chikori => "チコリ",
            KikuGenus::Chisha => "チシャ",
            KikuGenus::Tsuwabuki => "ツワブキ",
            KikuGenus::Torebisu => "トレビス",
            KikuGenus::Fuki => "フキ",
            KikuGenus::Yamagobou => "ヤマゴボウ",
            KikuGenus::Riifuretasu => "リーフレタス",
            KikuGenus::Retasu => "レタス",
        }
    }
    fn genus(&self) -> &'static str { "キク" }
    fn family(&self) -> &'static str { "キク" }
}


impl VegMetadata for GomaGenus {
    fn name(&self) -> &'static str { "ゴマ" }
    fn genus(&self) -> &'static str { "ゴマ" }
    fn family(&self) -> &'static str { "ゴマノハグサ" }
}


impl VegMetadata for ShisoGenus {
    fn name(&self) -> &'static str {
        match self {
            ShisoGenus::Appuruminto => "アップルミント",
            ShisoGenus::Uintaasaborii => "ウィンターサボリー",
            ShisoGenus::Egoma => "エゴマ",
            ShisoGenus::Oregano => "オレガノ",
            ShisoGenus::Shiso => "シソ",
            ShisoGenus::Supearaminto => "スペアミント",
            ShisoGenus::Seiji => "セージ",
            ShisoGenus::Taimu => "タイム",
            ShisoGenus::Chorogi => "チョロギ",
            ShisoGenus::Numahakka => "ヌマハッカ",
            ShisoGenus::Painappuruminto => "パイナップルミント",
            ShisoGenus::Hakka => "ハッカ",
            ShisoGenus::Bajiru => "バジル",
            ShisoGenus::Pepaminto => "ペパーミント",
            ShisoGenus::Majoramu => "マジョラム",
            ShisoGenus::Rabendaa => "ラベンダー",
            ShisoGenus::Remontaimu => "レモンタイム",
            ShisoGenus::Remonbaamu => "レモンバーム",
            ShisoGenus::Roozumarii => "ローズマリー",
        
        }
    }
    fn genus(&self) -> &'static str { "シソ" }
    fn family(&self) -> &'static str { "シソ" }
}


impl VegMetadata for MurasakiGenus {
    fn name(&self) -> &'static str { "コンフリー" }
    fn genus(&self) -> &'static str { "ムラサキ" }
    fn family(&self) -> &'static str { "シソ" }
}

impl VegMetadata for NasuGenus {
    fn name(&self) -> &'static str { 
        match self {
            NasuGenus::Karantotomato => "カラントトマト",
            NasuGenus::Shishitougarashi => "シシトウガラシ",
            NasuGenus::Jagaimo => "ジャガイモ",
            NasuGenus::Shokuyouhoozuki => "ショクヨウホオズキ",
            NasuGenus::Shironasu => "シロナス",
            NasuGenus::Tougarashi => "トウガラシ",
            NasuGenus::Tomato => "トマト",
            NasuGenus::Nasu => "ナス",
            NasuGenus::Piiman => "ピーマン",
            NasuGenus::Hiranasu => "ヒラナス",
            NasuGenus::Pepiino => "ペピーノ",
                    
        }
    }
    fn genus(&self) -> &'static str { "ナス" }
    fn family(&self) -> &'static str { "ナス" }
}


impl VegMetadata for HirugaoGenus {
    fn name(&self) -> &'static str { 
        match self {
            HirugaoGenus::Satsumaimo => "サツマイモ",
            HirugaoGenus::Yousai => "ヨウサイ",
                    
        }
    }
    fn genus(&self) -> &'static str { "ヒルガオ" }
    fn family(&self) -> &'static str { "ナス" }
}


impl VegMetadata for OmodakaGenus {
    fn name(&self) -> &'static str {
        match self {
            OmodakaGenus::Kuwai => "クワイ",
            OmodakaGenus::Suitakuwai => "スイタクワイ",
        }
    }
    fn genus(&self) -> &'static str { "オモダカ" }
    fn family(&self) -> &'static str { "オモダカ" }
}


impl VegMetadata for IneGenus {
    fn name(&self) -> &'static str {
        match self {
            IneGenus::Toumorokoshi => "トウモロコシ",
            IneGenus::Nemagaridake => "ネマガリダケ",
            IneGenus::Hachiku => "ハチク",
            IneGenus::Makomo => "マコモ",
            IneGenus::Madake => "マダケ",
            IneGenus::Mousouchiku => "モウソウチク",
        }
    }
    fn genus(&self) -> &'static str { "イネ" }
    fn family(&self) -> &'static str { "カヤツリグサ" }
}


impl VegMetadata for KayatsurigusaGenus {
    fn name(&self) -> &'static str {
        match self {
            KayatsurigusaGenus::Ookuroguwai => "オオクログワイ",
            KayatsurigusaGenus::Kuroguwai => "クログワイ",
        }
    }
    fn genus(&self) -> &'static str { "カヤツリグサ" }
    fn family(&self) -> &'static str { "カヤツリグサ" }
}


impl VegMetadata for SatoimoGenus {
    fn name(&self) -> &'static str {
        match self {
            SatoimoGenus::Konnyaku => "コンニャク",
            SatoimoGenus::Satoimo => "サトイモ",
            SatoimoGenus::Hasuimo => "ハスイモ",
        }
    }
    fn genus(&self) -> &'static str { "サトイモ" }
    fn family(&self) -> &'static str { "サトイモ" }
}


impl VegMetadata for YuriGenus {
    fn name(&self) -> &'static str {
        match self {
            YuriGenus::Asatsuki => "アサツキ",
            YuriGenus::Asparagus => "アスパラガス",
            YuriGenus::Oniyuri => "オニユリ",
            YuriGenus::Katakuri => "カタクリ",
            YuriGenus::Kanzou => "カンゾウ",
            YuriGenus::Giboushi => "ギボウシ",
            YuriGenus::Gyoujaninniku => "ギョウジャニンニク",
            YuriGenus::Kooniyuri => "コオニユリ",
            YuriGenus::Shallot => "シャロット",
            YuriGenus::Tamanegi => "タマネギ",
            YuriGenus::Chive => "チャイブ",
            YuriGenus::Nira => "ニラ",
            YuriGenus::Ninniku => "ニンニク",
            YuriGenus::Negi => "ネギ",
            YuriGenus::Hananira => "ハナニラ",
            YuriGenus::Yaguranegi => "ヤグラネギ",
            YuriGenus::Yamayuri => "ヤマユリ",
            YuriGenus::Rakkyo => "ラッキョウ",
            YuriGenus::Riiki => "リーキ",
            YuriGenus::Wakegi => "ワケギ",
        }
    }
    fn genus(&self) -> &'static str { "ユリ" }
    fn family(&self) -> &'static str { "ユリ" }
}


impl VegMetadata for YamanoimoGenus {
    fn name(&self) -> &'static str {
        match self {
            YamanoimoGenus::Daisho => "ダイショ",
            YamanoimoGenus::Nagaimo => "ナガイモ",
            YamanoimoGenus::Yamanoimo => "ヤマノイモ",
        }
    }
    fn genus(&self) -> &'static str { "ヤマノイモ" }
    fn family(&self) -> &'static str { "ユリ" }
}


impl VegMetadata for AyameGenus {
    fn name(&self) -> &'static str { "サフラン" }
    fn genus(&self) -> &'static str { "アヤメ" }
    fn family(&self) -> &'static str { "ユリ" }
}


impl VegMetadata for ShougaGenus {
    fn name(&self) -> &'static str {
        match self {
            ShougaGenus::Shouga => "ショウガ",
            ShougaGenus::Myouga => "ミョウガ",
        }
    }
    fn genus(&self) -> &'static str { "ショウガ" }
    fn family(&self) -> &'static str { "ショウガ" }
}


impl VegMetadata for RanGenus {
    fn name(&self) -> &'static str { "シュンラン" }
    fn genus(&self) -> &'static str { "ラン" }
    fn family(&self) -> &'static str { "ラン" }
}


impl VegMetadata for UraboshiGenus {
    fn name(&self) -> &'static str { "ワラビ" }
    fn genus(&self) -> &'static str { "ウラボシ" }
    fn family(&self) -> &'static str { "シダ" }
}


impl VegMetadata for KijinooshidaGenus {
    fn name(&self) -> &'static str { "ヤマソテツ" }
    fn genus(&self) -> &'static str { "キジノオシダ" }
    fn family(&self) -> &'static str { "シダ" }
}


impl VegMetadata for ZenmaiGenus {
    fn name(&self) -> &'static str { "ゼンマイ" }
    fn genus(&self) -> &'static str { "ゼンマイ" }
    fn family(&self) -> &'static str { "シダ" }
}


impl VegMetadata for TokusaGenus {
    fn name(&self) -> &'static str { "ツクシ" }
    fn genus(&self) -> &'static str { "トクサ" }
    fn family(&self) -> &'static str { "トクサ" }
}


impl VegMetadata for KikurageGenus {
    fn name(&self) -> &'static str { "キクラゲ" }
    fn genus(&self) -> &'static str { "キクラゲ" }
    fn family(&self) -> &'static str { "キクラゲ" }
}


impl VegMetadata for KishimejiGenus {
    fn name(&self) -> &'static str {
        match self {
            KishimejiGenus::Enokitake => "エノキタケ",
            KishimejiGenus::Shiitake => "シイタケ",
            KishimejiGenus::Shakashimeji => "シャカシメジ",
            KishimejiGenus::Hiratake => "ヒラタケ",
            KishimejiGenus::Honshimeji => "ホンシメジ",
            KishimejiGenus::Matsutake => "マツタケ",
        }
    }
    fn genus(&self) -> &'static str { "キシメジ" }
    fn family(&self) -> &'static str { "ハラタケ" }
}


impl VegMetadata for HaratakeGenus {
    fn name(&self) -> &'static str { "マッシュルーム" }
    fn genus(&self) -> &'static str { "ハラタケ" }
    fn family(&self) -> &'static str { "ハラタケ" }
}


impl VegMetadata for BenitakeGenus {
    fn name(&self) -> &'static str { "ハツタケ" }
    fn genus(&self) -> &'static str { "ベニタケ" }
    fn family(&self) -> &'static str { "ハラタケ" }
}


impl VegMetadata for MoegitakeGenus {
    fn name(&self) -> &'static str { "ナメコ" }
    fn genus(&self) -> &'static str { "モエギタケ" }
    fn family(&self) -> &'static str { "ハラタケ" }
}



// Struct for returned metadata
#[derive(Debug)]
pub struct VegMeta {
    pub name: &'static str,
    pub genus: &'static str,
    pub family: &'static str,
}



// Add get_metadata to Vegetable
impl Vegetable {
    pub fn get_metadata(&self) -> VegMeta {
        match self {
            Vegetable::AoiFamily(AoiFamily::Aoi(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },
            Vegetable::AoiFamily(AoiFamily::Shinanoki(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },
            Vegetable::SuirenFamily(SuirenFamily::Suiren(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::SuirenFamily(SuirenFamily::Hagoromomo(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::SumireFamily(SumireFamily::Uri(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::SeriFamily(SeriFamily::Seri(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::SeriFamily(SeriFamily::Ukogi(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::TadeFamily(TadeFamily::Tade(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NadeshikoFamily(NadeshikoFamily::Suberihiyu(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NadeshikoFamily(NadeshikoFamily::Tsuruna(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NadeshikoFamily(NadeshikoFamily::Tsurumurasaki(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NadeshikoFamily(NadeshikoFamily::Hiyu(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::BaraFamily(BaraFamily::Bara(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::FuurosouFamily(FuurosouFamily::Nouzenharen(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::FutomomoFamily(FutomomoFamily::Hishi(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::MameFamily(MameFamily::Mame(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::MukurojiFamily(MukurojiFamily::Mikan(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::KikuFamily(KikuFamily::Kiku(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::GomanohagusaFamily(GomanohagusaFamily::Goma(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::ShisoFamily(ShisoFamily::Shiso(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::ShisoFamily(ShisoFamily::Murasaki(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NasuFamily(NasuFamily::Nasu(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::NasuFamily(NasuFamily::Hirugao(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::OmodakaFamily(OmodakaFamily::Omodaka(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Kayatsurigusa(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },
            
            Vegetable::SatoimoFamily(SatoimoFamily::Satoimo(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::YuriFamily(YuriFamily::Yuri(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },
            
            Vegetable::YuriFamily(YuriFamily::Yamanoimo(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::YuriFamily(YuriFamily::Ayame(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::ShougaFamily(ShougaFamily::Shouga(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },
            
            Vegetable::RanFamily(RanFamily::Ran(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::ShidaFamily(ShidaFamily::Uraboshi(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::ShidaFamily(ShidaFamily::Kijinooshida(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::ShidaFamily(ShidaFamily::Zenmai(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::TokusaFamily(TokusaFamily::Tokusa(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },

            Vegetable::KikurageFamily(KikurageFamily::Kikurage(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },


            Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },


            Vegetable::HaratakeFamily(HaratakeFamily::Haratake(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },


            Vegetable::HaratakeFamily(HaratakeFamily::Benitake(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },


            Vegetable::HaratakeFamily(HaratakeFamily::Moegitake(genus)) => VegMeta {
                name: genus.name(),
                genus: genus.genus(),
                family: genus.family(),
            },


            
        }
    }
}



// Static lookup table
pub static VEGETABLE_LOOKUP: Lazy<HashMap<&'static str, Vegetable>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Aoi
    map.insert(AoiGenus::Okura.name(), Vegetable::AoiFamily(AoiFamily::Aoi(AoiGenus::Okura)));
    map.insert(AoiGenus::Roselle.name(), Vegetable::AoiFamily(AoiFamily::Aoi(AoiGenus::Roselle)));
    map.insert(AoiGenus::Okanori.name(), Vegetable::AoiFamily(AoiFamily::Aoi(AoiGenus::Okanori)));
    map.insert(AoiGenus::Zeniaoi.name(), Vegetable::AoiFamily(AoiFamily::Aoi(AoiGenus::Zeniaoi)));
    map.insert(AoiGenus::Tororoaoi.name(), Vegetable::AoiFamily(AoiFamily::Aoi(AoiGenus::Tororoaoi)));

    // Shinanoki
    map.insert(ShinanokiGenus::Moroheiya.name(), Vegetable::AoiFamily(AoiFamily::Shinanoki(ShinanokiGenus::Moroheiya)));

    // Suiren
    map.insert(SuirenGenus::Hasu.name(), Vegetable::SuirenFamily(SuirenFamily::Suiren(SuirenGenus::Hasu)));

    // Hagoromomo
    map.insert(HagoromomoGenus::Junsai.name(), Vegetable::SuirenFamily(SuirenFamily::Hagoromomo(HagoromomoGenus::Junsai)));

    // Uri
    map.insert(UriGenus::Uintameron.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Uintameron)));
    map.insert(UriGenus::Uintameron.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Uintameron)));
    map.insert(UriGenus::Kantaropu.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Kantaropu)));
    map.insert(UriGenus::Kyuri.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Kyuri)));
    map.insert(UriGenus::Kurodanekabocha.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Kurodanekabocha)));
    map.insert(UriGenus::Zasshukabocha.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Zasshukabocha)));
    map.insert(UriGenus::Shirouri.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Shirouri)));
    map.insert(UriGenus::Suika.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Suika)));
    map.insert(UriGenus::Zukkiini.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Zukkiini)));
    map.insert(UriGenus::Seiyoukabocha.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Seiyoukabocha)));
    map.insert(UriGenus::TsunoMeron.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::TsunoMeron)));
    map.insert(UriGenus::Tougan.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Tougan)));
    map.insert(UriGenus::Tokadohechima.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Tokadohechima)));
    map.insert(UriGenus::Nigauri.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Nigauri)));
    map.insert(UriGenus::Nihonkabocha.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Nihonkabocha)));
    map.insert(UriGenus::Nettomeron.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Nettomeron)));
    map.insert(UriGenus::Hayatouri.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Hayatouri)));
    map.insert(UriGenus::Hyoutan.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Hyoutan)));
    map.insert(UriGenus::Hechima.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Hechima)));
    map.insert(UriGenus::Hebiuri.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Hebiuri)));
    map.insert(UriGenus::Pepokabocha.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Pepokabocha)));
    map.insert(UriGenus::Makuwauri.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Makuwauri)));
    map.insert(UriGenus::Meron.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Meron)));
    map.insert(UriGenus::Yuugao.name(), Vegetable::SumireFamily(SumireFamily::Uri(UriGenus::Yuugao)));

    // Ukogi
    map.insert(UkogiGenus::Udo.name(), Vegetable::SeriFamily(SeriFamily::Ukogi(UkogiGenus::Udo)));
    map.insert(UkogiGenus::Taranoki.name(), Vegetable::SeriFamily(SeriFamily::Ukogi(UkogiGenus::Taranoki)));

    // Seri
    map.insert(SeriGenus::Ashitaba.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Ashitaba)));
    map.insert(SeriGenus::ItarianPaseri.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::ItarianPaseri)));
    map.insert(SeriGenus::Koriandaa.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Koriandaa)));
    map.insert(SeriGenus::SuupuSerori.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::SuupuSerori)));
    map.insert(SeriGenus::SuupuMinto.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::SuupuMinto)));
    map.insert(SeriGenus::Seri.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Seri)));
    map.insert(SeriGenus::Serori.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Serori)));
    map.insert(SeriGenus::Chaabiru.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Chaabiru)));
    map.insert(SeriGenus::Diru.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Diru)));
    map.insert(SeriGenus::Ninjin.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Ninjin)));
    map.insert(SeriGenus::Paasunippu.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Paasunippu)));
    map.insert(SeriGenus::Paseri.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Paseri)));
    map.insert(SeriGenus::Hamaboufuu.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Hamaboufuu)));
    map.insert(SeriGenus::Fennneru.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Fennneru)));
    map.insert(SeriGenus::Mitsuba.name(), Vegetable::SeriFamily(SeriFamily::Seri(SeriGenus::Mitsuba)));

    // Tade
    map.insert(TadeGenus::Yanagitade.name(), Vegetable::TadeFamily(TadeFamily::Tade(TadeGenus::Yanagitade)));
    map.insert(TadeGenus::Aitade.name(), Vegetable::TadeFamily(TadeFamily::Tade(TadeGenus::Aitade)));
    map.insert(TadeGenus::Rubaabu.name(), Vegetable::TadeFamily(TadeFamily::Tade(TadeGenus::Rubaabu)));

    // Akaza
    map.insert(AkazaGenus::Okahijiki.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Okahijiki)));
    map.insert(AkazaGenus::Teeburubiito.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Teeburubiito)));
    map.insert(AkazaGenus::Houkigi.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Houkigi)));
    map.insert(AkazaGenus::Fudansou.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Fudansou)));
    map.insert(AkazaGenus::Hourensou.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Hourensou)));
    map.insert(AkazaGenus::Matsuna.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Matsuna)));
    map.insert(AkazaGenus::Yamahourensou.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Akaza(AkazaGenus::Yamahourensou)));

    // Suberihiyu
    map.insert(SuberihiyuGenus::Tachisuberihiyu.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Suberihiyu(SuberihiyuGenus::Tachisuberihiyu)));

    // Tsuruna
    map.insert(TsurunaGenus::Tsuruna.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Tsuruna(TsurunaGenus::Tsuruna)));

    // Tsurumurasaki
    map.insert(TsurumurasakiGenus::Tsurumurasaki.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Tsurumurasaki(TsurumurasakiGenus::Tsurumurasaki)));

    // Hiyu
    map.insert(HiyuGenus::Amaransasu.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Hiyu(HiyuGenus::Amaransasu)));
    map.insert(HiyuGenus::Hiyu.name(), Vegetable::NadeshikoFamily(NadeshikoFamily::Hiyu(HiyuGenus::Hiyu)));

    // Bara
    map.insert(BaraGenus::Ichigo.name(), Vegetable::BaraFamily(BaraFamily::Bara(BaraGenus::Ichigo)));
    map.insert(BaraGenus::Saradabaanetto.name(), Vegetable::BaraFamily(BaraFamily::Bara(BaraGenus::Saradabaanetto)));
    map.insert(BaraGenus::WairudoSutoroberii.name(), Vegetable::BaraFamily(BaraFamily::Bara(BaraGenus::WairudoSutoroberii)));

    // Aburana
    map.insert(AburanaGenus::Aburana.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Aburana)));
    map.insert(AburanaGenus::Oosakashirona.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Oosakashirona)));
    map.insert(AburanaGenus::Kiren.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Kiren)));
    map.insert(AburanaGenus::Kabu.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Kabu)));
    map.insert(AburanaGenus::Karashina.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Karashina)));
    map.insert(AburanaGenus::Karifurawaa.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Karifurawaa)));
    map.insert(AburanaGenus::Kyabetsu.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Kyabetsu)));
    map.insert(AburanaGenus::Kureson.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Kureson)));
    map.insert(AburanaGenus::Keeru.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Keeru)));
    map.insert(AburanaGenus::Koorurabi.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Koorurabi)));
    map.insert(AburanaGenus::Kousaitai.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Kousaitai)));
    map.insert(AburanaGenus::Koshousou.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Koshousou)));
    map.insert(AburanaGenus::Komatsuna.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Komatsuna)));
    map.insert(AburanaGenus::Sugukina.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Sugukina)));
    map.insert(AburanaGenus::Tasai.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Tasai)));
    map.insert(AburanaGenus::Daikon.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Daikon)));
    map.insert(AburanaGenus::Taisai.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Taisai)));
    map.insert(AburanaGenus::Daishinsai.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Daishinsai)));
    map.insert(AburanaGenus::Takana.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Takana)));
    map.insert(AburanaGenus::Tanikutakana.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Tanikutakana)));
    map.insert(AburanaGenus::Chingensai.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Chingensai)));
    map.insert(AburanaGenus::Nozawana.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Nozawana)));
    map.insert(AburanaGenus::Hakusai.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Hakusai)));
    map.insert(AburanaGenus::Hukuran.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Hukuran)));
    map.insert(AburanaGenus::Hatsukadikon.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Hatsukadikon)));
    map.insert(AburanaGenus::Hamana.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Hamana)));
    map.insert(AburanaGenus::PuchiVeeru.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::PuchiVeeru)));
    map.insert(AburanaGenus::Burokkorii.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Burokkorii)));
    map.insert(AburanaGenus::Mizuna.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Mizuna)));
    map.insert(AburanaGenus::Mibuna.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Mibuna)));
    map.insert(AburanaGenus::Mekyabetsu.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Mekyabetsu)));
    map.insert(AburanaGenus::Rutabaga.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Rutabaga)));
    map.insert(AburanaGenus::Rukkora.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Rukkora)));
    map.insert(AburanaGenus::Wasabi.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Wasabi)));
    map.insert(AburanaGenus::Wasabidakon.name(), Vegetable::FuuchousouFamily(FuuchousouFamily::Aburana(AburanaGenus::Wasabidakon)));

    // Nouzenharen
    map.insert(NouzenharenGenus::Kinrenka.name(), Vegetable::FuurosouFamily(FuurosouFamily::Nouzenharen(NouzenharenGenus::Kinrenka)));
    
    // Hishi
    map.insert(HishiGenus::Hishi.name(), Vegetable::FutomomoFamily(FutomomoFamily::Hishi(HishiGenus::Hishi)));

    // Mame 
    map.insert(MameGenus::Azuki.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Azuki)));
    map.insert(MameGenus::Amerikahodoimo.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Amerikahodoimo)));
    map.insert(MameGenus::Ingenmame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Ingenmame)));
    map.insert(MameGenus::Endou.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Endou)));
    map.insert(MameGenus::Sasage.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Sasage)));
    map.insert(MameGenus::Shikakumame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Shikakumame)));
    map.insert(MameGenus::Juurokusasage.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Juurokusasage)));
    map.insert(MameGenus::Soramame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Soramame)));
    map.insert(MameGenus::Daizu.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Daizu)));
    map.insert(MameGenus::Tachinatamame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Tachinatamame)));
    map.insert(MameGenus::Choumame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Choumame)));
    map.insert(MameGenus::Natamame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Natamame)));
    map.insert(MameGenus::Nantenhagi.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Nantenhagi)));
    map.insert(MameGenus::Hasshoumame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Hasshoumame)));
    map.insert(MameGenus::Hiyokomame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Hiyokomame)));
    map.insert(MameGenus::Hiramame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Hiramame)));
    map.insert(MameGenus::Fujimame.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Fujimame)));
    map.insert(MameGenus::Benibanaingen.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Benibanaingen)));
    map.insert(MameGenus::Raimabiin.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Raimabiin)));
    map.insert(MameGenus::Rakkasei.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Rakkasei)));
    map.insert(MameGenus::Ryokutou.name(), Vegetable::MameFamily(MameFamily::Mame(MameGenus::Ryokutou)));

    // Mikan
    map.insert(MikanGenus::Sanshou.name(), Vegetable::MukurojiFamily(MukurojiFamily::Mikan(MikanGenus::Sanshou)));
    map.insert(MikanGenus::Henruuda.name(), Vegetable::MukurojiFamily(MukurojiFamily::Mikan(MikanGenus::Henruuda)));

    // Kiku
    map.insert(KikuGenus::Aatichooku.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Aatichooku)));
    map.insert(KikuGenus::Endaibu.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Endaibu)));
    map.insert(KikuGenus::Orandasennichi.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Orandasennichi)));
    map.insert(KikuGenus::Kakijisha.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Kakijisha)));
    map.insert(KikuGenus::Kamomiiru.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Kamomiiru)));
    map.insert(KikuGenus::Karudon.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Karudon)));
    map.insert(KikuGenus::Kareepuranto.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Kareepuranto)));
    map.insert(KikuGenus::Kiku.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Kiku)));
    map.insert(KikuGenus::Kikuimo.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Kikuimo)));
    map.insert(KikuGenus::Gobou.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Gobou)));
    map.insert(KikuGenus::Sarushifai.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Sarushifai)));
    map.insert(KikuGenus::Sanchu.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Sanchu)));
    map.insert(KikuGenus::Shungiku.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Shungiku)));
    map.insert(KikuGenus::Shokuyoutanpopo.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Shokuyoutanpopo)));
    map.insert(KikuGenus::Suizenjina.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Suizenjina)));
    map.insert(KikuGenus::Sutebia.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Sutebia)));
    map.insert(KikuGenus::Tachijisha.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Tachijisha)));
    map.insert(KikuGenus::Chikori.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Chikori)));
    map.insert(KikuGenus::Chisha.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Chisha)));
    map.insert(KikuGenus::Tsuwabuki.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Tsuwabuki)));
    map.insert(KikuGenus::Torebisu.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Torebisu)));
    map.insert(KikuGenus::Fuki.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Fuki)));
    map.insert(KikuGenus::Yamagobou.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Yamagobou)));
    map.insert(KikuGenus::Riifuretasu.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Riifuretasu)));
    map.insert(KikuGenus::Retasu.name(), Vegetable::KikuFamily(KikuFamily::Kiku(KikuGenus::Retasu)));

    // Goma
    map.insert(GomaGenus::Goma.name(), Vegetable::GomanohagusaFamily(GomanohagusaFamily::Goma(GomaGenus::Goma)));

    // Shiso
    map.insert(ShisoGenus::Appuruminto.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Appuruminto)));
    map.insert(ShisoGenus::Uintaasaborii.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Uintaasaborii)));
    map.insert(ShisoGenus::Egoma.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Egoma)));
    map.insert(ShisoGenus::Oregano.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Oregano)));
    map.insert(ShisoGenus::Shiso.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Shiso)));
    map.insert(ShisoGenus::Supearaminto.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Supearaminto)));
    map.insert(ShisoGenus::Seiji.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Seiji)));
    map.insert(ShisoGenus::Taimu.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Taimu)));
    map.insert(ShisoGenus::Chorogi.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Chorogi)));
    map.insert(ShisoGenus::Numahakka.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Numahakka)));
    map.insert(ShisoGenus::Painappuruminto.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Painappuruminto)));
    map.insert(ShisoGenus::Hakka.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Hakka)));
    map.insert(ShisoGenus::Bajiru.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Bajiru)));
    map.insert(ShisoGenus::Pepaminto.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Pepaminto)));
    map.insert(ShisoGenus::Majoramu.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Majoramu)));
    map.insert(ShisoGenus::Rabendaa.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Rabendaa)));
    map.insert(ShisoGenus::Remontaimu.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Remontaimu)));
    map.insert(ShisoGenus::Remonbaamu.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Remonbaamu)));
    map.insert(ShisoGenus::Roozumarii.name(), Vegetable::ShisoFamily(ShisoFamily::Shiso(ShisoGenus::Roozumarii)));

    // Murasaki
    map.insert(MurasakiGenus::Konfurii.name(), Vegetable::ShisoFamily(ShisoFamily::Murasaki(MurasakiGenus::Konfurii)));

    // Nasu
    map.insert(NasuGenus::Karantotomato.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Karantotomato)));
    map.insert(NasuGenus::Shishitougarashi.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Shishitougarashi)));
    map.insert(NasuGenus::Jagaimo.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Jagaimo)));
    map.insert(NasuGenus::Shokuyouhoozuki.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Shokuyouhoozuki)));
    map.insert(NasuGenus::Shironasu.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Shironasu)));
    map.insert(NasuGenus::Tougarashi.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Tougarashi)));
    map.insert(NasuGenus::Tomato.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Tomato)));
    map.insert(NasuGenus::Nasu.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Nasu)));
    map.insert(NasuGenus::Piiman.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Piiman)));
    map.insert(NasuGenus::Hiranasu.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Hiranasu)));
    map.insert(NasuGenus::Pepiino.name(), Vegetable::NasuFamily(NasuFamily::Nasu(NasuGenus::Pepiino)));

    // Hirugao
    map.insert(HirugaoGenus::Satsumaimo.name(), Vegetable::NasuFamily(NasuFamily::Hirugao(HirugaoGenus::Satsumaimo)));
    map.insert(HirugaoGenus::Yousai.name(), Vegetable::NasuFamily(NasuFamily::Hirugao(HirugaoGenus::Yousai)));

    // Omodaka
    map.insert(OmodakaGenus::Kuwai.name(), Vegetable::OmodakaFamily(OmodakaFamily::Omodaka(OmodakaGenus::Kuwai)));
    map.insert(OmodakaGenus::Suitakuwai.name(), Vegetable::OmodakaFamily(OmodakaFamily::Omodaka(OmodakaGenus::Suitakuwai)));

    // Ine
    map.insert(IneGenus::Toumorokoshi.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(IneGenus::Toumorokoshi)));
    map.insert(IneGenus::Nemagaridake.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(IneGenus::Nemagaridake)));
    map.insert(IneGenus::Hachiku.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(IneGenus::Hachiku)));
    map.insert(IneGenus::Makomo.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(IneGenus::Makomo)));
    map.insert(IneGenus::Madake.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(IneGenus::Madake)));
    map.insert(IneGenus::Mousouchiku.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Ine(IneGenus::Mousouchiku)));

    // Kayatsurigusa
    map.insert(KayatsurigusaGenus::Ookuroguwai.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Kayatsurigusa(KayatsurigusaGenus::Ookuroguwai)));
    map.insert(KayatsurigusaGenus::Kuroguwai.name(), Vegetable::KayatsurigusaFamily(KayatsurigusaFamily::Kayatsurigusa(KayatsurigusaGenus::Kuroguwai)));

    // Satoimo
    map.insert(SatoimoGenus::Konnyaku.name(), Vegetable::SatoimoFamily(SatoimoFamily::Satoimo(SatoimoGenus::Konnyaku)));
    map.insert(SatoimoGenus::Satoimo.name(), Vegetable::SatoimoFamily(SatoimoFamily::Satoimo(SatoimoGenus::Satoimo)));
    map.insert(SatoimoGenus::Hasuimo.name(), Vegetable::SatoimoFamily(SatoimoFamily::Satoimo(SatoimoGenus::Hasuimo)));

    // Yuri
    map.insert(YuriGenus::Asatsuki.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Asatsuki)));
    map.insert(YuriGenus::Asparagus.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Asparagus)));
    map.insert(YuriGenus::Oniyuri.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Oniyuri)));
    map.insert(YuriGenus::Katakuri.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Katakuri)));
    map.insert(YuriGenus::Kanzou.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Kanzou)));
    map.insert(YuriGenus::Giboushi.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Giboushi)));
    map.insert(YuriGenus::Gyoujaninniku.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Gyoujaninniku)));
    map.insert(YuriGenus::Kooniyuri.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Kooniyuri)));
    map.insert(YuriGenus::Shallot.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Shallot)));
    map.insert(YuriGenus::Tamanegi.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Tamanegi)));
    map.insert(YuriGenus::Chive.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Chive)));
    map.insert(YuriGenus::Nira.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Nira)));
    map.insert(YuriGenus::Ninniku.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Ninniku)));
    map.insert(YuriGenus::Negi.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Negi)));
    map.insert(YuriGenus::Hananira.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Hananira)));
    map.insert(YuriGenus::Yaguranegi.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Yaguranegi)));
    map.insert(YuriGenus::Yamayuri.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Yamayuri)));
    map.insert(YuriGenus::Rakkyo.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Rakkyo)));
    map.insert(YuriGenus::Riiki.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Riiki)));
    map.insert(YuriGenus::Wakegi.name(), Vegetable::YuriFamily(YuriFamily::Yuri(YuriGenus::Wakegi)));

    // Yamanoimo
    map.insert(YamanoimoGenus::Daisho.name(), Vegetable::YuriFamily(YuriFamily::Yamanoimo(YamanoimoGenus::Daisho)));
    map.insert(YamanoimoGenus::Nagaimo.name(), Vegetable::YuriFamily(YuriFamily::Yamanoimo(YamanoimoGenus::Nagaimo)));
    map.insert(YamanoimoGenus::Yamanoimo.name(), Vegetable::YuriFamily(YuriFamily::Yamanoimo(YamanoimoGenus::Yamanoimo)));

    // Ayame
    map.insert(AyameGenus::Safuran.name(), Vegetable::YuriFamily(YuriFamily::Ayame(AyameGenus::Safuran)));

    // Shouga
    map.insert(ShougaGenus::Shouga.name(), Vegetable::ShougaFamily(ShougaFamily::Shouga(ShougaGenus::Shouga)));
    map.insert(ShougaGenus::Shouga.name(), Vegetable::ShougaFamily(ShougaFamily::Shouga(ShougaGenus::Myouga)));

    // Ran
    map.insert(RanGenus::Shunran.name(), Vegetable::RanFamily(RanFamily::Ran(RanGenus::Shunran)));

    // Uraboshi
    map.insert(UraboshiGenus::Warabi.name(), Vegetable::ShidaFamily(ShidaFamily::Uraboshi(UraboshiGenus::Warabi)));

    // Kijinooshida
    map.insert(KijinooshidaGenus::Yamasotetsu.name(), Vegetable::ShidaFamily(ShidaFamily::Kijinooshida(KijinooshidaGenus::Yamasotetsu)));

    // Zenmai
    map.insert(ZenmaiGenus::Zenmai.name(), Vegetable::ShidaFamily(ShidaFamily::Zenmai(ZenmaiGenus::Zenmai)));

    // Tokusa
    map.insert(TokusaGenus::Tsukushi.name(), Vegetable::TokusaFamily(TokusaFamily::Tokusa(TokusaGenus::Tsukushi)));

    // Kikurage
    map.insert(KikurageGenus::Kikurage.name(), Vegetable::KikurageFamily(KikurageFamily::Kikurage(KikurageGenus::Kikurage)));

    // Kishimeji
    map.insert(KishimejiGenus::Enokitake.name(), Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(KishimejiGenus::Enokitake)));
    map.insert(KishimejiGenus::Shiitake.name(), Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(KishimejiGenus::Shiitake)));
    map.insert(KishimejiGenus::Shakashimeji.name(), Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(KishimejiGenus::Shakashimeji)));
    map.insert(KishimejiGenus::Hiratake.name(), Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(KishimejiGenus::Hiratake)));
    map.insert(KishimejiGenus::Honshimeji.name(), Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(KishimejiGenus::Honshimeji)));
    map.insert(KishimejiGenus::Matsutake.name(), Vegetable::HaratakeFamily(HaratakeFamily::Kishimeji(KishimejiGenus::Matsutake)));

    // Haratake
    map.insert(HaratakeGenus::Masshuruumu.name(), Vegetable::HaratakeFamily(HaratakeFamily::Haratake(HaratakeGenus::Masshuruumu)));

    // Benitake
    map.insert(BenitakeGenus::Hatsutake.name(), Vegetable::HaratakeFamily(HaratakeFamily::Benitake(BenitakeGenus::Hatsutake)));

    // Moegitake
    map.insert(MoegitakeGenus::Nameko.name(), Vegetable::HaratakeFamily(HaratakeFamily::Moegitake(MoegitakeGenus::Nameko)));


    map
});