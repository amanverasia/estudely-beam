import { useState, type ReactNode } from "react";

type Tab = "send" | "receive" | "settings";

interface LayoutProps {
  send: ReactNode;
  receive: ReactNode;
  settings: ReactNode;
}

export function Layout({ send, receive, settings }: LayoutProps) {
  const [tab, setTab] = useState<Tab>("send");

  const tabs: { id: Tab; label: string }[] = [
    { id: "send", label: "Send" },
    { id: "receive", label: "Receive" },
    { id: "settings", label: "Settings" },
  ];

  return (
    <div className="flex h-full flex-col bg-gray-50 dark:bg-gray-900">
      <nav className="flex border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
        {tabs.map((t) => (
          <button
            key={t.id}
            onClick={() => setTab(t.id)}
            className={`px-6 py-3 text-sm font-medium transition-colors ${
              tab === t.id
                ? "border-b-2 border-blue-500 text-blue-600 dark:text-blue-400"
                : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            }`}
          >
            {t.label}
          </button>
        ))}
      </nav>
      <main className="flex-1 overflow-y-auto p-6">
        {tab === "send" && send}
        {tab === "receive" && receive}
        {tab === "settings" && settings}
      </main>
    </div>
  );
}
