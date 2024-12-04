use anchor_lang::error_code;

#[error_code]
pub enum GiftErrors {
    #[msg("the greetings are too big")]
    GreetingsAreTooLong,
    #[msg("too many gifts, you are allowed to use only less then 20")]
    VectorOfMintsIsTooLong,
    #[msg("false reciever")]
    NotValidReciever,
    #[msg("not a valid gift NFT")]
    NotValidGiftNFT,
    #[msg("There is no mint address in the vector in the gift config")]
    MintAddressNotInTheVector,
    #[msg("This address is not gift owner")]
    WrongOwnerOfTheGift,
}