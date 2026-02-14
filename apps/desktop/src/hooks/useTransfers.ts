import { useState, useEffect, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import type { Transfer, TransferId } from "../lib/types";
import { EVENTS } from "../lib/events";
import type {
  CodePayload,
  ProgressPayload,
  CompletePayload,
  ErrorPayload,
} from "../lib/events";
import * as commands from "../lib/commands";

export function useTransfers() {
  const [transfers, setTransfers] = useState<Map<TransferId, Transfer>>(
    new Map(),
  );

  const updateTransfer = useCallback(
    (id: TransferId, update: Partial<Transfer>) => {
      setTransfers((prev) => {
        const next = new Map(prev);
        const existing = next.get(id);
        if (existing) {
          next.set(id, { ...existing, ...update });
        }
        return next;
      });
    },
    [],
  );

  useEffect(() => {
    const unlisteners: (() => void)[] = [];

    listen<CodePayload>(EVENTS.TRANSFER_CODE, (event) => {
      updateTransfer(event.payload.transfer_id, {
        code: event.payload.code,
        status: "awaiting_peer",
      });
    }).then((u) => unlisteners.push(u));

    listen<ProgressPayload>(EVENTS.TRANSFER_PROGRESS, (event) => {
      updateTransfer(event.payload.transfer_id, {
        status: "transferring",
        bytesTransferred: event.payload.bytes_transferred,
        bytesTotal: event.payload.bytes_total,
      });
    }).then((u) => unlisteners.push(u));

    listen<CompletePayload>(EVENTS.TRANSFER_COMPLETE, (event) => {
      updateTransfer(event.payload.transfer_id, {
        status: "completed",
        savedPath: event.payload.saved_path ?? undefined,
      });
    }).then((u) => unlisteners.push(u));

    listen<ErrorPayload>(EVENTS.TRANSFER_ERROR, (event) => {
      updateTransfer(event.payload.transfer_id, {
        status: "error",
        error: event.payload.error,
      });
    }).then((u) => unlisteners.push(u));

    return () => {
      unlisteners.forEach((u) => u());
    };
  }, [updateTransfer]);

  const sendFile = useCallback(async (path: string) => {
    const id = await commands.sendFile(path);
    setTransfers((prev) => {
      const next = new Map(prev);
      next.set(id, {
        id,
        direction: "send",
        status: "connecting",
        fileName: path.split("/").pop(),
        bytesTransferred: 0,
        bytesTotal: 0,
      });
      return next;
    });
    return id;
  }, []);

  const sendText = useCallback(async (text: string) => {
    const id = await commands.sendText(text);
    setTransfers((prev) => {
      const next = new Map(prev);
      next.set(id, {
        id,
        direction: "send",
        status: "connecting",
        fileName: "message.txt",
        bytesTransferred: 0,
        bytesTotal: 0,
      });
      return next;
    });
    return id;
  }, []);

  const receiveConnect = useCallback(async (code: string) => {
    const offer = await commands.receiveConnect(code);
    setTransfers((prev) => {
      const next = new Map(prev);
      next.set(offer.transfer_id, {
        id: offer.transfer_id,
        direction: "receive",
        status: "offer_pending",
        fileName: offer.file_name,
        fileSize: offer.file_size,
        bytesTransferred: 0,
        bytesTotal: offer.file_size,
      });
      return next;
    });
    return {
      transferId: offer.transfer_id,
      fileName: offer.file_name,
      fileSize: offer.file_size,
    };
  }, []);

  const acceptReceive = useCallback(
    async (transferId: TransferId, saveDir?: string) => {
      updateTransfer(transferId, { status: "connecting" });
      await commands.receiveAccept(transferId, saveDir);
    },
    [updateTransfer],
  );

  const rejectReceive = useCallback(async (transferId: TransferId) => {
    await commands.receiveReject(transferId);
    setTransfers((prev) => {
      const next = new Map(prev);
      next.delete(transferId);
      return next;
    });
  }, []);

  const cancel = useCallback(
    async (transferId: TransferId) => {
      await commands.cancelTransfer(transferId);
      updateTransfer(transferId, { status: "cancelled" });
    },
    [updateTransfer],
  );

  return {
    transfers: Array.from(transfers.values()),
    sendFile,
    sendText,
    receiveConnect,
    acceptReceive,
    rejectReceive,
    cancel,
  };
}
