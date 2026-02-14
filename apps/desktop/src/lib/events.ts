import type { TransferId } from "./types";

export const EVENTS = {
  TRANSFER_CODE: "transfer:code",
  TRANSFER_PROGRESS: "transfer:progress",
  TRANSFER_COMPLETE: "transfer:complete",
  TRANSFER_ERROR: "transfer:error",
  RECEIVE_OFFER: "receive:offer",
} as const;

export interface CodePayload {
  transfer_id: TransferId;
  code: string;
}

export interface ProgressPayload {
  transfer_id: TransferId;
  bytes_transferred: number;
  bytes_total: number;
}

export interface CompletePayload {
  transfer_id: TransferId;
  saved_path: string | null;
}

export interface ErrorPayload {
  transfer_id: TransferId;
  error: string;
}

export interface OfferPayload {
  transfer_id: TransferId;
  file_name: string;
  file_size: number;
}
