# 🖥️ Front

**Responsable:** <!-- nombre -->

La interfaz del remitente (y una vista simple para el destinatario) que conecta la wallet, crea el escrow y muestra lo que hace el agente.

## Estructura

```
front/
└── src/
    ├── pages/          # Pantallas
    ├── components/     # Componentes reutilizables
    └── lib/            # Conexión a wallet, contrato y API del agente
```

## Pantallas

| Pantalla | Qué muestra |
|---|---|
| **Inicio / conectar wallet** | Conexión con Freighter (u otra wallet de Stellar) |
| **Nueva remesa** | Chat o formulario: «Envía 100 USDC a mi mamá este mes, busca una tasa razonable» → monto, destinatario, plazo, tasa mínima y objetivo |
| **Confirmar y depositar** | Resumen y firma de `create_escrow` |
| **Seguimiento** | Tasa actual, gráfico corto, decisión del agente y su justificación, estado del escrow |
| **Resultado** | Envío ejecutado, monto en pesos, enlace a Stellar Expert |
| **Vista destinatario** (opcional) | Cuánto llega, cuándo y cómo retirar |

## Tareas

- [ ] Elegir framework (p. ej. Next.js / Vite + React)
- [ ] Integrar wallet (Freighter) en testnet
- [ ] Formulario / chat de nueva remesa
- [ ] Llamar a `create_escrow`, `cancel` y `refund` del contrato
- [ ] Consumir la API del agente para estado y tasa
- [ ] Pantalla de seguimiento con la justificación del agente
- [ ] Botón de demo para simular cambio de tasa
- [ ] Diseño responsive (el destinatario lo verá en el celular)

## Interfaces

**Usa de Blockchain:** `ESCROW_CONTRACT_ID`, funciones `create_escrow`, `cancel`, `refund`, `get_escrow`.

**Usa del Agente:** `POST /instructions`, `GET /escrows/:id/status`, `GET /rate`.

## Variables de entorno

```
NEXT_PUBLIC_ESCROW_CONTRACT_ID=
NEXT_PUBLIC_SOROBAN_RPC_URL=
NEXT_PUBLIC_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
NEXT_PUBLIC_AGENT_API_URL=
```
