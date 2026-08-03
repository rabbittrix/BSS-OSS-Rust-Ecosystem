import { useQuery } from "@tanstack/react-query";
import { listCatalog } from "../lib/tauri";

export default function CatalogPage() {
  const { data = [], isLoading } = useQuery({
    queryKey: ["catalog"],
    queryFn: listCatalog,
  });

  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-2xl font-semibold tracking-tight">Product Catalog</h1>
        <p className="text-sm text-zinc-500">TMF620 Product Catalog browser</p>
      </header>

      <div className="overflow-hidden rounded-2xl border border-zinc-200 dark:border-zinc-800">
        <table className="w-full text-left text-sm">
          <thead className="bg-zinc-100 text-xs uppercase text-zinc-500 dark:bg-zinc-900">
            <tr>
              <th className="px-4 py-3">Name</th>
              <th className="px-4 py-3">ID</th>
              <th className="px-4 py-3">Status</th>
            </tr>
          </thead>
          <tbody>
            {isLoading && (
              <tr>
                <td className="px-4 py-4 text-zinc-500" colSpan={3}>
                  Loading…
                </td>
              </tr>
            )}
            {data.map((row) => (
              <tr
                key={row.id}
                className="border-t border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-950"
              >
                <td className="px-4 py-3 font-medium">{row.name}</td>
                <td className="px-4 py-3 font-mono text-xs text-zinc-500">{row.id}</td>
                <td className="px-4 py-3">
                  <span className="rounded-full bg-cyan-100 px-2 py-0.5 text-xs text-cyan-800 dark:bg-cyan-950 dark:text-cyan-200">
                    {row.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
