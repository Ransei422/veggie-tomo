use std::collections::HashMap;
use once_cell::sync::Lazy;

//
// === Enums for taxonomy ===
//

#[derive(Debug, Clone)]
pub enum Vegetable {
    AoiFamily(AoiFamily),
    SuirenFamily(SuirenFamily),
    // SumireFamily(SumireFamily),
    // SeriFamily(SeriFamily),
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


// AoiFamily--------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum AoiFamily {
    Aoi(AoiGenus),
    Shinanoki(ShinanokiGenus),
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum AoiGenus {
    Okura,
    Roselle,
    Okanori,
    Zeniaoi,
    Tororoaoi,
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ShinanokiGenus {
    Moroheiya,
}

// SuirenFamily--------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum SuirenFamily {
    Suiren(SuirenGenus),
    Hagoromomo(HagoromomoGenus),
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum SuirenGenus {
    Hasu,
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum HagoromomoGenus {
    Junsai,
}































//
// === Trait for metadata ===
//

pub trait VegMetadata {
    fn name(&self) -> &'static str;
    fn genus(&self) -> &'static str;
    fn family(&self) -> &'static str;
}

impl VegMetadata for AoiGenus {
    fn name(&self) -> &'static str {
        match self {
            AoiGenus::Okura => "オクラ",
            AoiGenus::Roselle => "ローゼル",
            AoiGenus::Okanori => "おかのり",
            AoiGenus::Zeniaoi => "ぜにあおい",
            AoiGenus::Tororoaoi => "とろろあおい",
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
    fn name(&self) -> &'static str { "はす" }
    fn genus(&self) -> &'static str { "スイレン" }
    fn family(&self) -> &'static str { "スイレン" }
}

impl VegMetadata for HagoromomoGenus {
    fn name(&self) -> &'static str { "じゅんさい" }
    fn genus(&self) -> &'static str { "ハゴロモモ" }
    fn family(&self) -> &'static str { "スイレン" }
}














//
// === Struct for returned metadata ===
//

#[derive(Debug)]
pub struct VegMeta {
    pub name: &'static str,
    pub genus: &'static str,
    pub family: &'static str,
}

//
// === Add get_metadata to Vegetable ===
//

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
        }
    }
}

//
// === Static lookup table ===
//

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
   


    map
});

//
// === Search by name ===
//

pub fn search_by_name(name: &str) -> Option<Vegetable> {
    VEGETABLE_LOOKUP.get(name).cloned()
}
