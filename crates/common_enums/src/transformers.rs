

impl CountryAlpha2 {
    pub const fn from_alpha2_to_alpha3(code: Self) -> CountryAlpha3 {
        match code {
            Self::AF => CountryAlpha3::AFG,
            Self::AX => CountryAlpha3::ALA,
            Self::AL => CountryAlpha3::ALB,
            Self::DZ => CountryAlpha3::DZA,
            Self::AS => CountryAlpha3::ASM,
            Self::AD => CountryAlpha3::AND,
            Self::AO => CountryAlpha3::AGO,
            Self::AI => CountryAlpha3::AIA,
            Self::AQ => CountryAlpha3::ATA,
            Self::AG => CountryAlpha3::ATG,
            Self::AR => CountryAlpha3::ARG,
            Self::AM => CountryAlpha3::ARM,
            Self::AW => CountryAlpha3::ABW,
            Self::AU => CountryAlpha3::AUS,
            Self::AT => CountryAlpha3::AUT,
            Self::AZ => CountryAlpha3::AZE,
            Self::BS => CountryAlpha3::BHS,
            Self::BH => CountryAlpha3::BHR,
            Self::BD => CountryAlpha3::BGD,
            Self::BB => CountryAlpha3::BRB,
            Self::BY => CountryAlpha3::BLR,
            Self::BE => CountryAlpha3::BEL,
            Self::BZ => CountryAlpha3::BLZ,
            Self::BJ => CountryAlpha3::BEN,
            Self::BM => CountryAlpha3::BMU,
            Self::BT => CountryAlpha3::BTN,
            Self::BO => CountryAlpha3::BOL,
            Self::BQ => CountryAlpha3::BES,
            Self::BA => CountryAlpha3::BIH,
            Self::BW => CountryAlpha3::BWA,
            Self::BV => CountryAlpha3::BVT,
            Self::BR => CountryAlpha3::BRA,
            Self::IO => CountryAlpha3::IOT,
            Self::BN => CountryAlpha3::BRN,
            Self::BG => CountryAlpha3::BGR,
            Self::BF => CountryAlpha3::BFA,
            Self::BI => CountryAlpha3::BDI,
            Self::CV => CountryAlpha3::CPV,
            Self::KH => CountryAlpha3::KHM,
            Self::CM => CountryAlpha3::CMR,
            Self::CA => CountryAlpha3::CAN,
            Self::KY => CountryAlpha3::CYM,
            Self::CF => CountryAlpha3::CAF,
            Self::TD => CountryAlpha3::TCD,
            Self::CL => CountryAlpha3::CHL,
            Self::CN => CountryAlpha3::CHN,
            Self::CX => CountryAlpha3::CXR,
            Self::CC => CountryAlpha3::CCK,
            Self::CO => CountryAlpha3::COL,
            Self::KM => CountryAlpha3::COM,
            Self::CG => CountryAlpha3::COG,
            Self::CD => CountryAlpha3::COD,
            Self::CK => CountryAlpha3::COK,
            Self::CR => CountryAlpha3::CRI,
            Self::CI => CountryAlpha3::CIV,
            Self::HR => CountryAlpha3::HRV,
            Self::CU => CountryAlpha3::CUB,
            Self::CW => CountryAlpha3::CUW,
            Self::CY => CountryAlpha3::CYP,
            Self::CZ => CountryAlpha3::CZE,
            Self::DK => CountryAlpha3::DNK,
            Self::DJ => CountryAlpha3::DJI,
            Self::DM => CountryAlpha3::DMA,
            Self::DO => CountryAlpha3::DOM,
            Self::EC => CountryAlpha3::ECU,
            Self::EG => CountryAlpha3::EGY,
            Self::SV => CountryAlpha3::SLV,
            Self::GQ => CountryAlpha3::GNQ,
            Self::ER => CountryAlpha3::ERI,
            Self::EE => CountryAlpha3::EST,
            Self::ET => CountryAlpha3::ETH,
            Self::FK => CountryAlpha3::FLK,
            Self::FO => CountryAlpha3::FRO,
            Self::FJ => CountryAlpha3::FJI,
            Self::FI => CountryAlpha3::FIN,
            Self::FR => CountryAlpha3::FRA,
            Self::GF => CountryAlpha3::GUF,
            Self::PF => CountryAlpha3::PYF,
            Self::TF => CountryAlpha3::ATF,
            Self::GA => CountryAlpha3::GAB,
            Self::GM => CountryAlpha3::GMB,
            Self::GE => CountryAlpha3::GEO,
            Self::DE => CountryAlpha3::DEU,
            Self::GH => CountryAlpha3::GHA,
            Self::GI => CountryAlpha3::GIB,
            Self::GR => CountryAlpha3::GRC,
            Self::GL => CountryAlpha3::GRL,
            Self::GD => CountryAlpha3::GRD,
            Self::GP => CountryAlpha3::GLP,
            Self::GU => CountryAlpha3::GUM,
            Self::GT => CountryAlpha3::GTM,
            Self::GG => CountryAlpha3::GGY,
            Self::GN => CountryAlpha3::GIN,
            Self::GW => CountryAlpha3::GNB,
            Self::GY => CountryAlpha3::GUY,
            Self::HT => CountryAlpha3::HTI,
            Self::HM => CountryAlpha3::HMD,
            Self::VA => CountryAlpha3::VAT,
            Self::HN => CountryAlpha3::HND,
            Self::HK => CountryAlpha3::HKG,
            Self::HU => CountryAlpha3::HUN,
            Self::IS => CountryAlpha3::ISL,
            Self::IN => CountryAlpha3::IND,
            Self::ID => CountryAlpha3::IDN,
            Self::IR => CountryAlpha3::IRN,
            Self::IQ => CountryAlpha3::IRQ,
            Self::IE => CountryAlpha3::IRL,
            Self::IM => CountryAlpha3::IMN,
            Self::IL => CountryAlpha3::ISR,
            Self::IT => CountryAlpha3::ITA,
            Self::JM => CountryAlpha3::JAM,
            Self::JP => CountryAlpha3::JPN,
            Self::JE => CountryAlpha3::JEY,
            Self::JO => CountryAlpha3::JOR,
            Self::KZ => CountryAlpha3::KAZ,
            Self::KE => CountryAlpha3::KEN,
            Self::KI => CountryAlpha3::KIR,
            Self::KP => CountryAlpha3::PRK,
            Self::KR => CountryAlpha3::KOR,
            Self::KW => CountryAlpha3::KWT,
            Self::KG => CountryAlpha3::KGZ,
            Self::LA => CountryAlpha3::LAO,
            Self::LV => CountryAlpha3::LVA,
            Self::LB => CountryAlpha3::LBN,
            Self::LS => CountryAlpha3::LSO,
            Self::LR => CountryAlpha3::LBR,
            Self::LY => CountryAlpha3::LBY,
            Self::LI => CountryAlpha3::LIE,
            Self::LT => CountryAlpha3::LTU,
            Self::LU => CountryAlpha3::LUX,
            Self::MO => CountryAlpha3::MAC,
            Self::MK => CountryAlpha3::MKD,
            Self::MG => CountryAlpha3::MDG,
            Self::MW => CountryAlpha3::MWI,
            Self::MY => CountryAlpha3::MYS,
            Self::MV => CountryAlpha3::MDV,
            Self::ML => CountryAlpha3::MLI,
            Self::MT => CountryAlpha3::MLT,
            Self::MH => CountryAlpha3::MHL,
            Self::MQ => CountryAlpha3::MTQ,
            Self::MR => CountryAlpha3::MRT,
            Self::MU => CountryAlpha3::MUS,
            Self::YT => CountryAlpha3::MYT,
            Self::MX => CountryAlpha3::MEX,
            Self::FM => CountryAlpha3::FSM,
            Self::MD => CountryAlpha3::MDA,
            Self::MC => CountryAlpha3::MCO,
            Self::MN => CountryAlpha3::MNG,
            Self::ME => CountryAlpha3::MNE,
            Self::MS => CountryAlpha3::MSR,
            Self::MA => CountryAlpha3::MAR,
            Self::MZ => CountryAlpha3::MOZ,
            Self::MM => CountryAlpha3::MMR,
            Self::NA => CountryAlpha3::NAM,
            Self::NR => CountryAlpha3::NRU,
            Self::NP => CountryAlpha3::NPL,
            Self::NL => CountryAlpha3::NLD,
            Self::NC => CountryAlpha3::NCL,
            Self::NZ => CountryAlpha3::NZL,
            Self::NI => CountryAlpha3::NIC,
            Self::NE => CountryAlpha3::NER,
            Self::NG => CountryAlpha3::NGA,
            Self::NU => CountryAlpha3::NIU,
            Self::NF => CountryAlpha3::NFK,
            Self::MP => CountryAlpha3::MNP,
            Self::NO => CountryAlpha3::NOR,
            Self::OM => CountryAlpha3::OMN,
            Self::PK => CountryAlpha3::PAK,
            Self::PW => CountryAlpha3::PLW,
            Self::PS => CountryAlpha3::PSE,
            Self::PA => CountryAlpha3::PAN,
            Self::PG => CountryAlpha3::PNG,
            Self::PY => CountryAlpha3::PRY,
            Self::PE => CountryAlpha3::PER,
            Self::PH => CountryAlpha3::PHL,
            Self::PN => CountryAlpha3::PCN,
            Self::PL => CountryAlpha3::POL,
            Self::PT => CountryAlpha3::PRT,
            Self::PR => CountryAlpha3::PRI,
            Self::QA => CountryAlpha3::QAT,
            Self::RE => CountryAlpha3::REU,
            Self::RO => CountryAlpha3::ROU,
            Self::RU => CountryAlpha3::RUS,
            Self::RW => CountryAlpha3::RWA,
            Self::BL => CountryAlpha3::BLM,
            Self::SH => CountryAlpha3::SHN,
            Self::KN => CountryAlpha3::KNA,
            Self::LC => CountryAlpha3::LCA,
            Self::MF => CountryAlpha3::MAF,
            Self::PM => CountryAlpha3::SPM,
            Self::VC => CountryAlpha3::VCT,
            Self::WS => CountryAlpha3::WSM,
            Self::SM => CountryAlpha3::SMR,
            Self::ST => CountryAlpha3::STP,
            Self::SA => CountryAlpha3::SAU,
            Self::SN => CountryAlpha3::SEN,
            Self::RS => CountryAlpha3::SRB,
            Self::SC => CountryAlpha3::SYC,
            Self::SL => CountryAlpha3::SLE,
            Self::SG => CountryAlpha3::SGP,
            Self::SX => CountryAlpha3::SXM,
            Self::SK => CountryAlpha3::SVK,
            Self::SI => CountryAlpha3::SVN,
            Self::SB => CountryAlpha3::SLB,
            Self::SO => CountryAlpha3::SOM,
            Self::ZA => CountryAlpha3::ZAF,
            Self::GS => CountryAlpha3::SGS,
            Self::SS => CountryAlpha3::SSD,
            Self::ES => CountryAlpha3::ESP,
            Self::LK => CountryAlpha3::LKA,
            Self::SD => CountryAlpha3::SDN,
            Self::SR => CountryAlpha3::SUR,
            Self::SJ => CountryAlpha3::SJM,
            Self::SZ => CountryAlpha3::SWZ,
            Self::SE => CountryAlpha3::SWE,
            Self::CH => CountryAlpha3::CHE,
            Self::SY => CountryAlpha3::SYR,
            Self::TW => CountryAlpha3::TWN,
            Self::TJ => CountryAlpha3::TJK,
            Self::TZ => CountryAlpha3::TZA,
            Self::TH => CountryAlpha3::THA,
            Self::TL => CountryAlpha3::TLS,
            Self::TG => CountryAlpha3::TGO,
            Self::TK => CountryAlpha3::TKL,
            Self::TO => CountryAlpha3::TON,
            Self::TT => CountryAlpha3::TTO,
            Self::TN => CountryAlpha3::TUN,
            Self::TR => CountryAlpha3::TUR,
            Self::TM => CountryAlpha3::TKM,
            Self::TC => CountryAlpha3::TCA,
            Self::TV => CountryAlpha3::TUV,
            Self::UG => CountryAlpha3::UGA,
            Self::UA => CountryAlpha3::UKR,
            Self::AE => CountryAlpha3::ARE,
            Self::GB => CountryAlpha3::GBR,
            Self::US => CountryAlpha3::USA,
            Self::UM => CountryAlpha3::UMI,
            Self::UY => CountryAlpha3::URY,
            Self::UZ => CountryAlpha3::UZB,
            Self::VU => CountryAlpha3::VUT,
            Self::VE => CountryAlpha3::VEN,
            Self::VN => CountryAlpha3::VNM,
            Self::VG => CountryAlpha3::VGB,
            Self::VI => CountryAlpha3::VIR,
            Self::WF => CountryAlpha3::WLF,
            Self::EH => CountryAlpha3::ESH,
            Self::YE => CountryAlpha3::YEM,
            Self::ZM => CountryAlpha3::ZMB,
            Self::ZW => CountryAlpha3::ZWE,
        }
    }
}