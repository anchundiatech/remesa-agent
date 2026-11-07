# 🧠 Agente

**Responsable:** <!-- nombre -->

El agente que vigila la tasa, decide cuándo enviar, llama al contrato y avisa al destinatario.

## Estructura

```
agent/
├── src/
│   ├── index           # Punto de entrada: bucle de vigilancia o endpoint
│   ├── policy/         # Decide si enviar ahora, esperar o no enviar
│   ├── price/          # Obtiene la tasa USDC → COP (SDEX, oráculo o API)
│   ├── stellar/        # Llama a release() y get_escrow() del contrato
│   └── notify/         # Mensaje al destinatario (WhatsApp / Telegram / SMS / email)
└── tests/
```

## Tareas

- [ ] Elegir lenguaje (TypeScript o Python) y el SDK de Stellar correspondiente
- [ ] Interpretar la instrucción del usuario («envía 100 USDC a mi mamá este mes…») con un LLM y extraer parámetros
- [ ] `price/`: obtener la tasa actual y un histórico corto
- [ ] `policy/`: implementar la política (ver abajo) y registrar la justificación de cada decisión
- [ ] `stellar/`: firmar y enviar `release(escrow_id, amount)` con la llave del agente
- [ ] `notify/`: mensaje claro al destinatario con monto en pesos y cómo retirar
- [ ] Modo demo: permitir simular una tasa para mostrar la decisión en vivo
- [ ] Tests de la política

## Política de decisión

Entradas: `amount`, `recipient`, `deadline`, `min_rate`, `target_rate`.

1. Tasa ≥ `target_rate` → **enviar ya**.
2. Tasa ≥ `min_rate` y por encima del promedio de los últimos *N* días → **enviar**.
3. Faltan menos de *X* días para el `deadline` y tasa ≥ `min_rate` → **enviar**.
4. Llega el `deadline` con tasa < `min_rate` → **no enviar**, avisar al remitente.

Salida: `{ decision: "send" | "wait" | "skip", rate, reason }`

## Interfaces

**Usa de Blockchain:** `ESCROW_CONTRACT_ID`, funciones `release` y `get_escrow`.

**Expone al Front** (API propuesta):

| Método | Ruta | Descripción |
|---|---|---|
| `POST` | `/instructions` | Recibe la instrucción del remitente y el `escrow_id` |
| `GET` | `/escrows/:id/status` | Estado, tasa actual y última decisión con su justificación |
| `GET` | `/rate` | Tasa USDC → COP actual |

## Variables de entorno

```
AGENT_SECRET_KEY=        # nunca subir al repo
ESCROW_CONTRACT_ID=
SOROBAN_RPC_URL=
HORIZON_URL=https://horizon-testnet.stellar.org
LLM_API_KEY=
NOTIFY_PROVIDER_KEY=
```
