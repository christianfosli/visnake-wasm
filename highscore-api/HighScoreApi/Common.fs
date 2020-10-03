namespace HighScoreApi.Common

open System

module Types =
    // --- Simple types --- //
    type UserName = string
    type Score = private Score of int
    type ValidationError = ValidationError of string

    module Score =
        let max = 12 * 12 - 1

        let value (Score score) = score

        let create n =
            if n < 0 || n > max
            then Error(ValidationError "score out of range")
            else Ok(Score n)

    // --- Domain Models --- //
    type HighScore =
        { UserName: UserName
          Score: Score
          TimeStamp: DateTimeOffset }

module DbUtils =
    open Types

    let connString =
        Environment.GetEnvironmentVariable "CONNECTION_STRING"