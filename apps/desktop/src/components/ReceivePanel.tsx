import { useState, useCallback } from "react";
import type { Transfer, ReceiveOffer } from "../lib/types";
import { TransferList } from "./TransferList";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
}

interface ReceivePanelProps {
  transfers: Transfer[];
  onReceiveConnect: (code: string) => Promise<ReceiveOffer>;
  onAccept: (transferId: number, saveDir?: string) => Promise<void>;
  onReject: (transferId: number) => Promise<void>;
  onCancel: (id: number) => void;
}

export function ReceivePanel({
  transfers,
  onReceiveConnect,
  onAccept,
  onReject,
  onCancel,
}: ReceivePanelProps) {
  const [code, setCode] = useState("");
  const [connecting, setConnecting] = useState(false);
  const [offer, setOffer] = useState<ReceiveOffer | null>(null);
  const [error, setError] = useState<string | null>(null);

  const receiveTransfers = transfers.filter((t) => t.direction === "receive");

  const handleConnect = useCallback(async () => {
    if (!code.trim()) return;
    setConnecting(true);
    setError(null);
    try {
      const result = await onReceiveConnect(code.trim());
      setOffer(result);
      setCode("");
    } catch (e) {
      setError(String(e));
    } finally {
      setConnecting(false);
    }
  }, [code, onReceiveConnect]);

  const handleAccept = useCallback(async () => {
    if (!offer) return;
    try {
      await onAccept(offer.transferId);
      setOffer(null);
    } catch (e) {
      setError(String(e));
    }
  }, [offer, onAccept]);

  const handleReject = useCallback(async () => {
    if (!offer) return;
    try {
      await onReject(offer.transferId);
      setOffer(null);
    } catch (e) {
      setError(String(e));
    }
  }, [offer, onReject]);

  return (
    <div className="mx-auto max-w-lg space-y-6">
      {!offer && (
        <div className="space-y-3">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
            Wormhole Code
          </label>
          <div className="flex gap-2">
            <input
              type="text"
              value={code}
              onChange={(e) => setCode(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleConnect()}
              placeholder="e.g., 7-crossover-clockwork"
              className="flex-1 rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100"
              disabled={connecting}
            />
            <button
              onClick={handleConnect}
              disabled={!code.trim() || connecting}
              className="rounded-lg bg-blue-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-600 disabled:opacity-50"
            >
              {connecting ? "Connecting..." : "Connect"}
            </button>
          </div>
        </div>
      )}

      {offer && (
        <div className="rounded-lg border border-gray-200 bg-white p-6 dark:border-gray-600 dark:bg-gray-800">
          <h3 className="mb-3 text-sm font-medium text-gray-500 dark:text-gray-400">
            Incoming File
          </h3>
          <p className="text-lg font-semibold text-gray-800 dark:text-gray-100">
            {offer.fileName}
          </p>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {formatBytes(offer.fileSize)}
          </p>
          <div className="mt-4 flex gap-3">
            <button
              onClick={handleAccept}
              className="rounded-lg bg-green-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-green-600"
            >
              Accept
            </button>
            <button
              onClick={handleReject}
              className="rounded-lg bg-red-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-red-600"
            >
              Reject
            </button>
          </div>
        </div>
      )}

      {error && (
        <div className="rounded-lg bg-red-50 p-3 text-sm text-red-600 dark:bg-red-900/20 dark:text-red-400">
          {error}
        </div>
      )}

      <TransferList transfers={receiveTransfers} onCancel={onCancel} />
    </div>
  );
}
