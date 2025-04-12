use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Provided url is too long")]
    URLTooLong,
    #[msg("Deadline cannot be in the past")]
    DeadlineInThePast,
    #[msg("Deadline has already passed")]
    DeadlinePassed,
    #[msg("Pull request not merged")]
    PullRequestNotMerged,
    #[msg("Submission accepted")]
    SubmissionAccepted,
}
