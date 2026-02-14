import { invoke } from "@tauri-apps/api/core";
import type { TransferId, AppSettings } from "./types";
import type { OfferPayload } from "./events";

export async function sendFile(path: string): Promise<TransferId> {
  return invoke<TransferId>("send_file", { path });
}

export async function sendText(text: string): Promise<TransferId> {
  return invoke<TransferId>("send_text", { text });
}

export async function receiveConnect(code: string): Promise<OfferPayload> {
  return invoke<OfferPayload>("receive_connect", { code });
}

export async function receiveAccept(
  transferId: TransferId,
  saveDir?: string,
): Promise<void> {
  return invoke("receive_accept", {
    transferId,
    saveDir: saveDir ?? null,
  });
}

export async function receiveReject(transferId: TransferId): Promise<void> {
  return invoke("receive_reject", { transferId });
}

export async function cancelTransfer(transferId: TransferId): Promise<boolean> {
  return invoke<boolean>("cancel_transfer", { transferId });
}

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export async function setSettings(settings: AppSettings): Promise<void> {
  return invoke("set_settings", { settings });
}
