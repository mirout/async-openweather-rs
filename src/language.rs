pub enum Lang {
    Afrikaans,
    Albanian,
    Arabic,
    Azerbaijani,
    Bulgarian,
    Catalan,
    Czech,
    Danish,
    German,
    Greek,
    English,
    Basque,
    Persian,
    Finish,
    French,
    Galician,
    Hebrew,
    Hindi,
    Croatian,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Latvian,
    Lithuanian,
    Macedonian,
    Norwegian,
    Dutch,
    Polish,
    Portuguese,
    PortuguesBrasil,
    Romanian,
    Russian,
    Swedish,
    Slovak,
    Slovenian,
    Spanish,
    Serbian,
    Thai,
    Turkish,
    Ukrainian,
    Vietnamese,
    ChineseSimplified,
    ChineseTraditional,
    Zulu,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::English
    }
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            Lang::Afrikaans => { "af".to_string() }
            Lang::Albanian => { "al".to_string() }
            Lang::Arabic => { "ar".to_string() }
            Lang::Azerbaijani => { "az".to_string() }
            Lang::Bulgarian => { "bg".to_string() }
            Lang::Catalan => { "ca".to_string() }
            Lang::Czech => { "cz".to_string() }
            Lang::Danish => { "da".to_string() }
            Lang::German => { "de".to_string() }
            Lang::Greek => { "el".to_string() }
            Lang::English => { "en".to_string() }
            Lang::Basque => { "eu".to_string() }
            Lang::Persian => { "fa".to_string() }
            Lang::Finish => { "fi".to_string() }
            Lang::French => { "fr".to_string() }
            Lang::Galician => { "gl".to_string() }
            Lang::Hebrew => { "he".to_string() }
            Lang::Hindi => { "hi".to_string() }
            Lang::Croatian => { "hr".to_string() }
            Lang::Hungarian => { "hu".to_string() }
            Lang::Indonesian => { "id".to_string() }
            Lang::Italian => { "it".to_string() }
            Lang::Japanese => { "ja".to_string() }
            Lang::Korean => { "kr".to_string() }
            Lang::Latvian => { "la".to_string() }
            Lang::Lithuanian => { "lt".to_string() }
            Lang::Macedonian => { "mk".to_string() }
            Lang::Norwegian => { "no".to_string() }
            Lang::Dutch => { "nl".to_string() }
            Lang::Polish => { "pl".to_string() }
            Lang::Portuguese => { "pt".to_string() }
            Lang::PortuguesBrasil => { "pt_br".to_string() }
            Lang::Romanian => { "ro".to_string() }
            Lang::Russian => { "ru".to_string() }
            Lang::Swedish => { "sv".to_string() }
            Lang::Slovak => { "sk".to_string() }
            Lang::Slovenian => { "sl".to_string() }
            Lang::Spanish => { "sp".to_string() }
            Lang::Serbian => { "sr".to_string() }
            Lang::Thai => { "th".to_string() }
            Lang::Turkish => { "tr".to_string() }
            Lang::Ukrainian => { "ua".to_string() }
            Lang::Vietnamese => { "vi".to_string() }
            Lang::ChineseSimplified => { "zh_ch".to_string() }
            Lang::ChineseTraditional => { "zh_tw".to_string() }
            Lang::Zulu => { "zu".to_string() }
        }
    }
}