import { useState } from "react";
import type { Transfer } from "../lib/types";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
}

function statusLabel(status: Transfer["status"]): string {
  switch (status) {
    case "connecting":
      return "Connecting...";
    case "awaiting_peer":
      return "Waiting for peer";
    case "offer_pending":
      return "Offer pending";
    case "transferring":
      return "Transferring";
    case "completed":
      return "Complete";
    case "error":
      return "Failed";
    case "cancelled":
      return "Cancelled";
  }
}

function statusColor(status: Transfer["status"]): string {
  switch (status) {
    case "completed":
      return "text-green-600 dark:text-green-400";
    case "error":
    case "cancelled":
      return "text-red-500 dark:text-red-400";
    case "awaiting_peer":
      return "text-amber-500 dark:text-amber-400";
    default:
      return "text-gray-500 dark:text-gray-400";
  }
}

interface TransferProgressProps {
  transfer: Transfer;
  onCancel?: () => void;
}

export function TransferProgress({
  transfer,
  onCancel,
}: TransferProgressProps) {
  const [copied, setCopied] = useState(false);

  const fraction =
    transfer.bytesTotal > 0
      ? transfer.bytesTransferred / transfer.bytesTotal
      : 0;
  const percent = Math.round(fraction * 100);

  const showCode =
    transfer.code &&
    (transfer.status === "awaiting_peer" || transfer.status === "connecting");

  const isActive =
    transfer.status !== "completed" &&
    transfer.status !== "error" &&
    transfer.status !== "cancelled";

  const handleCopy = async () => {
    if (!transfer.code) return;
    await navigator.clipboard.writeText(transfer.code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-600 dark:bg-gray-800">
      {/* Header row: filename + status */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-400 dark:text-gray-500">
            {transfer.direction === "send" ? "Sending" : "Receiving"}
          </span>
          <span className="text-sm font-medium text-gray-700 dark:text-gray-200">
            {transfer.fileName ?? "Transfer"}
          </span>
          {transfer.fileSize != null && transfer.status === "offer_pending" && (
            <span className="text-xs text-gray-400">
              ({formatBytes(transfer.fileSize)})
            </span>
          )}
        </div>
        <span className={`text-xs font-medium ${statusColor(transfer.status)}`}>
          {transfer.status === "transferring"
            ? `${percent}% — ${formatBytes(transfer.bytesTransferred)} / ${formatBytes(transfer.bytesTotal)}`
            : statusLabel(transfer.status)}
        </span>
      </div>

      {/* Wormhole code (shown inline when waiting for peer) */}
      {showCode && (
        <div className="mt-3 flex items-center gap-2 rounded-md bg-gray-50 px-3 py-2 dark:bg-gray-700/50">
          <span className="text-xs text-gray-400 dark:text-gray-500">
            Code:
          </span>
          <code className="flex-1 font-mono text-sm font-semibold text-gray-800 dark:text-gray-100">
            {transfer.code}
          </code>
          <button
            onClick={handleCopy}
            className="rounded bg-blue-500 px-2 py-1 text-xs text-white transition-colors hover:bg-blue-600"
          >
            {copied ? "Copied!" : "Copy"}
          </button>
        </div>
      )}

      {/* Progress bar */}
      {(transfer.status === "transferring" ||
        transfer.status === "completed") && (
        <div className="mt-3 h-2 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-600">
          <div
            className={`h-full rounded-full transition-all ${
              transfer.status === "completed" ? "bg-green-500" : "bg-blue-500"
            }`}
            style={{ width: `${percent}%` }}
          />
        </div>
      )}

      {/* Error message */}
      {transfer.error && (
        <p className="mt-2 text-sm text-red-500">{transfer.error}</p>
      )}

      {/* Saved path */}
      {transfer.savedPath && (
        <p className="mt-2 text-sm text-green-600 dark:text-green-400">
          Saved to: {transfer.savedPath}
        </p>
      )}

      {/* Cancel button */}
      {onCancel && isActive && (
        <button
          onClick={onCancel}
          className="mt-2 text-sm text-red-500 hover:text-red-700"
        >
          Cancel
        </button>
      )}
    </div>
  );
}
