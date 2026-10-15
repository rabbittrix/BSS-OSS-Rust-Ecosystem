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

export type SimulatedResult = { simulated: true } & Record<string, unknown>;

function inTauri(): boolean {
  try {
    return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
  } catch {
    return false;
  }
}

export function isSimulatedResult(value: unknown): value is SimulatedResult {
  return (
    typeof value === "object" &&
    value !== null &&
    "simulated" in value &&
    (value as SimulatedResult).simulated === true
  );
}

async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!inTauri()) {
    throw new Error("Tauri runtime not available");
  }
  return invoke<T>(cmd, args);
}

type StepTemplate = Pick<LogicStep, "seq" | "tmf" | "method" | "path" | "status" | "detail">;

async function playLogicSteps(
  product: string,
  templates: StepTemplate[],
  onStep: (step: LogicStep) => void,
  delayMs = 350,
): Promise<void> {
  const flowId = crypto.randomUUID();
  for (const t of templates) {
    onStep({
      id: crypto.randomUUID(),
      flow_id: flowId,
      product,
      ...t,
      at: new Date().toISOString(),
    });
    await new Promise((r) => setTimeout(r, delayMs));
  }
}

/** Subscribe to live TMF logic steps (Tauri only). */
export async function onLogicStep(
  handler: (step: LogicStep) => void,
): Promise<UnlistenFn | null> {
  if (!inTauri()) return null;
  return listen<LogicStep>("logic-step", (event) => handler(event.payload));
}

export async function listEvents(limit = 20): Promise<ProductEvent[]> {
  if (!inTauri()) return [];
  try {
    return await tauriInvoke<ProductEvent[]>("list_product_events", { limit });
  } catch {
    return [];
  }
}

export async function listLogicSteps(limit = 40, flowId?: string): Promise<LogicStep[]> {
  if (!inTauri()) return [];
  try {
    return await tauriInvoke<LogicStep[]>("list_logic_steps", {
      limit,
      flowId: flowId ?? null,
    });
  } catch {
    return [];
  }
}

/** Browser fallback: paced TMF sequence for the Live Logic Viewer. */
export function simulateTurboBoostLogic(
  onStep: (step: LogicStep) => void,
  minutes = 60,
): Promise<void> {
  return playLogicSteps(
    "turbo_boost",
    [
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
    ],
    onStep,
  );
}

export function simulateRealTimeTopUpLogic(
  onStep: (step: LogicStep) => void,
  amount: number,
  unit: string,
): Promise<void> {
  return playLogicSteps(
    "real_time_topup",
    [
      {
        seq: 1,
        tmf: "FLOW",
        method: "START",
        path: "/products/real-time-topup",
        status: "info",
        detail: `channel=browser-simulation amount=${amount} ${unit}`,
      },
      {
        seq: 2,
        tmf: "TMF629",
        method: "GET",
        path: "/tmf-api/customerManagement/v4/customer/{id}",
        status: "started",
        detail: "Validate customer exists",
      },
      {
        seq: 3,
        tmf: "TMF629",
        method: "GET",
        path: "/tmf-api/customerManagement/v4/customer/{id}",
        status: "succeeded",
        detail: "Customer validated",
      },
      {
        seq: 4,
        tmf: "TMF676",
        method: "POST",
        path: "/tmf-api/payment/v4/payment",
        status: "started",
        detail: `amount=${amount} ${unit}`,
      },
      {
        seq: 5,
        tmf: "TMF676",
        method: "POST",
        path: "/tmf-api/payment/v4/payment",
        status: "succeeded",
        detail: "payment_id=sim-pay",
      },
      {
        seq: 6,
        tmf: "TMF654",
        method: "POST",
        path: "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}/adjust",
        status: "started",
        detail: "Credit monetary prepay balance",
      },
      {
        seq: 7,
        tmf: "TMF654",
        method: "POST",
        path: "/tmf-api/prepayBalanceManagement/v4/prepayBalance/sim-bal/adjust",
        status: "succeeded",
        detail: `new_balance=${amount} ${unit}`,
      },
    ],
    onStep,
  );
}

export function simulateDataWalletLogic(
  onStep: (step: LogicStep) => void,
  amountGb: number,
): Promise<void> {
  return playLogicSteps(
    "data_wallet",
    [
      {
        seq: 1,
        tmf: "TMF632",
        method: "GET",
        path: "/tmf-api/partyManagement/v4/party/{donor}",
        status: "started",
        detail: "Resolve donor party",
      },
      {
        seq: 2,
        tmf: "TMF632",
        method: "GET",
        path: "/tmf-api/partyManagement/v4/party/{recipient}",
        status: "succeeded",
        detail: "Donor & recipient parties OK",
      },
      {
        seq: 3,
        tmf: "TMF637",
        method: "GET",
        path: "/tmf-api/productInventoryManagement/v4/product",
        status: "started",
        detail: "Locate data inventory / quota product",
      },
      {
        seq: 4,
        tmf: "TMF637",
        method: "GET",
        path: "/tmf-api/productInventoryManagement/v4/product",
        status: "succeeded",
        detail: "Inventory linked to DATA bucket",
      },
      {
        seq: 5,
        tmf: "TMF654",
        method: "POST",
        path: "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}/adjust",
        status: "started",
        detail: `Debit donor ${amountGb} GB`,
      },
      {
        seq: 6,
        tmf: "TMF654",
        method: "POST",
        path: "/tmf-api/prepayBalanceManagement/v4/prepayBalance/donor/adjust",
        status: "succeeded",
        detail: "Donor debit applied",
      },
      {
        seq: 7,
        tmf: "TMF654",
        method: "POST",
        path: "/tmf-api/prepayBalanceManagement/v4/prepayBalance/recipient/adjust",
        status: "succeeded",
        detail: "Recipient credit applied",
      },
    ],
    onStep,
  );
}

