import { useState, useEffect } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

interface DropZoneProps {
  onDrop: (paths: string[]) => void;
  children?: React.ReactNode;
}

export function DropZone({ onDrop, children }: DropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    const webview = getCurrentWebviewWindow();
    const unlisten = webview.onDragDropEvent((event) => {
      if (event.payload.type === "over") {
        setIsDragging(true);
      } else if (event.payload.type === "drop") {
        setIsDragging(false);
        onDrop(event.payload.paths);
      } else {
        setIsDragging(false);
      }
    });

    return () => {
      unlisten.then((u) => u());
    };
  }, [onDrop]);

  return (
    <div
      className={`rounded-xl border-2 border-dashed p-8 text-center transition-colors ${
        isDragging
          ? "border-blue-500 bg-blue-50 dark:bg-blue-900/20"
          : "border-gray-300 dark:border-gray-600"
      }`}
    >
      {children ?? (
        <p className="text-gray-500 dark:text-gray-400">
          Drop files here to send
        </p>
      )}
    </div>
  );
}
