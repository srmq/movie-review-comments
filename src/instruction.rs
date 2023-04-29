use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError};

pub enum MovieInstruction {
  AddMovieReview {
    title: String,
    rating: u8,
    description: String
  },
  UpdateMovieReview {
    title: String,
    rating: u8,
    description: String
  },
  AddComment {
    comment: String
  },
  InitializeMint
}

#[derive(BorshDeserialize)]
struct CommentPayload {
    comment: String
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
  title: String,
  rating: u8,
  description: String
}

impl MovieInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
          0 => {
              // Payload moved into the match statement for each payload
              let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
              Self::AddMovieReview {
              title: payload.title,
              rating: payload.rating,
              description: payload.description }
          },
          1 => {
              let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
              Self::UpdateMovieReview {
                  title: payload.title,
                  rating: payload.rating,
                  description: payload.description
              }
          },
          2 => {
              // Comment payload uses its own deserializer cause of the different data type
              let payload = CommentPayload::try_from_slice(rest).unwrap();
              Self::AddComment {
                  comment: payload.comment
              }
          },
          // New variant added here
          3 => Self::InitializeMint,          
          _ => return Err(ProgramError::InvalidInstructionData)
      })        
    }
}