const customers = [
  { id: "c-1001", name: "Ana Silva", party: "Individual", segment: "Consumer", msisdn: "+351 910 000 001" },
  { id: "c-1002", name: "Orbit Telco BV", party: "Organization", segment: "Enterprise", msisdn: "+31 20 555 0100" },
  { id: "c-1003", name: "Jordan Lee", party: "Individual", segment: "Youth", msisdn: "+1 415 555 0199" },
];

export default function CustomersPage() {
  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-2xl font-semibold tracking-tight">Customer 360</h1>
        <p className="text-sm text-zinc-500">TMF629 Customer · TMF632 Party</p>
      </header>

      <div className="grid gap-4 lg:grid-cols-3">
        {customers.map((c) => (
          <article
            key={c.id}
            className="rounded-2xl border border-zinc-200 bg-white p-5 dark:border-zinc-800 dark:bg-zinc-900"
          >
            <h2 className="text-lg font-semibold">{c.name}</h2>
            <p className="mt-1 text-xs text-zinc-500">{c.id}</p>
            <dl className="mt-4 space-y-2 text-sm">
              <div className="flex justify-between">
                <dt className="text-zinc-500">Party</dt>
                <dd>{c.party}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-zinc-500">Segment</dt>
                <dd>{c.segment}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-zinc-500">MSISDN</dt>
                <dd className="font-mono text-xs">{c.msisdn}</dd>
              </div>
            </dl>
          </article>
        ))}
      </div>
    </div>
  );
}
