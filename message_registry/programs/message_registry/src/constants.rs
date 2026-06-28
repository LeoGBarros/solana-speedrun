use anchor_lang::prelude::*;

#[constant]
pub const MESSAGE_SEED: &[u8] = b"message";

/// Tamanho máximo (em bytes) permitido para o texto da mensagem.
pub const MAX_LEN: usize = 200;
