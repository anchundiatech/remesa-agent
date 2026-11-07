#![no_std]
//! Contrato de escrow para remesas.
//!
//! Esqueleto: las firmas y reglas están definidas; la lógica queda por implementar.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Active,
    Released,
    Refunded,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    pub sender: Address,
    pub recipient: Address,
    pub agent: Address,
    pub token: Address,  // contrato del activo USDC
    pub amount: i128,    // tope que el agente puede liberar
    pub deadline: u64,   // timestamp límite
    pub status: Status,
}

#[contracttype]
pub enum DataKey {
    Escrow(u64),
    NextId,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// El remitente deposita USDC y fija las reglas.
    /// TODO: sender.require_auth(); transferir `amount` del sender al contrato; guardar Escrow.
    pub fn create_escrow(
        _env: Env,
        _sender: Address,
        _recipient: Address,
        _agent: Address,
        _token: Address,
        _amount: i128,
        _deadline: u64,
    ) -> u64 {
        todo!()
    }

    /// Solo el agente. Libera al destinatario fijo, con `amount <= tope` y antes del deadline.
    /// TODO: agent.require_auth(); validar estado, monto y fecha; transferir; marcar Released.
    pub fn release(_env: Env, _escrow_id: u64, _amount: i128) {
        todo!()
    }

    /// Tras el deadline, devuelve los fondos al remitente.
    /// TODO: validar deadline vencido y estado Active; transferir; marcar Refunded.
    pub fn refund(_env: Env, _escrow_id: u64) {
        todo!()
    }

    /// El remitente cancela antes del envío.
    /// TODO: sender.require_auth(); validar Active; transferir; marcar Cancelled.
    pub fn cancel(_env: Env, _escrow_id: u64) {
        todo!()
    }

    /// Consulta pública del estado.
    pub fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .expect("escrow no encontrado")
    }
}
