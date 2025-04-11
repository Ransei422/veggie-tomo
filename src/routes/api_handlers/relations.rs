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
    // TadeFamily(TadeFamily),
    // NadeshikoFamily(NadeshikoFamily),
    // BaraFamily(BaraFamily),
    // FuuchousouFamily(FuuchousouFamily),
    // FuurosouFamily(FuurosouFamily),
    // FutomomoFamily(FutomomoFamily),
    // MameFamily(MameFamily),
    // MukuroshiFamily(MukuroshiFamily),
    // KikuFamily(KikuFamily),
    // GomanohagusaFamily(GomanohagusaFamily),
    // ShisoFamily(ShisoFamily),
    // NasuFamily(NasuFamily),
    // OmodakaFamily(OmodakaFamily),
    // KayatsurigusaFamily(KayatsurigusaFamily),
    // SatoimoFamily(SatoimoFamily),
    // ShougaFamily(ShougaFamily),
    // YuriFamily(YuriFamily),
    // RanFamily(RanFamily),
    // ShidaFamily(ShidaFamily),
    // TokusaFamily(TokusaFamily),
    // KikurageFamily(KikurageFamily),
    // HaratakeFamily(HaratakeFamily),
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



    map
});