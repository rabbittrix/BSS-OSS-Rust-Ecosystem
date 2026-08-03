import { NavLink, Route, Routes } from "react-router-dom";
import { useEffect, useState } from "react";
import { Activity, Boxes, LayoutDashboard, Sparkles, Users } from "lucide-react";
import OverviewPage from "./pages/OverviewPage";
import CatalogPage from "./pages/CatalogPage";
import CustomersPage from "./pages/CustomersPage";
import InnovativePage from "./pages/InnovativePage";
import { clsx } from "clsx";

const nav = [
  { to: "/", label: "Overview", icon: LayoutDashboard },
  { to: "/catalog", label: "Catalog", icon: Boxes },
  { to: "/customers", label: "Customers", icon: Users },
  { to: "/innovative", label: "Innovative", icon: Sparkles },
];

export default function App() {
  const [dark, setDark] = useState(() =>
    window.matchMedia("(prefers-color-scheme: dark)").matches,
  );

  useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
  }, [dark]);

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = (e: MediaQueryListEvent) => setDark(e.matches);
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);

  return (
    <div className="flex min-h-screen">
      <aside className="flex w-60 flex-col border-r border-zinc-200 bg-white/80 p-4 backdrop-blur dark:border-zinc-800 dark:bg-zinc-900/80">
        <div className="mb-8 flex items-center gap-2 px-2">
          <Activity className="h-5 w-5 text-cyan-600 dark:text-cyan-400" />
          <div>
            <p className="text-sm font-semibold tracking-tight">Telecom Suite</p>
            <p className="text-xs text-zinc-500">BSS/OSS Dashboard</p>
          </div>
        </div>
        <nav className="flex flex-1 flex-col gap-1">
          {nav.map(({ to, label, icon: Icon }) => (
            <NavLink
              key={to}
              to={to}
              end={to === "/"}
              className={({ isActive }) =>
                clsx(
                  "flex items-center gap-2 rounded-lg px-3 py-2 text-sm transition",
                  isActive
                    ? "bg-cyan-600 text-white"
                    : "text-zinc-600 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800",
                )
              }
            >
              <Icon className="h-4 w-4" />
              {label}
            </NavLink>
          ))}
        </nav>
        <button
          type="button"
          onClick={() => setDark((v) => !v)}
          className="mt-4 rounded-lg border border-zinc-200 px-3 py-2 text-xs dark:border-zinc-700"
        >
          Toggle {dark ? "light" : "dark"} mode
        </button>
      </aside>
      <main className="flex-1 overflow-auto p-8">
        <Routes>
          <Route path="/" element={<OverviewPage />} />
          <Route path="/catalog" element={<CatalogPage />} />
          <Route path="/customers" element={<CustomersPage />} />
          <Route path="/innovative" element={<InnovativePage />} />
        </Routes>
      </main>
    </div>
  );
}
