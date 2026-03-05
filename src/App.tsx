import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Settings = Record<string, number>;

function App() {
  const [settings, setSettings] = useState<Settings>({});

  useEffect(() => {
    invoke<Settings>("get_settings").then((payload) => {
      setSettings(payload);
    });
  }, []);

  const updateSettings = useCallback(
    (() => {
      let timer: ReturnType<typeof setTimeout>;
      return (newValues: Partial<Settings>) => {
        clearTimeout(timer);
        timer = setTimeout(() => {
          invoke("update_settings", { newSettings: newValues });
        }, 50);
      };
    })(),
    []
  );

  const handleChange = (key: string, value: number) => {
    setSettings((prev) => ({ ...prev, [key]: value }));
    updateSettings({ [key]: value });
  };

  return (
    <main className="container">
      <h3>Visual Control Panel</h3>

      {/* Particle Size */}
      <div className="control">
        <label>Particle Size: {settings.particle_size}</label>
        <input
          type="range"
          min="5"
          max="100"
          step="1"
          value={settings.particle_size ?? 20}
          onChange={(e) =>
            handleChange("particle_size", parseFloat(e.target.value))
          }
        />
      </div>

      {/* Gravity */}
      <div className="control">
        <label>Gravity Strength: {settings.gravity_strength}</label>
        <input
          type="range"
          min="0.5"
          max="10"
          step="0.5"
          value={settings.gravity_strength ?? 1}
          onChange={(e) =>
            handleChange("gravity_strength", parseFloat(e.target.value))
          }
        />
      </div>

      {/* Hue */}
      <div className="control">
        <label>Hue Offset: {settings.hue_offset}</label>
        <input
          type="range"
          min="0"
          max="360"
          step="1"
          value={settings.hue_offset ?? 0}
          onChange={(e) =>
            handleChange("hue_offset", parseFloat(e.target.value))
          }
        />
      </div>

      {/* Life Decay */}
      <div className="control">
        <label>Life Decay: {settings.life_decay}</label>
        <input
          type="range"
          min="0"
          max="5"
          step="0.1"
          value={settings.life_decay ?? 0.1}
          onChange={(e) =>
            handleChange("life_decay", parseFloat(e.target.value))
          }
        />
      </div>

      {/* Rotate Speed */}
      <div className="control">
        <label>Rotate Speed: {settings.rotate_speed}</label>
        <input
          type="range"
          min="0"
          max="100"
          step="1"
          value={settings.rotate_speed ?? 0}
          onChange={(e) =>
            handleChange("rotate_speed", parseFloat(e.target.value))
          }
        />
      </div>

      <button onClick={() => invoke("hide_control")}>Close</button>
    </main>
  );
}

export default App;