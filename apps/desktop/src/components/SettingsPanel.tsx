import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import type { AppSettings } from "../lib/types";

interface SettingsPanelProps {
  settings: AppSettings | null;
  onSave: (settings: AppSettings) => Promise<void>;
}

export function SettingsPanel({ settings, onSave }: SettingsPanelProps) {
  const [local, setLocal] = useState<AppSettings | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    if (settings) {
      setLocal({ ...settings });
    }
  }, [settings]);

  if (!local) {
    return (
      <p className="text-gray-500 dark:text-gray-400">Loading settings...</p>
    );
  }

  const handleSave = async () => {
    await onSave(local);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  const handlePickDir = async () => {
    const selected = await open({ directory: true });
    if (selected) {
      setLocal({ ...local, download_dir: selected });
    }
  };

  return (
    <div className="mx-auto max-w-lg space-y-6">
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
          Relay URL
        </label>
        <input
          type="text"
          value={local.relay_url}
          onChange={(e) => setLocal({ ...local, relay_url: e.target.value })}
          className="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100"
        />
      </div>

      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
          Code Length (words)
        </label>
        <input
          type="number"
          min={2}
          max={8}
          value={local.code_length}
          onChange={(e) =>
            setLocal({ ...local, code_length: parseInt(e.target.value) || 2 })
          }
          className="w-24 rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100"
        />
      </div>

      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
          Download Directory
        </label>
        <div className="flex gap-2">
          <input
            type="text"
            value={local.download_dir}
            onChange={(e) =>
              setLocal({ ...local, download_dir: e.target.value })
            }
            className="flex-1 rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100"
          />
          <button
            onClick={handlePickDir}
            className="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
          >
            Browse
          </button>
        </div>
      </div>

      <button
        onClick={handleSave}
        className="rounded-lg bg-blue-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-600"
      >
        {saved ? "Saved!" : "Save Settings"}
      </button>
    </div>
  );
}
