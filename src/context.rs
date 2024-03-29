pub mod context {
    #[derive(Debug)]
    pub struct HMContext {
        pub instruction_index: u32,
        pub r#type: Option<Type>,
    }

    #[derive(Debug)]
    pub struct TokenSplittingFleet {
        pub fleet_account: String,
        pub fleet_driver_account: String,
    }

    #[derive(Debug)]
    pub struct RegularDriver {
        pub driver_account: String,
    }

    #[derive(Debug)]
    pub struct NoTokenSplitting {
        pub driver_account: String,
    }

    #[derive(Debug)]
    pub struct AiTrainerRewards {
        pub account: String,
    }

    #[derive(Debug)]
    pub enum Type {
        TokenSplittingFleet(TokenSplittingFleet),
        RegularDriver(RegularDriver),
        NoTokenSplitting(NoTokenSplitting),
        AiTrainerRewards(AiTrainerRewards),
        NoContext(),
    }
}
