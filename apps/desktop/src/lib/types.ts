export type TransferId = number;

export type TransferDirection = "send" | "receive";

export type TransferStatus =
  | "connecting"
  | "awaiting_peer"
  | "offer_pending"
  | "transferring"
  | "completed"
  | "error"
  | "cancelled";

export interface Transfer {
  id: TransferId;
  direction: TransferDirection;
  status: TransferStatus;
  code?: string;
  fileName?: string;
  fileSize?: number;
  bytesTransferred: number;
  bytesTotal: number;
  savedPath?: string;
  error?: string;
}

export interface ReceiveOffer {
  transferId: TransferId;
  fileName: string;
  fileSize: number;
}

export interface AppSettings {
  relay_url: string;
  code_length: number;
  download_dir: string;
}
