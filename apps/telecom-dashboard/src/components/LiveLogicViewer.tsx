import { clsx } from "clsx";
import { Radio } from "lucide-react";
import type { LogicStep } from "../lib/tauri";

const statusStyles: Record<string, string> = {
  started: "border-amber-400 bg-amber-50 text-amber-800 dark:bg-amber-950/40 dark:text-amber-200",
  succeeded:
    "border-emerald-400 bg-emerald-50 text-emerald-800 dark:bg-emerald-950/40 dark:text-emerald-200",
  failed: "border-red-400 bg-red-50 text-red-800 dark:bg-red-950/40 dark:text-red-200",
  info: "border-cyan-400 bg-cyan-50 text-cyan-800 dark:bg-cyan-950/40 dark:text-cyan-200",
};

type Props = {
  steps: LogicStep[];
  running: boolean;
  title?: string;
};

export default function LiveLogicViewer({
  steps,
  running,
  title = "Live Logic Viewer",
}: Props) {
  return (
    <section className="rounded-2xl border border-zinc-200 bg-white p-5 dark:border-zinc-800 dark:bg-zinc-900">
      <div className="mb-4 flex items-center justify-between gap-3">
        <div className="flex items-center gap-2">
          <Radio
            className={clsx(
              "h-4 w-4",
              running ? "animate-pulse text-cyan-500" : "text-zinc-400",
            )}
          />
          <div>
            <h2 className="text-sm font-semibold tracking-tight">{title}</h2>
            <p className="text-xs text-zinc-500">
              Real-time TMF call sequence from <code>telecom-product-engine</code>
            </p>
          </div>
        </div>
        <span className="rounded-full bg-zinc-100 px-2 py-0.5 text-[10px] uppercase tracking-wide text-zinc-500 dark:bg-zinc-800">
          {running ? "streaming" : steps.length ? "idle" : "waiting"}
        </span>
      </div>

      {steps.length === 0 ? (
        <p className="rounded-xl border border-dashed border-zinc-300 px-4 py-8 text-center text-sm text-zinc-500 dark:border-zinc-700">
          Click <strong>Buy Turbo Boost</strong> (or another product) to watch TMF629 → TMF656 →
          TMF640 calls appear here.
        </p>
      ) : (
        <ol className="relative space-y-3 border-l border-zinc-200 pl-4 dark:border-zinc-700">
          {steps.map((step) => (
            <li key={step.id} className="relative">
              <span className="absolute -left-[1.35rem] top-2 h-2.5 w-2.5 rounded-full bg-cyan-500 ring-4 ring-white dark:ring-zinc-900" />
              <div
                className={clsx(
                  "rounded-xl border px-3 py-2 text-sm",
                  statusStyles[step.status] ?? statusStyles.info,
                )}
              >
                <div className="flex flex-wrap items-center gap-2">
                  <span className="rounded bg-black/10 px-1.5 py-0.5 font-mono text-[10px] font-semibold dark:bg-white/10">
                    {step.tmf}
                  </span>
                  <span className="font-mono text-xs">
                    {step.method} {step.path}
                  </span>
                  <span className="ml-auto text-[10px] uppercase opacity-70">{step.status}</span>
                </div>
                <p className="mt-1 text-xs opacity-90">{step.detail}</p>
              </div>
            </li>
          ))}
        </ol>
      )}
    </section>
  );
}
