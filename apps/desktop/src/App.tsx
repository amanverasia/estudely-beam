import { Layout } from "./components/Layout";
import { SendPanel } from "./components/SendPanel";
import { ReceivePanel } from "./components/ReceivePanel";
import { SettingsPanel } from "./components/SettingsPanel";
import { useTransfers } from "./hooks/useTransfers";
import { useSettings } from "./hooks/useSettings";

function App() {
  const {
    transfers,
    sendFile,
    sendText,
    receiveConnect,
    acceptReceive,
    rejectReceive,
    cancel,
  } = useTransfers();
  const { settings, saveSettings } = useSettings();

  return (
    <Layout
      send={
        <SendPanel
          transfers={transfers}
          onSendFile={sendFile}
          onSendText={sendText}
          onCancel={cancel}
        />
      }
      receive={
        <ReceivePanel
          transfers={transfers}
          onReceiveConnect={receiveConnect}
          onAccept={acceptReceive}
          onReject={rejectReceive}
          onCancel={cancel}
        />
      }
      settings={<SettingsPanel settings={settings} onSave={saveSettings} />}
    />
  );
}

export default App;
