import type { Transfer } from "../lib/types";
import { TransferProgress } from "./TransferProgress";

interface TransferListProps {
  transfers: Transfer[];
  onCancel: (id: number) => void;
}

export function TransferList({ transfers, onCancel }: TransferListProps) {
  if (transfers.length === 0) return null;

  return (
    <div className="mt-6 space-y-3">
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">
        Transfers
      </h3>
      {transfers.map((t) => (
        <TransferProgress
          key={t.id}
          transfer={t}
          onCancel={() => onCancel(t.id)}
        />
      ))}
    </div>
  );
}
