use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub enum CurrencyCode {
    /// Polish Zloty (Table A and C)
    PLN,
    /// United States Dollar (Table A and C)
    USD,
    /// Euro (Table A and C)
    EUR,
    /// British Pound Sterling (Table A and C)
    GBP,
    /// Swiss Franc (Table A and C)
    CHF,
    /// Australian Dollar (Table A and C)
    AUD,
    /// Canadian Dollar (Table A and C)
    CAD,
    /// Hungarian Forint (Table A and C)
    HUF,
    /// Japanese Yen (Table A and C)
    JPY,
    /// Czech Koruna (Table A and C)
    CZK,
    /// Danish Krone (Table A and C)
    DKK,
    /// Norwegian Krone (Table A and C)
    NOK,
    /// Swedish Krona (Table A and C)
    SEK,
    /// Special Drawing Rights (IMF) (Table A and C)
    XDR,

    /// Thai Baht (Table A)
    THB,
    /// Hong Kong Dollar (Table A)
    HKD,
    /// New Zealand Dollar (Table A)
    NZD,
    /// Singapore Dollar (Table A)
    SGD,
    /// Ukrainian Hryvnia (Table A)
    UAH,
    /// Icelandic Króna (Table A)
    ISK,
    /// Romanian Leu (Table A)
    RON,
    /// Bulgarian Lev (Table A)
    BGN,
    /// Turkish Lira (Table A)
    TRY,
    /// Israeli New Shekel (Table A)
    ILS,
    /// Chilean Peso (Table A)
    CLP,
    /// Philippine Peso (Table A)
    PHP,
    /// Mexican Peso (Table A)
    MXN,
    /// South African Rand (Table A)
    ZAR,
    /// Brazilian Real (Table A)
    BRL,
    /// Malaysian Ringgit (Table A)
    MYR,
    /// Indonesian Rupiah (Table A)
    IDR,
    /// Indian Rupee (Table A)
    INR,
    /// South Korean Won (Table A)
    KRW,
    /// Chinese Yuan Renminbi (Table A)
    CNY,

    /// Afghan Afghani (Table B)
    AFN,
    /// Malagasy Ariary (Table B)
    MGA,
    /// Venezuelan Bolívar Soberano (Table B)
    VES,
    /// Bolivian Boliviano (Table B)
    BOB,
    /// Costa Rican Colón (Table B)
    CRC,
    /// Salvadoran Colón (Table B)
    SVC,
    /// Nicaraguan Córdoba Oro (Table B)
    NIO,
    /// Gambian Dalasi (Table B)
    GMD,
    /// Macedonian Denar (Table B)
    MKD,
    /// Algerian Dinar (Table B)
    DZD,
    /// Bahraini Dinar (Table B)
    BHD,
    /// Iraqi Dinar (Table B)
    IQD,
    /// Jordanian Dinar (Table B)
    JOD,
    /// Kuwaiti Dinar (Table B)
    KWD,
    /// Libyan Dinar (Table B)
    LYD,
    /// Serbian Dinar (Table B)
    RSD,
    /// Tunisian Dinar (Table B)
    TND,
    /// Moroccan Dirham (Table B)
    MAD,
    /// UAE Dirham (Table B)
    AED,
    /// São Tomé and Príncipe Dobra (Table B)
    STN,
    /// Bahamian Dollar (Table B)
    BSD,
    /// Barbadian Dollar (Table B)
    BBD,
    /// Belize Dollar (Table B)
    BZD,
    /// Brunei Dollar (Table B)
    BND,
    /// Fijian Dollar (Table B)
    FJD,
    /// Guyanese Dollar (Table B)
    GYD,
    /// Jamaican Dollar (Table B)
    JMD,
    /// Liberian Dollar (Table B)
    LRD,
    /// Namibian Dollar (Table B)
    NAD,
    /// Surinamese Dollar (Table B)
    SRD,
    /// Trinidad and Tobago Dollar (Table B)
    TTD,
    /// East Caribbean Dollar (Table B)
    XCD,
    /// Solomon Islands Dollar (Table B)
    SBD,
    /// Vietnamese Dong (Table B)
    VND,
    /// Armenian Dram (Table B)
    AMD,
    /// Cape Verdean Escudo (Table B)
    CVE,
    /// Aruban Florin (Table B)
    AWG,
    /// Burundian Franc (Table B)
    BIF,
    /// CFA Franc BCEAO (Table B)
    XOF,
    /// CFA Franc BEAC (Table B)
    XAF,
    /// CFP Franc (Table B)
    XPF,
    /// Djiboutian Franc (Table B)
    DJF,
    /// Guinean Franc (Table B)
    GNF,
    /// Comorian Franc (Table B)
    KMF,
    /// Congolese Franc (Table B)
    CDF,
    /// Rwandan Franc (Table B)
    RWF,
    /// Egyptian Pound (Table B)
    EGP,
    /// Gibraltar Pound (Table B)
    GIP,
    /// Lebanese Pound (Table B)
    LBP,
    /// South Sudanese Pound (Table B)
    SSP,
    /// Sudanese Pound (Table B)
    SDG,
    /// Syrian Pound (Table B)
    SYP,
    /// Ghanaian Cedi (Table B)
    GHS,
    /// Haitian Gourde (Table B)
    HTG,
    /// Paraguayan Guaraní (Table B)
    PYG,
    /// Caribbean Guilder (Table B)
    XCG,
    /// Papua New Guinean Kina (Table B)
    PGK,
    /// Lao Kip (Table B)
    LAK,
    /// Malawian Kwacha (Table B)
    MWK,
    /// Zambian Kwacha (Table B)
    ZMW,
    /// Angolan Kwanza (Table B)
    AOA,
    /// Myanmar Kyat (Table B)
    MMK,
    /// Georgian Lari (Table B)
    GEL,
    /// Moldovan Leu (Table B)
    MDL,
    /// Albanian Lek (Table B)
    ALL,
    /// Honduran Lempira (Table B)
    HNL,
    /// Sierra Leonean Leone (Table B)
    SLE,
    /// Eswatini Lilangeni (Table B)
    SZL,
    /// Lesotho Loti (Table B)
    LSL,
    /// Azerbaijani Manat (Table B)
    AZN,
    /// Mozambican Metical (Table B)
    MZN,
    /// Nigerian Naira (Table B)
    NGN,
    /// Eritrean Nakfa (Table B)
    ERN,
    /// New Taiwan Dollar (Table B)
    TWD,
    /// Turkmenistan Manat (Table B)
    TMT,
    /// Mauritanian Ouguiya (Table B)
    MRU,
    /// Tongan Pa'anga (Table B)
    TOP,
    /// Macanese Pataca (Table B)
    MOP,
    /// Argentine Peso (Table B)
    ARS,
    /// Dominican Peso (Table B)
    DOP,
    /// Colombian Peso (Table B)
    COP,
    /// Cuban Peso (Table B)
    CUP,
    /// Uruguayan Peso (Table B)
    UYU,
    /// Botswana Pula (Table B)
    BWP,
    /// Guatemalan Quetzal (Table B)
    GTQ,
    /// Iranian Rial (Table B)
    IRR,
    /// Yemeni Rial (Table B)
    YER,
    /// Qatari Rial (Table B)
    QAR,
    /// Omani Rial (Table B)
    OMR,
    /// Saudi Riyal (Table B)
    SAR,
    /// Cambodian Riel (Table B)
    KHR,
    /// Belarusian Ruble (Table B)
    BYN,
    /// Russian Ruble (Table B)
    RUB,
    /// Sri Lankan Rupee (Table B)
    LKR,
    /// Maldivian Rufiyaa (Table B)
    MVR,
    /// Mauritian Rupee (Table B)
    MUR,
    /// Nepalese Rupee (Table B)
    NPR,
    /// Pakistani Rupee (Table B)
    PKR,
    /// Seychellois Rupee (Table B)
    SCR,
    /// Peruvian Sol (Table B)
    PEN,
    /// Kyrgyzstani Som (Table B)
    KGS,
    /// Tajikistani Somoni (Table B)
    TJS,
    /// Uzbekistani Sum (Table B)
    UZS,
    /// Kenyan Shilling (Table B)
    KES,
    /// Somali Shilling (Table B)
    SOS,
    /// Tanzanian Shilling (Table B)
    TZS,
    /// Ugandan Shilling (Table B)
    UGX,
    /// Bangladeshi Taka (Table B)
    BDT,
    /// Samoan Tālā (Table B)
    WST,
    /// Kazakhstani Tenge (Table B)
    KZT,
    /// Mongolian Tögrög (Table B)
    MNT,
    /// Vanuatu Vatu (Table B)
    VUV,
    /// Bosnia and Herzegovina Convertible Mark (Table B)
    BAM,
    /// Zimbabwe Gold (Table B)
    ZWG,
    /// Panamanian Balboa (Table B)
    PAB,
    /// Ethiopian Birr (Table B)
    ETB,

    /// Archived/unknown currency code
    Other(String),
}

