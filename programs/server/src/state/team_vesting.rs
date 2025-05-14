use anchor_lang::prelude::*;

#[account]
pub struct TeamVesting {
    pub team_wallet: Pubkey,
    pub start_time: i64,
    pub total_allocation: u64,
    pub claimed: u64,
    pub cliff_months: u8,
    pub total_months: u8,
    pub unlock_interval_months: u8,
    pub bump: u8,
}

impl TeamVesting {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 1 + 1 + 1;

    pub fn available_to_claim(&self, current_time: i64) -> u64 {
        // Usar el cliff_months almacenado. Como será 0 para devnet, cliff_end = self.start_time
        let cliff_end = self.start_time + (self.cliff_months as i64 * 30 * 24 * 60 * 60);
        // let cliff_end = self.start_time + 2 * 24 * 60 * 60; // ANTERIOR: cliff de 48h para test en devnet
        
        if current_time < cliff_end {
            return 0;
        }

        // Tratar unlock_interval_months == 0 como 1 (backward-compat con cuentas antiguas)
        let effective_interval = if self.unlock_interval_months == 0 { 1 } else { self.unlock_interval_months };

        // total_intervals usa total_months / effective_interval
        let total_intervals = self.total_months / effective_interval;
        if total_intervals == 0 { return 0; } // Prevenir división por cero si config es 0/X

        // Para pruebas, cada "mes de desbloqueo" (definido por unlock_interval_months=1) durará 1 hora
        let interval_seconds = 1 * 60 * 60; // 1 HORA POR "MES DE PRUEBA"
        // let interval_seconds = self.unlock_interval_months as i64 * 30 * 24 * 60 * 60; // ANTERIOR: usaba duración real de mes

        // elapsed se calculará desde cliff_end (que ahora es start_time)
        let elapsed = current_time - cliff_end;
        // intervals_passed contará cuántos periodos de 1 hora han transcurrido desde start_time
        let intervals_passed = (elapsed / interval_seconds).min(total_intervals as i64);

        let unlocked = (self.total_allocation / total_intervals as u64) * intervals_passed as u64;

        unlocked.saturating_sub(self.claimed)
    }
}