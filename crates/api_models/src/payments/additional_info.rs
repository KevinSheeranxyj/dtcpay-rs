
#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankDebitAdditionalData {
    Ach(Box<AchBankDebitAdditionalData>),
    Bacs(Box<BacsBankDebitAdditionalData>),
    Becs(Box<BecsBankDebitAdditionalData>),
    Sepa(Box<SepaBankDeibtAdditionalData>),
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct BacsBankDebitAdditionalData {

    pub account_number: MaskedBankAccount,

    pub sort_code: MaskedSortCode,

    pub bank_account_holder_name: Option<Secret<String>>,
}

pub struct BecsBankDebitAdditionalData {

    pub account_number: MaskedBankAccount,

    pub bsb_number: Secret<String>,

}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AchBankDebitAdditionalData {

    pub account_number: MaskBankAccount,

    pub routing_number: MaskedRoutingNUmber,

    pub card_holder_name: Option<Secret<String>>,

}

pub struct SepaBankDebitAdditionalData {
    pub iban: MaskedIban,

    pub bank_account_holder_name: Option<Secret<String>>
}



pub enum BankRedirectDetails {
    BancontactCard(Box<BancontractBankRedirectAddtionalData>),
    Blik(Box<BlikBankRedirectAdditionalData>),
    Giropay(Box<GiropayRedirectAdditionalData>),
}

pub enum BankTransferAdditionalData {
    Ach {},
    Sepa {},
    Bacs {},
    Multibanco {},
    Permata {},
    BniVa {},
    BriVa {},
    CimbVa {},
    DanamonVa {},
    MandiriVa {},
    Pix(Box<PixBankTransferAdditionalData>),
    Pse {},
    LocalBankTransfer(Box<LocalBankTransferAdditionalData>),

}