impl<'de> serde::Deserialize<'de> for CurrencyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "PLN" => Ok(CurrencyCode::PLN),
            "USD" => Ok(CurrencyCode::USD),
            "EUR" => Ok(CurrencyCode::EUR),
            "GBP" => Ok(CurrencyCode::GBP),
            "CHF" => Ok(CurrencyCode::CHF),
            "AUD" => Ok(CurrencyCode::AUD),
            "CAD" => Ok(CurrencyCode::CAD),
            "HUF" => Ok(CurrencyCode::HUF),
            "JPY" => Ok(CurrencyCode::JPY),
            "CZK" => Ok(CurrencyCode::CZK),
            "DKK" => Ok(CurrencyCode::DKK),
            "NOK" => Ok(CurrencyCode::NOK),
            "SEK" => Ok(CurrencyCode::SEK),
            "XDR" => Ok(CurrencyCode::XDR),
            "THB" => Ok(CurrencyCode::THB),
            "HKD" => Ok(CurrencyCode::HKD),
            "NZD" => Ok(CurrencyCode::NZD),
            "SGD" => Ok(CurrencyCode::SGD),
            "UAH" => Ok(CurrencyCode::UAH),
            "ISK" => Ok(CurrencyCode::ISK),
            "RON" => Ok(CurrencyCode::RON),
            "BGN" => Ok(CurrencyCode::BGN),
            "TRY" => Ok(CurrencyCode::TRY),
            "ILS" => Ok(CurrencyCode::ILS),
            "CLP" => Ok(CurrencyCode::CLP),
            "PHP" => Ok(CurrencyCode::PHP),
            "MXN" => Ok(CurrencyCode::MXN),
            "ZAR" => Ok(CurrencyCode::ZAR),
            "BRL" => Ok(CurrencyCode::BRL),
            "MYR" => Ok(CurrencyCode::MYR),
            "IDR" => Ok(CurrencyCode::IDR),
            "INR" => Ok(CurrencyCode::INR),
            "KRW" => Ok(CurrencyCode::KRW),
            "CNY" => Ok(CurrencyCode::CNY),
            "AFN" => Ok(CurrencyCode::AFN),
            "MGA" => Ok(CurrencyCode::MGA),
            "VES" => Ok(CurrencyCode::VES),
            "BOB" => Ok(CurrencyCode::BOB),
            "CRC" => Ok(CurrencyCode::CRC),
            "SVC" => Ok(CurrencyCode::SVC),
            "NIO" => Ok(CurrencyCode::NIO),
            "GMD" => Ok(CurrencyCode::GMD),
            "MKD" => Ok(CurrencyCode::MKD),
            "DZD" => Ok(CurrencyCode::DZD),
            "BHD" => Ok(CurrencyCode::BHD),
            "IQD" => Ok(CurrencyCode::IQD),
            "JOD" => Ok(CurrencyCode::JOD),
            "KWD" => Ok(CurrencyCode::KWD),
            "LYD" => Ok(CurrencyCode::LYD),
            "RSD" => Ok(CurrencyCode::RSD),
            "TND" => Ok(CurrencyCode::TND),
            "MAD" => Ok(CurrencyCode::MAD),
            "AED" => Ok(CurrencyCode::AED),
            "STN" => Ok(CurrencyCode::STN),
            "BSD" => Ok(CurrencyCode::BSD),
            "BBD" => Ok(CurrencyCode::BBD),
            "BZD" => Ok(CurrencyCode::BZD),
            "BND" => Ok(CurrencyCode::BND),
            "FJD" => Ok(CurrencyCode::FJD),
            "GYD" => Ok(CurrencyCode::GYD),
            "JMD" => Ok(CurrencyCode::JMD),
            "LRD" => Ok(CurrencyCode::LRD),
            "NAD" => Ok(CurrencyCode::NAD),
            "SRD" => Ok(CurrencyCode::SRD),
            "TTD" => Ok(CurrencyCode::TTD),
            "XCD" => Ok(CurrencyCode::XCD),
            "SBD" => Ok(CurrencyCode::SBD),
            "VND" => Ok(CurrencyCode::VND),
            "AMD" => Ok(CurrencyCode::AMD),
            "CVE" => Ok(CurrencyCode::CVE),
            "AWG" => Ok(CurrencyCode::AWG),
            "BIF" => Ok(CurrencyCode::BIF),
            "XOF" => Ok(CurrencyCode::XOF),
            "XAF" => Ok(CurrencyCode::XAF),
            "XPF" => Ok(CurrencyCode::XPF),
            "DJF" => Ok(CurrencyCode::DJF),
            "GNF" => Ok(CurrencyCode::GNF),
            "KMF" => Ok(CurrencyCode::KMF),
            "CDF" => Ok(CurrencyCode::CDF),
            "RWF" => Ok(CurrencyCode::RWF),
            "EGP" => Ok(CurrencyCode::EGP),
            "GIP" => Ok(CurrencyCode::GIP),
            "LBP" => Ok(CurrencyCode::LBP),
            "SSP" => Ok(CurrencyCode::SSP),
            "SDG" => Ok(CurrencyCode::SDG),
            "SYP" => Ok(CurrencyCode::SYP),
            "GHS" => Ok(CurrencyCode::GHS),
            "HTG" => Ok(CurrencyCode::HTG),
            "PYG" => Ok(CurrencyCode::PYG),
            "XCG" => Ok(CurrencyCode::XCG),
            "PGK" => Ok(CurrencyCode::PGK),
            "LAK" => Ok(CurrencyCode::LAK),
            "MWK" => Ok(CurrencyCode::MWK),
            "ZMW" => Ok(CurrencyCode::ZMW),
            "AOA" => Ok(CurrencyCode::AOA),
            "MMK" => Ok(CurrencyCode::MMK),
            "GEL" => Ok(CurrencyCode::GEL),
            "MDL" => Ok(CurrencyCode::MDL),
            "ALL" => Ok(CurrencyCode::ALL),
            "HNL" => Ok(CurrencyCode::HNL),
            "SLE" => Ok(CurrencyCode::SLE),
            "SZL" => Ok(CurrencyCode::SZL),
            "LSL" => Ok(CurrencyCode::LSL),
            "AZN" => Ok(CurrencyCode::AZN),
            "MZN" => Ok(CurrencyCode::MZN),
            "NGN" => Ok(CurrencyCode::NGN),
            "ERN" => Ok(CurrencyCode::ERN),
            "TWD" => Ok(CurrencyCode::TWD),
            "TMT" => Ok(CurrencyCode::TMT),
            "MRU" => Ok(CurrencyCode::MRU),
            "TOP" => Ok(CurrencyCode::TOP),
            "MOP" => Ok(CurrencyCode::MOP),
            "ARS" => Ok(CurrencyCode::ARS),
            "DOP" => Ok(CurrencyCode::DOP),
            "COP" => Ok(CurrencyCode::COP),
            "CUP" => Ok(CurrencyCode::CUP),
            "UYU" => Ok(CurrencyCode::UYU),
            "BWP" => Ok(CurrencyCode::BWP),
            "GTQ" => Ok(CurrencyCode::GTQ),
            "IRR" => Ok(CurrencyCode::IRR),
            "YER" => Ok(CurrencyCode::YER),
            "QAR" => Ok(CurrencyCode::QAR),
            "OMR" => Ok(CurrencyCode::OMR),
            "SAR" => Ok(CurrencyCode::SAR),
            "KHR" => Ok(CurrencyCode::KHR),
            "BYN" => Ok(CurrencyCode::BYN),
            "RUB" => Ok(CurrencyCode::RUB),
            "LKR" => Ok(CurrencyCode::LKR),
            "MVR" => Ok(CurrencyCode::MVR),
            "MUR" => Ok(CurrencyCode::MUR),
            "NPR" => Ok(CurrencyCode::NPR),
            "PKR" => Ok(CurrencyCode::PKR),
            "SCR" => Ok(CurrencyCode::SCR),
            "PEN" => Ok(CurrencyCode::PEN),
            "KGS" => Ok(CurrencyCode::KGS),
            "TJS" => Ok(CurrencyCode::TJS),
            "UZS" => Ok(CurrencyCode::UZS),
            "KES" => Ok(CurrencyCode::KES),
            "SOS" => Ok(CurrencyCode::SOS),
            "TZS" => Ok(CurrencyCode::TZS),
            "UGX" => Ok(CurrencyCode::UGX),
            "BDT" => Ok(CurrencyCode::BDT),
            "WST" => Ok(CurrencyCode::WST),
            "KZT" => Ok(CurrencyCode::KZT),
            "MNT" => Ok(CurrencyCode::MNT),
            "VUV" => Ok(CurrencyCode::VUV),
            "BAM" => Ok(CurrencyCode::BAM),
            "ZWG" => Ok(CurrencyCode::ZWG),
            "PAB" => Ok(CurrencyCode::PAB),
            "ETB" => Ok(CurrencyCode::ETB),
            other => Ok(CurrencyCode::Other(other.to_string())),
        }
    }
}

