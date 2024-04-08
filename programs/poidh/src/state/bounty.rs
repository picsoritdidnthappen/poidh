use crate::error::ErrorCode;
use anchor_lang::prelude::*;

const MAX_NAME_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 200;
const MAX_PARTICPANTS: usize = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct CreateBountyParams {
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub bounty_type: u8,
    pub vote_type: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Votes {
    pub yes: u64,
    pub no: u64,
    pub deadline: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Participant {
    pub address: Pubkey,
    pub amount: u64,
}

impl Votes {
    pub const SIZE: usize = 8 + // yes
        8 + // no
        8; // deadline
}

impl Participant {
    pub const SIZE: usize = 32 + // address
        8; // amount
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq, Eq)]
pub enum VoteTYpe {
    Poidh,
    Generic,
}

impl From<u8> for VoteTYpe {
    fn from(value: u8) -> Self {
        match value {
            0 => VoteTYpe::Poidh,
            1 => VoteTYpe::Generic,
            _ => panic!("Invalid vote type"),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq, Eq)]
pub enum BountyType {
    Solo,
    Open,
}

impl From<u8> for BountyType {
    fn from(value: u8) -> Self {
        match value {
            0 => BountyType::Solo,
            1 => BountyType::Open,
            _ => panic!("Invalid bounty type"),
        }
    }
}

#[account]
pub struct Bounty {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub payment_mint: Pubkey,
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub claimer: Pubkey,
    pub created_at: i64,
    pub claim_id: u64,
    pub votes: Votes,
    pub participants: Vec<Participant>,
    pub bounty_type: u8,
    pub vote_type: u8,
    pub bump: [u8; 1],
}

impl Bounty {
    pub const SIZE: usize = 8 +
        32 + // authority
        32 + // mint
        32 + // payment_mint
        4 + MAX_NAME_LENGTH + // name (string prefix + max length)
        4 + MAX_DESCRIPTION_LENGTH + // description (string prefix + max length)
        8 + // amount
        32 + // claimer
        8 + // created_at
        8 + // claim_id
        Votes::SIZE + // votes
        4 + (Participant::SIZE * MAX_PARTICPANTS) + // participants (vec prefix + max 10 participants)
        1 + // bounty_type
        1; // vote_type

    pub fn new(
        authority: Pubkey,
        mint: Pubkey,
        payment_mint: Pubkey,
        name: String,
        description: String,
        amount: u64,
        created_at: i64,
        bounty_type: u8,
        vote_type: u8,
        bump: u8,
    ) -> Self {
        let mut bounty = Bounty {
            authority,
            mint,
            payment_mint,
            name,
            description,
            amount,
            claimer: Pubkey::default(),
            created_at,
            claim_id: 0,
            votes: Votes {
                yes: 0,
                no: 0,
                deadline: 0,
            },
            participants: vec![],
            bounty_type,
            vote_type,
            bump: [bump],
        };

        // If the bounty type is Open, add the authority and amount as a participant
        if bounty_type == BountyType::Open as u8 {
            bounty.participants.push(Participant {
                address: authority,
                amount,
            });
        }

        bounty
    }

    pub fn as_seeds(&self) -> [&[u8]; 4] {
        [
            "bounty".as_bytes(),
            self.authority.as_ref(),
            self.mint.as_ref(),
            &self.bump,
        ]
    }

    pub fn validate_bounty_type(&self, bounty_type: u8) {
        let _ = BountyType::from(bounty_type);
    }

    pub fn validate_vote_type(&self, vote_type: u8) {
        let _ = VoteTYpe::from(vote_type);
    }

    pub fn validate_participants_empty(&self) -> Result<()> {
        for participant in &self.participants {
            if participant.amount != 0 {
                return Err(ProgramError::InvalidAccountData.into());
            }
        }

        Ok(())
    }

    pub fn add_participant(&mut self, participant: Pubkey, amount: u64) -> Result<()> {
        if self.bounty_type != BountyType::Open as u8 {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        if self.participants.len() >= MAX_PARTICPANTS {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        self.participants.push(Participant {
            address: participant,
            amount,
        });

        Ok(())
    }

    pub fn increase_participant_shares(&mut self, participant: Pubkey, amount: u64) -> Result<()> {
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        let participant_index = self
            .participants
            .iter()
            .position(|p| p.address == participant)
            .ok_or(ErrorCode::ParticipantDoesNotExist)?;

        let participant = &mut self.participants[participant_index];

        participant.amount = participant
            .amount
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn decrease_participant_shares(&mut self, participant: Pubkey, amount: u64) -> Result<()> {
        let participant_index = self
            .participants
            .iter()
            .position(|p| p.address == participant)
            .ok_or(ErrorCode::ParticipantDoesNotExist)?;

        let participant_shares = &mut self.participants[participant_index];

        if amount > participant_shares.amount {
            return Err(ErrorCode::InsufficientShares.into());
        }

        participant_shares.amount = participant_shares
            .amount
            .checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;

        if participant_shares.amount == 0 {
            self.participants.remove(participant_index);
        }

        Ok(())
    }
}