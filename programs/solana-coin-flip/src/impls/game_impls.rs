use crate::*;
use anchor_lang::prelude::*;

impl GameType {
    pub fn from_u32(val: u32) -> anchor_lang::Result<GameType> {
        match val {
            0 => Ok(GameType::None),
            1 => Ok(GameType::CoinFlip),
            _ => Err(error!(VrfFlipError::InvalidGameType)),
        }
    }

    pub fn get_game_config(&self) -> anchor_lang::Result<GameConfig> {
        match self {
            GameType::CoinFlip => Ok(GameConfig {
                num_vrf_requests: 1,
                min: 1,
                max: 2,
                payout_multiplier: 1,
            }),
            _ => Err(error!(VrfFlipError::InvalidGameType)),
        }
    }
}

impl Default for GameType {
    fn default() -> GameType {
        GameType::None
    }
}