impl CurrencyCode {
    pub fn name(&self) -> &str {
        match self {
            // Table A and C currencies (main currencies with bid/ask rates)
            CurrencyCode::PLN => "Polish Zloty",
            CurrencyCode::USD => "United States Dollar",
            CurrencyCode::EUR => "Euro",
            CurrencyCode::GBP => "British Pound Sterling",
            CurrencyCode::CHF => "Swiss Franc",
            CurrencyCode::AUD => "Australian Dollar",
            CurrencyCode::CAD => "Canadian Dollar",
            CurrencyCode::HUF => "Hungarian Forint",
            CurrencyCode::JPY => "Japanese Yen",
            CurrencyCode::CZK => "Czech Koruna",
            CurrencyCode::DKK => "Danish Krone",
            CurrencyCode::NOK => "Norwegian Krone",
            CurrencyCode::SEK => "Swedish Krona",
            CurrencyCode::XDR => "Special Drawing Rights (IMF)",

            // Table A only currencies (mid rates only)
            CurrencyCode::THB => "Thai Baht",
            CurrencyCode::HKD => "Hong Kong Dollar",
            CurrencyCode::NZD => "New Zealand Dollar",
            CurrencyCode::SGD => "Singapore Dollar",
            CurrencyCode::UAH => "Ukrainian Hryvnia",
            CurrencyCode::ISK => "Icelandic Króna",
            CurrencyCode::RON => "Romanian Leu",
            CurrencyCode::BGN => "Bulgarian Lev",
            CurrencyCode::TRY => "Turkish Lira",
            CurrencyCode::ILS => "Israeli New Shekel",
            CurrencyCode::CLP => "Chilean Peso",
            CurrencyCode::PHP => "Philippine Peso",
            CurrencyCode::MXN => "Mexican Peso",
            CurrencyCode::ZAR => "South African Rand",
            CurrencyCode::BRL => "Brazilian Real",
            CurrencyCode::MYR => "Malaysian Ringgit",
            CurrencyCode::IDR => "Indonesian Rupiah",
            CurrencyCode::INR => "Indian Rupee",
            CurrencyCode::KRW => "South Korean Won",
            CurrencyCode::CNY => "Chinese Yuan Renminbi",

            // Table B currencies (additional currencies)
            CurrencyCode::AFN => "Afghan Afghani",
            CurrencyCode::MGA => "Malagasy Ariary",
            CurrencyCode::VES => "Venezuelan Bolívar Soberano",
            CurrencyCode::BOB => "Bolivian Boliviano",
            CurrencyCode::CRC => "Costa Rican Colón",
            CurrencyCode::SVC => "Salvadoran Colón",
            CurrencyCode::NIO => "Nicaraguan Córdoba Oro",
            CurrencyCode::GMD => "Gambian Dalasi",
            CurrencyCode::MKD => "Macedonian Denar",
            CurrencyCode::DZD => "Algerian Dinar",
            CurrencyCode::BHD => "Bahraini Dinar",
            CurrencyCode::IQD => "Iraqi Dinar",
            CurrencyCode::JOD => "Jordanian Dinar",
            CurrencyCode::KWD => "Kuwaiti Dinar",
            CurrencyCode::LYD => "Libyan Dinar",
            CurrencyCode::RSD => "Serbian Dinar",
            CurrencyCode::TND => "Tunisian Dinar",
            CurrencyCode::MAD => "Moroccan Dirham",
            CurrencyCode::AED => "UAE Dirham",
            CurrencyCode::STN => "São Tomé and Príncipe Dobra",
            CurrencyCode::BSD => "Bahamian Dollar",
            CurrencyCode::BBD => "Barbadian Dollar",
            CurrencyCode::BZD => "Belize Dollar",
            CurrencyCode::BND => "Brunei Dollar",
            CurrencyCode::FJD => "Fijian Dollar",
            CurrencyCode::GYD => "Guyanese Dollar",
            CurrencyCode::JMD => "Jamaican Dollar",
            CurrencyCode::LRD => "Liberian Dollar",
            CurrencyCode::NAD => "Namibian Dollar",
            CurrencyCode::SRD => "Surinamese Dollar",
            CurrencyCode::TTD => "Trinidad and Tobago Dollar",
            CurrencyCode::XCD => "East Caribbean Dollar",
            CurrencyCode::SBD => "Solomon Islands Dollar",
            CurrencyCode::VND => "Vietnamese Dong",
            CurrencyCode::AMD => "Armenian Dram",
            CurrencyCode::CVE => "Cape Verdean Escudo",
            CurrencyCode::AWG => "Aruban Florin",
            CurrencyCode::BIF => "Burundian Franc",
            CurrencyCode::XOF => "CFA Franc BCEAO",
            CurrencyCode::XAF => "CFA Franc BEAC",
            CurrencyCode::XPF => "CFP Franc",
            CurrencyCode::DJF => "Djiboutian Franc",
            CurrencyCode::GNF => "Guinean Franc",
            CurrencyCode::KMF => "Comorian Franc",
            CurrencyCode::CDF => "Congolese Franc",
            CurrencyCode::RWF => "Rwandan Franc",
            CurrencyCode::EGP => "Egyptian Pound",
            CurrencyCode::GIP => "Gibraltar Pound",
            CurrencyCode::LBP => "Lebanese Pound",
            CurrencyCode::SSP => "South Sudanese Pound",
            CurrencyCode::SDG => "Sudanese Pound",
            CurrencyCode::SYP => "Syrian Pound",
            CurrencyCode::GHS => "Ghanaian Cedi",
            CurrencyCode::HTG => "Haitian Gourde",
            CurrencyCode::PYG => "Paraguayan Guaraní",
            CurrencyCode::XCG => "Caribbean Guilder",
            CurrencyCode::PGK => "Papua New Guinean Kina",
            CurrencyCode::LAK => "Lao Kip",
            CurrencyCode::MWK => "Malawian Kwacha",
            CurrencyCode::ZMW => "Zambian Kwacha",
            CurrencyCode::AOA => "Angolan Kwanza",
            CurrencyCode::MMK => "Myanmar Kyat",
            CurrencyCode::GEL => "Georgian Lari",
            CurrencyCode::MDL => "Moldovan Leu",
            CurrencyCode::ALL => "Albanian Lek",
            CurrencyCode::HNL => "Honduran Lempira",
            CurrencyCode::SLE => "Sierra Leonean Leone",
            CurrencyCode::SZL => "Eswatini Lilangeni",
            CurrencyCode::LSL => "Lesotho Loti",
            CurrencyCode::AZN => "Azerbaijani Manat",
            CurrencyCode::MZN => "Mozambican Metical",
            CurrencyCode::NGN => "Nigerian Naira",
            CurrencyCode::ERN => "Eritrean Nakfa",
            CurrencyCode::TWD => "New Taiwan Dollar",
            CurrencyCode::TMT => "Turkmenistan Manat",
            CurrencyCode::MRU => "Mauritanian Ouguiya",
            CurrencyCode::TOP => "Tongan Pa'anga",
            CurrencyCode::MOP => "Macanese Pataca",
            CurrencyCode::ARS => "Argentine Peso",
            CurrencyCode::DOP => "Dominican Peso",
            CurrencyCode::COP => "Colombian Peso",
            CurrencyCode::CUP => "Cuban Peso",
            CurrencyCode::UYU => "Uruguayan Peso",
            CurrencyCode::BWP => "Botswana Pula",
            CurrencyCode::GTQ => "Guatemalan Quetzal",
            CurrencyCode::IRR => "Iranian Rial",
            CurrencyCode::YER => "Yemeni Rial",
            CurrencyCode::QAR => "Qatari Rial",
            CurrencyCode::OMR => "Omani Rial",
            CurrencyCode::SAR => "Saudi Riyal",
            CurrencyCode::KHR => "Cambodian Riel",
            CurrencyCode::BYN => "Belarusian Ruble",
            CurrencyCode::RUB => "Russian Ruble",
            CurrencyCode::LKR => "Sri Lankan Rupee",
            CurrencyCode::MVR => "Maldivian Rufiyaa",
            CurrencyCode::MUR => "Mauritian Rupee",
            CurrencyCode::NPR => "Nepalese Rupee",
            CurrencyCode::PKR => "Pakistani Rupee",
            CurrencyCode::SCR => "Seychellois Rupee",
            CurrencyCode::PEN => "Peruvian Sol",
            CurrencyCode::KGS => "Kyrgyzstani Som",
            CurrencyCode::TJS => "Tajikistani Somoni",
            CurrencyCode::UZS => "Uzbekistani Sum",
            CurrencyCode::KES => "Kenyan Shilling",
            CurrencyCode::SOS => "Somali Shilling",
            CurrencyCode::TZS => "Tanzanian Shilling",
            CurrencyCode::UGX => "Ugandan Shilling",
            CurrencyCode::BDT => "Bangladeshi Taka",
            CurrencyCode::WST => "Samoan Tālā",
            CurrencyCode::KZT => "Kazakhstani Tenge",
            CurrencyCode::MNT => "Mongolian Tögrög",
            CurrencyCode::VUV => "Vanuatu Vatu",
            CurrencyCode::BAM => "Bosnia and Herzegovina Convertible Mark",
            CurrencyCode::ZWG => "Zimbabwe Gold",
            CurrencyCode::PAB => "Panamanian Balboa",
            CurrencyCode::ETB => "Ethiopian Birr",

            CurrencyCode::Other(name) => name,
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Table A and C currencies (main currencies with bid/ask rates)
            CurrencyCode::PLN => write!(f, "PLN"),
            CurrencyCode::USD => write!(f, "USD"),
            CurrencyCode::EUR => write!(f, "EUR"),
            CurrencyCode::GBP => write!(f, "GBP"),
            CurrencyCode::CHF => write!(f, "CHF"),
            CurrencyCode::AUD => write!(f, "AUD"),
            CurrencyCode::CAD => write!(f, "CAD"),
            CurrencyCode::HUF => write!(f, "HUF"),
            CurrencyCode::JPY => write!(f, "JPY"),
            CurrencyCode::CZK => write!(f, "CZK"),
            CurrencyCode::DKK => write!(f, "DKK"),
            CurrencyCode::NOK => write!(f, "NOK"),
            CurrencyCode::SEK => write!(f, "SEK"),
            CurrencyCode::XDR => write!(f, "XDR"),

            // Table A only currencies (mid rates only)
            CurrencyCode::THB => write!(f, "THB"),
            CurrencyCode::HKD => write!(f, "HKD"),
            CurrencyCode::NZD => write!(f, "NZD"),
            CurrencyCode::SGD => write!(f, "SGD"),
            CurrencyCode::UAH => write!(f, "UAH"),
            CurrencyCode::ISK => write!(f, "ISK"),
            CurrencyCode::RON => write!(f, "RON"),
            CurrencyCode::BGN => write!(f, "BGN"),
            CurrencyCode::TRY => write!(f, "TRY"),
            CurrencyCode::ILS => write!(f, "ILS"),
            CurrencyCode::CLP => write!(f, "CLP"),
            CurrencyCode::PHP => write!(f, "PHP"),
            CurrencyCode::MXN => write!(f, "MXN"),
            CurrencyCode::ZAR => write!(f, "ZAR"),
            CurrencyCode::BRL => write!(f, "BRL"),
            CurrencyCode::MYR => write!(f, "MYR"),
            CurrencyCode::IDR => write!(f, "IDR"),
            CurrencyCode::INR => write!(f, "INR"),
            CurrencyCode::KRW => write!(f, "KRW"),
            CurrencyCode::CNY => write!(f, "CNY"),

            // Table B currencies (additional currencies)
            CurrencyCode::AFN => write!(f, "AFN"),
            CurrencyCode::MGA => write!(f, "MGA"),
            CurrencyCode::VES => write!(f, "VES"),
            CurrencyCode::BOB => write!(f, "BOB"),
            CurrencyCode::CRC => write!(f, "CRC"),
            CurrencyCode::SVC => write!(f, "SVC"),
            CurrencyCode::NIO => write!(f, "NIO"),
            CurrencyCode::GMD => write!(f, "GMD"),
            CurrencyCode::MKD => write!(f, "MKD"),
            CurrencyCode::DZD => write!(f, "DZD"),
            CurrencyCode::BHD => write!(f, "BHD"),
            CurrencyCode::IQD => write!(f, "IQD"),
            CurrencyCode::JOD => write!(f, "JOD"),
            CurrencyCode::KWD => write!(f, "KWD"),
            CurrencyCode::LYD => write!(f, "LYD"),
            CurrencyCode::RSD => write!(f, "RSD"),
            CurrencyCode::TND => write!(f, "TND"),
            CurrencyCode::MAD => write!(f, "MAD"),
            CurrencyCode::AED => write!(f, "AED"),
            CurrencyCode::STN => write!(f, "STN"),
            CurrencyCode::BSD => write!(f, "BSD"),
            CurrencyCode::BBD => write!(f, "BBD"),
            CurrencyCode::BZD => write!(f, "BZD"),
            CurrencyCode::BND => write!(f, "BND"),
            CurrencyCode::FJD => write!(f, "FJD"),
            CurrencyCode::GYD => write!(f, "GYD"),
            CurrencyCode::JMD => write!(f, "JMD"),
            CurrencyCode::LRD => write!(f, "LRD"),
            CurrencyCode::NAD => write!(f, "NAD"),
            CurrencyCode::SRD => write!(f, "SRD"),
            CurrencyCode::TTD => write!(f, "TTD"),
            CurrencyCode::XCD => write!(f, "XCD"),
            CurrencyCode::SBD => write!(f, "SBD"),
            CurrencyCode::VND => write!(f, "VND"),
            CurrencyCode::AMD => write!(f, "AMD"),
            CurrencyCode::CVE => write!(f, "CVE"),
            CurrencyCode::AWG => write!(f, "AWG"),
            CurrencyCode::BIF => write!(f, "BIF"),
            CurrencyCode::XOF => write!(f, "XOF"),
            CurrencyCode::XAF => write!(f, "XAF"),
            CurrencyCode::XPF => write!(f, "XPF"),
            CurrencyCode::DJF => write!(f, "DJF"),
            CurrencyCode::GNF => write!(f, "GNF"),
            CurrencyCode::KMF => write!(f, "KMF"),
            CurrencyCode::CDF => write!(f, "CDF"),
            CurrencyCode::RWF => write!(f, "RWF"),
            CurrencyCode::EGP => write!(f, "EGP"),
            CurrencyCode::GIP => write!(f, "GIP"),
            CurrencyCode::LBP => write!(f, "LBP"),
            CurrencyCode::SSP => write!(f, "SSP"),
            CurrencyCode::SDG => write!(f, "SDG"),
            CurrencyCode::SYP => write!(f, "SYP"),
            CurrencyCode::GHS => write!(f, "GHS"),
            CurrencyCode::HTG => write!(f, "HTG"),
            CurrencyCode::PYG => write!(f, "PYG"),
            CurrencyCode::XCG => write!(f, "XCG"),
            CurrencyCode::PGK => write!(f, "PGK"),
            CurrencyCode::LAK => write!(f, "LAK"),
            CurrencyCode::MWK => write!(f, "MWK"),
            CurrencyCode::ZMW => write!(f, "ZMW"),
            CurrencyCode::AOA => write!(f, "AOA"),
            CurrencyCode::MMK => write!(f, "MMK"),
            CurrencyCode::GEL => write!(f, "GEL"),
            CurrencyCode::MDL => write!(f, "MDL"),
            CurrencyCode::ALL => write!(f, "ALL"),
            CurrencyCode::HNL => write!(f, "HNL"),
            CurrencyCode::SLE => write!(f, "SLE"),
            CurrencyCode::SZL => write!(f, "SZL"),
            CurrencyCode::LSL => write!(f, "LSL"),
            CurrencyCode::AZN => write!(f, "AZN"),
            CurrencyCode::MZN => write!(f, "MZN"),
            CurrencyCode::NGN => write!(f, "NGN"),
            CurrencyCode::ERN => write!(f, "ERN"),
            CurrencyCode::TWD => write!(f, "TWD"),
            CurrencyCode::TMT => write!(f, "TMT"),
            CurrencyCode::MRU => write!(f, "MRU"),
            CurrencyCode::TOP => write!(f, "TOP"),
            CurrencyCode::MOP => write!(f, "MOP"),
            CurrencyCode::ARS => write!(f, "ARS"),
            CurrencyCode::DOP => write!(f, "DOP"),
            CurrencyCode::COP => write!(f, "COP"),
            CurrencyCode::CUP => write!(f, "CUP"),
            CurrencyCode::UYU => write!(f, "UYU"),
            CurrencyCode::BWP => write!(f, "BWP"),
            CurrencyCode::GTQ => write!(f, "GTQ"),
            CurrencyCode::IRR => write!(f, "IRR"),
            CurrencyCode::YER => write!(f, "YER"),
            CurrencyCode::QAR => write!(f, "QAR"),
            CurrencyCode::OMR => write!(f, "OMR"),
            CurrencyCode::SAR => write!(f, "SAR"),
            CurrencyCode::KHR => write!(f, "KHR"),
            CurrencyCode::BYN => write!(f, "BYN"),
            CurrencyCode::RUB => write!(f, "RUB"),
            CurrencyCode::LKR => write!(f, "LKR"),
            CurrencyCode::MVR => write!(f, "MVR"),
            CurrencyCode::MUR => write!(f, "MUR"),
            CurrencyCode::NPR => write!(f, "NPR"),
            CurrencyCode::PKR => write!(f, "PKR"),
            CurrencyCode::SCR => write!(f, "SCR"),
            CurrencyCode::PEN => write!(f, "PEN"),
            CurrencyCode::KGS => write!(f, "KGS"),
            CurrencyCode::TJS => write!(f, "TJS"),
            CurrencyCode::UZS => write!(f, "UZS"),
            CurrencyCode::KES => write!(f, "KES"),
            CurrencyCode::SOS => write!(f, "SOS"),
            CurrencyCode::TZS => write!(f, "TZS"),
            CurrencyCode::UGX => write!(f, "UGX"),
            CurrencyCode::BDT => write!(f, "BDT"),
            CurrencyCode::WST => write!(f, "WST"),
            CurrencyCode::KZT => write!(f, "KZT"),
            CurrencyCode::MNT => write!(f, "MNT"),
            CurrencyCode::VUV => write!(f, "VUV"),
            CurrencyCode::BAM => write!(f, "BAM"),
            CurrencyCode::ZWG => write!(f, "ZWG"),
            CurrencyCode::PAB => write!(f, "PAB"),
            CurrencyCode::ETB => write!(f, "ETB"),
            CurrencyCode::Other(code) => write!(f, "{}", code),
        }
    }
}
