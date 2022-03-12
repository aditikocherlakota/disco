#![allow(unused)]

pub enum Token {
  TokEof = -1,
  TokDef = -2,
  TokExtern = -3,
  TokIdentifier = -4,
  TokNumber = -5
}