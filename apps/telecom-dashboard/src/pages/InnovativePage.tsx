import { useCallback, useEffect, useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import LiveLogicViewer from "../components/LiveLogicViewer";
import {
  bnplDevice,
  dataWalletTransfer,
  issueIdentity,
  onLogicStep,
  realTimeTopUp,
  simulateTurboBoostLogic,
  turboBoost,
  type LogicStep,
} from "../lib/tauri";

type Action = {
  id: string;
  title: string;
  blurb: string;
  cta: string;
  run: () => Promise<unknown>;
};

const actions: Action[] = [
  {
    id: "topup",
    title: "Real-Time Top-up",
    blurb: "TMF676 Payment → TMF654 Prepay Balance",
    cta: "Top up €10",
    run: () => realTimeTopUp(10, "EUR"),
  },
  {
    id: "turbo",
    title: "Turbo Boost",
    blurb: "TMF656 Slice + TMF640 Service Activation",
    cta: "Buy Turbo Boost",
    run: () => turboBoost(60),
  },
  {
    id: "wallet",
    title: "Data Wallet",
    blurb: "TMF637 Inventory + TMF654 P2P transfer",
    cta: "Send 1 GB",
    run: () => dataWalletTransfer(1),
  },
  {
    id: "bnpl",
    title: "BNPL Device",
    blurb: "TMF632 Party + TMF666 Account financing",
    cta: "Finance handset",
    run: () => bnplDevice("Pixel Fold", 999, 12),
  },
  {
    id: "identity",
    title: "Identity-as-a-Service",
    blurb: "TMF669 Identity & Credential",
    cta: "Issue identity",
    run: () => issueIdentity("subscriber.demo"),
  },
];

export default function InnovativePage() {
  const qc = useQueryClient();
  const [steps, setSteps] = useState<LogicStep[]>([]);
  const [running, setRunning] = useState(false);

  const appendStep = useCallback((step: LogicStep) => {
    setSteps((prev) => [...prev, step]);
  }, []);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    onLogicStep((step) => appendStep(step)).then((fn) => {
      if (fn) unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, [appendStep]);

  const runProduct = async (action: Action) => {
    setRunning(true);
    setSteps([]);
    try {
      if (action.id === "turbo") {
        // Prefer live Tauri stream; fall back to browser simulation.
        const result = await action.run();
        if (result && typeof result === "object" && "simulated" in (result as object)) {
          await simulateTurboBoostLogic(appendStep, 60);
        }
      } else {
        await action.run();
      }
      qc.invalidateQueries({ queryKey: ["events"] });
    } finally {
      setRunning(false);
    }
  };

  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-2xl font-semibold tracking-tight">Innovative Services</h1>
        <p className="text-sm text-zinc-500">
          One-click activation via <code>telecom-product-engine</code> — watch TMF calls in the Live
          Logic Viewer.
        </p>
      </header>

      <div className="grid gap-6 xl:grid-cols-[1fr_1.1fr]">
        <div className="grid gap-4 sm:grid-cols-2">
          {actions.map((a) => (
            <ProductCard
              key={a.id}
              action={a}
              disabled={running}
              onRun={() => runProduct(a)}
            />
          ))}
        </div>
        <LiveLogicViewer steps={steps} running={running} />
      </div>
    </div>
  );
}

function ProductCard({
  action,
  disabled,
  onRun,
}: {
  action: Action;
  disabled: boolean;
  onRun: () => Promise<void>;
}) {
  const mutation = useMutation({ mutationFn: onRun });

  return (
    <article className="flex flex-col rounded-2xl border border-zinc-200 bg-white p-5 dark:border-zinc-800 dark:bg-zinc-900">
      <h2 className="text-lg font-semibold">{action.title}</h2>
      <p className="mt-2 flex-1 text-sm text-zinc-500">{action.blurb}</p>
      <button
        type="button"
        disabled={disabled || mutation.isPending}
        onClick={() => mutation.mutate()}
        className="mt-4 rounded-xl bg-cyan-600 px-4 py-2 text-sm font-medium text-white hover:bg-cyan-500 disabled:opacity-60"
      >
        {mutation.isPending ? "Running…" : action.cta}
      </button>
      {mutation.isError && (
        <p className="mt-2 text-xs text-red-600">{String(mutation.error)}</p>
      )}
      {mutation.isSuccess && !mutation.isPending && (
        <p className="mt-2 text-xs text-emerald-600">Done — see Live Logic Viewer.</p>
      )}
    </article>
  );
}
