import { useState, useEffect, useCallback } from "react";
import type { AppSettings } from "../lib/types";
import * as commands from "../lib/commands";

export function useSettings() {
  const [settings, setSettingsState] = useState<AppSettings | null>(null);

  useEffect(() => {
    commands.getSettings().then(setSettingsState);
  }, []);

  const saveSettings = useCallback(async (newSettings: AppSettings) => {
    await commands.setSettings(newSettings);
    setSettingsState(newSettings);
  }, []);

  return { settings, saveSettings };
}
