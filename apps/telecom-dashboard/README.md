# Telecom Product Suite Dashboard

Tauri 2 + React + Vite + Tailwind desktop UI for BSS/OSS innovative products.

## Run (web UI only)

```bash
cd apps/telecom-dashboard
npm install
npm run dev
```

## Run (Tauri desktop)

Requires Rust + [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

```bash
cd apps/telecom-dashboard
npm install
npm run tauri:dev
```

Commands invoke `telecom-product-engine` via Tauri `invoke` (in-memory gateway by default).

## Pages

- **Overview** — usage chart + live product events
- **Catalog** — TMF620-style offerings table
- **Customers** — Customer 360 cards (TMF629/632)
- **Innovative** — one-click products + **Live Logic Viewer** (streams TMF call steps as they run)

### Live Logic Viewer

Click **Buy Turbo Boost** to watch the paced sequence:

1. `TMF629` GET customer  
2. `TMF656` POST networkSlice  
3. `TMF640` POST/PATCH serviceActivation  

In Tauri, steps are emitted as `logic-step` events from `telecom-product-engine`. In the browser (`npm run dev`), Turbo Boost falls back to a client-side simulation of the same sequence.
