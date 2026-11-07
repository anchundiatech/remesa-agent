# ⛓️ Blockchain (Stellar)

**Responsable:** <!-- nombre -->

Todo lo que vive en la red Stellar: el contrato de escrow en Soroban, el despliegue en testnet y los scripts de prueba.

## Estructura

```
blockchain/
├── contracts/
│   └── escrow/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs      # Contrato de escrow
└── scripts/                # Despliegue, cuentas de prueba, trustlines USDC
```

## Tareas

- [ ] Configurar Rust + target wasm y Stellar CLI
- [ ] Crear cuentas de testnet (remitente, destinatario, agente) con Friendbot
- [ ] Añadir trustline a USDC de testnet
- [ ] Implementar `create_escrow`, `release`, `refund`, `cancel`, `get_escrow`
- [ ] Validar en `release`: solo el agente, `amount ≤ tope`, destinatario fijo, antes del `deadline`
- [ ] Tests del contrato
- [ ] Desplegar en testnet y compartir el `CONTRACT_ID`
- [ ] Probar `pathPaymentStrictSend` USDC → activo destino en SDEX
- [ ] Investigar anchor COP en testnet (SEP-24 / SEP-31)

## Interfaz que entrega a los demás

| Función | Llama | Parámetros | Retorna |
|---|---|---|---|
| `create_escrow` | Front (remitente) | `sender, recipient, token, amount, deadline, agent` | `escrow_id` |
| `release` | Agente | `escrow_id, amount` | — |
| `refund` | Front / cualquiera tras vencer | `escrow_id` | — |
| `cancel` | Front (remitente) | `escrow_id` | — |
| `get_escrow` | Agente y Front | `escrow_id` | estado del escrow |

**Datos a compartir con el equipo** (en `.env`, nunca llaves secretas en el repo):

```
ESCROW_CONTRACT_ID=
USDC_ASSET_CONTRACT_ID=
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
```
