import { useQuery } from "@tanstack/react-query";
import {
  Area,
  AreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { listEvents } from "../lib/tauri";

const usage = [
  { t: "00", gb: 12 },
  { t: "04", gb: 8 },
  { t: "08", gb: 22 },
  { t: "12", gb: 35 },
  { t: "16", gb: 41 },
  { t: "20", gb: 28 },
];

export default function OverviewPage() {
  const { data: events = [] } = useQuery({
    queryKey: ["events"],
    queryFn: () => listEvents(12),
    refetchInterval: 4000,
  });

  return (
    <div className="space-y-8">
      <header>
        <h1 className="text-2xl font-semibold tracking-tight">Overview</h1>
        <p className="text-sm text-zinc-500">
          Real-time product orchestration and network usage.
        </p>
      </header>

      <section className="grid gap-4 md:grid-cols-3">
        {[
          { label: "Active boosts", value: "12" },
          { label: "Top-ups today", value: "184" },
          { label: "P2P transfers", value: "47" },
        ].map((k) => (
          <div
            key={k.label}
            className="rounded-2xl border border-zinc-200 bg-white p-5 dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p className="text-xs uppercase tracking-wide text-zinc-500">{k.label}</p>
            <p className="mt-2 text-3xl font-semibold">{k.value}</p>
          </div>
        ))}
      </section>

      <section className="rounded-2xl border border-zinc-200 bg-white p-5 dark:border-zinc-800 dark:bg-zinc-900">
        <h2 className="mb-4 text-sm font-medium">Usage (GB)</h2>
        <div className="h-64">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={usage}>
              <CartesianGrid strokeDasharray="3 3" opacity={0.2} />
              <XAxis dataKey="t" />
              <YAxis />
              <Tooltip />
              <Area type="monotone" dataKey="gb" stroke="#0891b2" fill="#67e8f9" fillOpacity={0.35} />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </section>

      <section className="rounded-2xl border border-zinc-200 bg-white p-5 dark:border-zinc-800 dark:bg-zinc-900">
        <h2 className="mb-3 text-sm font-medium">Live product events</h2>
        <ul className="space-y-2 text-sm">
          {events.length === 0 && (
            <li className="text-zinc-500">No events yet — activate a product on Innovative.</li>
          )}
          {events.map((e) => (
            <li
              key={e.id}
              className="flex items-center justify-between rounded-lg bg-zinc-50 px-3 py-2 dark:bg-zinc-950"
            >
              <span>
                <span className="font-medium text-cyan-700 dark:text-cyan-300">{e.product}</span>{" "}
                — {e.message}
              </span>
              <span className="text-xs text-zinc-500">{e.kind}</span>
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
}
