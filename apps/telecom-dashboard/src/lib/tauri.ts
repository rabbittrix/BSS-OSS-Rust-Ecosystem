import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type ProductEvent = {
  id: string;
  kind: string;
  product: string;
  message: string;
  related_ids: string[];
  at: string;
};

export type LogicStep = {
  id: string;
  flow_id: string;
  seq: number;
  product: string;
  tmf: string;
  method: string;
  path: string;
  status: "started" | "succeeded" | "failed" | "info" | string;
  detail: string;
  at: string;
};

export type Money = { value: number; unit: string };

async function inTauri(): Promise<boolean> {
  try {
    return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
  } catch {
    return false;
  }
}

/** Subscribe to live TMF logic steps (Tauri only). */
export async function onLogicStep(
  handler: (step: LogicStep) => void,
): Promise<UnlistenFn | null> {
  if (!(await inTauri())) return null;
  return listen<LogicStep>("logic-step", (event) => handler(event.payload));
}

export async function listEvents(limit = 20): Promise<ProductEvent[]> {
  try {
    return await invoke<ProductEvent[]>("list_product_events", { limit });
  } catch {
    return [];
  }
}

export async function listLogicSteps(limit = 40, flowId?: string): Promise<LogicStep[]> {
  try {
    return await invoke<LogicStep[]>("list_logic_steps", {
      limit,
      flowId: flowId ?? null,
    });
  } catch {
    return [];
  }
}

/** Browser fallback: simulate Turbo Boost TMF sequence for the Live Logic Viewer. */
export function simulateTurboBoostLogic(
  onStep: (step: LogicStep) => void,
  minutes = 60,
): Promise<void> {
  const flowId = crypto.randomUUID();
  const product = "turbo_boost";
  const base = [
    {
      seq: 1,
      tmf: "FLOW",
      method: "START",
      path: "/products/turbo-boost",
      status: "info",
      detail: `Buy Turbo Boost — ${minutes} min (browser simulation)`,
    },
    {
      seq: 2,
      tmf: "TMF629",
      method: "GET",
      path: "/tmf-api/customerManagement/v4/customer/{id}",
      status: "started",
      detail: "Validate subscriber before network upgrade",
    },
    {
      seq: 3,
      tmf: "TMF629",
      method: "GET",
      path: "/tmf-api/customerManagement/v4/customer/{id}",
      status: "succeeded",
      detail: "Subscriber OK",
    },
    {
      seq: 4,
      tmf: "TMF656",
      method: "POST",
      path: "/tmf-api/sliceManagement/v4/networkSlice",
      status: "started",
      detail: "Create temporary eMBB slice 'turbo-embb'",
    },
    {
      seq: 5,
      tmf: "TMF656",
      method: "POST",
      path: "/tmf-api/sliceManagement/v4/networkSlice",
      status: "succeeded",
      detail: "slice_id=sim-slice state=ACTIVE",
    },
    {
      seq: 6,
      tmf: "TMF640",
      method: "POST",
      path: "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
      status: "started",
      detail: "Activate speed upgrade against the new slice",
    },
    {
      seq: 7,
      tmf: "TMF640",
      method: "POST",
      path: "/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation",
      status: "succeeded",
      detail: "activation_id=sim-act",
    },
    {
      seq: 8,
      tmf: "FLOW",
      method: "COMPLETE",
      path: "/products/turbo-boost",
      status: "info",
      detail: "Turbo Boost ready — temporary capacity granted",
    },
  ] as const;

  return (async () => {
    for (const s of base) {
      onStep({
        id: crypto.randomUUID(),
        flow_id: flowId,
        product,
        ...s,
        at: new Date().toISOString(),
      });
      await new Promise((r) => setTimeout(r, 350));
    }
  })();
}

export async function realTimeTopUp(amount: number, unit = "EUR") {
  return invoke("real_time_topup", { amount, unit });
}

export async function turboBoost(minutes: number) {
  try {
    return await invoke("turbo_boost", { minutes });
  } catch {
    return { simulated: true, minutes };
  }
}

export async function dataWalletTransfer(amountGb: number) {
  return invoke("data_wallet_transfer", { amountGb });
}

export async function bnplDevice(deviceName: string, total: number, installments: number) {
  return invoke("bnpl_device", { deviceName, total, installments });
}

export async function issueIdentity(login: string) {
  return invoke("issue_identity", { login });
}

export async function listCatalog() {
  try {
    return await invoke<{ id: string; name: string; status: string }[]>("list_catalog");
  } catch {
    return [
      { id: "demo-1", name: "5G Unlimited", status: "ACTIVE" },
      { id: "demo-2", name: "Fiber 1Gbps", status: "ACTIVE" },
      { id: "demo-3", name: "IoT Starter", status: "RETIRED" },
    ];
  }
}
