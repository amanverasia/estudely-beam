import { useState, useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import type { Transfer } from "../lib/types";
import { DropZone } from "./DropZone";
import { TransferList } from "./TransferList";

interface SendPanelProps {
  transfers: Transfer[];
  onSendFile: (path: string) => Promise<number>;
  onSendText: (text: string) => Promise<number>;
  onCancel: (id: number) => void;
}

export function SendPanel({
  transfers,
  onSendFile,
  onSendText,
  onCancel,
}: SendPanelProps) {
  const [textMode, setTextMode] = useState(false);
  const [text, setText] = useState("");
  const [sending, setSending] = useState(false);

  const sendTransfers = transfers.filter((t) => t.direction === "send");

  const handleDrop = useCallback(
    async (paths: string[]) => {
      if (paths.length > 0) {
        setSending(true);
        try {
          await onSendFile(paths[0]);
        } finally {
          setSending(false);
        }
      }
    },
    [onSendFile],
  );

  const handlePickFile = async () => {
    const selected = await open({ multiple: false });
    if (selected) {
      setSending(true);
      try {
        await onSendFile(selected);
      } finally {
        setSending(false);
      }
    }
  };

  const handleSendText = async () => {
    if (!text.trim()) return;
    setSending(true);
    try {
      await onSendText(text);
      setText("");
    } finally {
      setSending(false);
    }
  };

  return (
    <div className="mx-auto max-w-lg space-y-6">
      <div className="flex items-center gap-4">
        <button
          onClick={() => setTextMode(false)}
          className={`text-sm font-medium ${
            !textMode
              ? "text-blue-600 dark:text-blue-400"
              : "text-gray-500 dark:text-gray-400"
          }`}
        >
          File
        </button>
        <button
          onClick={() => setTextMode(true)}
          className={`text-sm font-medium ${
            textMode
              ? "text-blue-600 dark:text-blue-400"
              : "text-gray-500 dark:text-gray-400"
          }`}
        >
          Text
        </button>
      </div>

      {textMode ? (
        <div className="space-y-3">
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="Type your message..."
            className="w-full rounded-lg border border-gray-300 bg-white p-3 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100"
            rows={4}
          />
          <button
            onClick={handleSendText}
            disabled={!text.trim() || sending}
            className="rounded-lg bg-blue-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-600 disabled:opacity-50"
          >
            {sending ? "Sending..." : "Send Text"}
          </button>
        </div>
      ) : (
        <DropZone onDrop={handleDrop}>
          <div className="space-y-2">
            <p className="text-gray-500 dark:text-gray-400">
              {sending ? "Preparing transfer..." : "Drop a file here to send"}
            </p>
            <button
              onClick={handlePickFile}
              disabled={sending}
              className="rounded-lg bg-blue-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-600 disabled:opacity-50"
            >
              Choose File
            </button>
          </div>
        </DropZone>
      )}

      <TransferList transfers={sendTransfers} onCancel={onCancel} />
    </div>
  );
}