export function simulateBnplLogic(
  onStep: (step: LogicStep) => void,
  deviceName: string,
  total: number,
  installments: number,
): Promise<void> {
  return playLogicSteps(
    "bnpl",
    [
      {
        seq: 1,
        tmf: "TMF632",
        method: "GET",
        path: "/tmf-api/partyManagement/v4/party/{id}",
        status: "started",
        detail: "Load party for financing",
      },
      {
        seq: 2,
        tmf: "TMF632",
        method: "GET",
        path: "/tmf-api/partyManagement/v4/party/{id}",
        status: "succeeded",
        detail: "Party verified",
      },
      {
        seq: 3,
        tmf: "TMF666",
        method: "POST",
        path: "/tmf-api/accountManagement/v4/billingAccount",
        status: "started",
        detail: `Create BNPL billing account — ${installments}x installments (${deviceName})`,
      },
      {
        seq: 4,
        tmf: "TMF666",
        method: "POST",
        path: "/tmf-api/accountManagement/v4/billingAccount",
        status: "succeeded",
        detail: `account_id=sim-bnpl total=${total} EUR`,
      },
    ],
    onStep,
  );
}

export function simulateIdentityLogic(
  onStep: (step: LogicStep) => void,
  login: string,
): Promise<void> {
  return playLogicSteps(
    "identity_aas",
    [
      {
        seq: 1,
        tmf: "TMF632",
        method: "GET",
        path: "/tmf-api/partyManagement/v4/party/{id}",
        status: "started",
        detail: "Resolve party for identity",
      },
      {
        seq: 2,
        tmf: "TMF669",
        method: "POST",
        path: "/tmf-api/identityManagement/v4/identity",
        status: "started",
        detail: `Issue identity for ${login}`,
      },
      {
        seq: 3,
        tmf: "TMF669",
        method: "POST",
        path: "/tmf-api/identityManagement/v4/identity",
        status: "succeeded",
        detail: "identity_id=sim-id hint=***",
      },
    ],
    onStep,
  );
}

/** Run browser-side logic playback after a simulated product invoke. */
export async function simulateProductLogic(
  productId: string,
  onStep: (step: LogicStep) => void,
  ctx: SimulatedResult,
): Promise<void> {
  switch (productId) {
    case "topup":
      return simulateRealTimeTopUpLogic(
        onStep,
        Number(ctx.amount ?? 10),
        String(ctx.unit ?? "EUR"),
      );
    case "turbo":
      return simulateTurboBoostLogic(onStep, Number(ctx.minutes ?? 60));
    case "wallet":
      return simulateDataWalletLogic(onStep, Number(ctx.amountGb ?? 1));
    case "bnpl":
      return simulateBnplLogic(
        onStep,
        String(ctx.deviceName ?? "device"),
        Number(ctx.total ?? 0),
        Number(ctx.installments ?? 12),
      );
    case "identity":
      return simulateIdentityLogic(onStep, String(ctx.login ?? "user"));
    default:
      return;
  }
}

export async function realTimeTopUp(amount: number, unit = "EUR") {
  if (!inTauri()) return { simulated: true as const, amount, unit };
  return tauriInvoke("real_time_topup", { amount, unit });
}

export async function turboBoost(minutes: number) {
  if (!inTauri()) return { simulated: true as const, minutes };
  return tauriInvoke("turbo_boost", { minutes });
}

export async function dataWalletTransfer(amountGb: number) {
  if (!inTauri()) return { simulated: true as const, amountGb };
  return tauriInvoke("data_wallet_transfer", { amountGb });
}

export async function bnplDevice(deviceName: string, total: number, installments: number) {
  if (!inTauri()) return { simulated: true as const, deviceName, total, installments };
  return tauriInvoke("bnpl_device", { deviceName, total, installments });
}

export async function issueIdentity(login: string) {
  if (!inTauri()) return { simulated: true as const, login };
  return tauriInvoke("issue_identity", { login });
}

export async function listCatalog() {
  if (!inTauri()) {
    return [
      { id: "demo-1", name: "5G Unlimited", status: "ACTIVE" },
      { id: "demo-2", name: "Fiber 1Gbps", status: "ACTIVE" },
      { id: "demo-3", name: "IoT Starter", status: "RETIRED" },
    ];
  }
  try {
    return await tauriInvoke<{ id: string; name: string; status: string }[]>("list_catalog");
  } catch {
    return [
      { id: "demo-1", name: "5G Unlimited", status: "ACTIVE" },
      { id: "demo-2", name: "Fiber 1Gbps", status: "ACTIVE" },
      { id: "demo-3", name: "IoT Starter", status: "RETIRED" },
    ];
  }
}
