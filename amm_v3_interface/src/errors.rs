use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum AmmV3Error {
    #[error("LOK")]
    Lok = 6000,
    #[error("Not approved")]
    NotApproved = 6001,
    #[error("invalid update amm config flag")]
    InvalidUpdateConfigFlag = 6002,
    #[error("Account lack")]
    AccountLack = 6003,
    #[error(
        "Remove liquitity, collect fees owed and reward then you can close position account"
    )]
    ClosePositionErr = 6004,
    #[error("Minting amount should be greater than 0")]
    ZeroMintAmount = 6005,
    #[error("Tick out of range")]
    InvaildTickIndex = 6006,
    #[error("The lower tick must be below the upper tick")]
    TickInvaildOrder = 6007,
    #[error("The tick must be greater, or equal to the minimum tick(-221818)")]
    TickLowerOverflow = 6008,
    #[error("The tick must be lesser than, or equal to the maximum tick(221818)")]
    TickUpperOverflow = 6009,
    #[error("tick % tick_spacing must be zero")]
    TickAndSpacingNotMatch = 6010,
    #[error("Invaild tick array account")]
    InvalidTickArray = 6011,
    #[error("Invaild tick array boundary")]
    InvalidTickArrayBoundary = 6012,
    #[error("Square root price limit overflow")]
    SqrtPriceLimitOverflow = 6013,
    #[error("sqrt_price_x64 out of range")]
    SqrtPriceX64 = 6014,
    #[error("Liquidity sub delta L must be smaller than before")]
    LiquiditySubValueErr = 6015,
    #[error("Liquidity add delta L must be greater, or equal to before")]
    LiquidityAddValueErr = 6016,
    #[error("Invaild liquidity when update position")]
    InvaildLiquidity = 6017,
    #[error("Both token amount must not be zero while supply liquidity")]
    ForbidBothZeroForSupplyLiquidity = 6018,
    #[error("Liquidity insufficient")]
    LiquidityInsufficient = 6019,
    #[error("Transaction too old")]
    TransactionTooOld = 6020,
    #[error("Price slippage check")]
    PriceSlippageCheck = 6021,
    #[error("Too little output received")]
    TooLittleOutputReceived = 6022,
    #[error("Too much input paid")]
    TooMuchInputPaid = 6023,
    #[error("Swap special amount can not be zero")]
    InvaildSwapAmountSpecified = 6024,
    #[error("Input pool vault is invalid")]
    InvalidInputPoolVault = 6025,
    #[error("Swap input or output amount is too small")]
    TooSmallInputOrOutputAmount = 6026,
    #[error("Not enought tick array account")]
    NotEnoughTickArrayAccount = 6027,
    #[error("Invaild first tick array account")]
    InvalidFirstTickArrayAccount = 6028,
    #[error("Invalid reward index")]
    InvalidRewardIndex = 6029,
    #[error("The init reward token reach to the max")]
    FullRewardInfo = 6030,
    #[error("The init reward token already in use")]
    RewardTokenAlreadyInUse = 6031,
    #[error(
        "The reward tokens must contain one of pool vault mint except the last reward"
    )]
    ExceptPoolVaultMint = 6032,
    #[error("Invalid reward init param")]
    InvalidRewardInitParam = 6033,
    #[error("Invalid collect reward desired amount")]
    InvalidRewardDesiredAmount = 6034,
    #[error("Invalid collect reward input account number")]
    InvalidRewardInputAccountNumber = 6035,
    #[error("Invalid reward period")]
    InvalidRewardPeriod = 6036,
    #[error(
        "Modification of emissiones is allowed within 72 hours from the end of the previous cycle"
    )]
    NotApproveUpdateRewardEmissiones = 6037,
    #[error("uninitialized reward info")]
    UnInitializedRewardInfo = 6038,
    #[error("Not support token_2022 mint extension")]
    NotSupportMint = 6039,
    #[error("Missing tickarray bitmap extension account")]
    MissingTickArrayBitmapExtensionAccount = 6040,
    #[error("Insufficient liquidity for this direction")]
    InsufficientLiquidityForDirection = 6041,
}
impl From<AmmV3Error> for ProgramError {
    fn from(e: AmmV3Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for AmmV3Error {
    fn type_of() -> &'static str {
        "AmmV3Error"
    }
}
impl PrintProgramError for AmmV3Error {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
