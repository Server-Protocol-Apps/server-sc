use anchor_lang::prelude::*;

#[account]
pub struct ProjectGrant {
    pub repo_id: Pubkey,          // Identificador único del repositorio
    pub authority: Pubkey,        // Quien puede autorizar recompensas (admin o backend)
    pub total_allocated: u64,     // Total de tokens asignados al proyecto
    pub total_claimed: u64,       // Tokens ya distribuidos desde el grant
    pub grant_round: u8,          // Etapa/fase actual del proyecto
    pub bump: u8,                 // Seguridad para PDA
}

impl ProjectGrant {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1; // 90 bytes
}