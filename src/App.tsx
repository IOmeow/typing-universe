import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type SettingsResponse = {
  particle_size: number;
  gravity_strength: number;
  hue_offset: number;
};

function App() {
  const [particleSize, setParticleSize] = useState(20);
  const [gravityStrength, setGravityStrength] = useState(1.0);
  const [hueOffset, setHueOffset] = useState(0);

  useEffect(() => {
    invoke<SettingsResponse>("get_settings").then((payload) => {
      setParticleSize(payload.particle_size);
      setGravityStrength(payload.gravity_strength);
      setHueOffset(payload.hue_offset);
    });
  }, []);

  const updateSettings = useCallback(
    (() => {
      let timer: ReturnType<typeof setTimeout>;
      return (newValues: { particle_size?: number; gravity_strength?: number; hue_offset?: number; }) => {
        clearTimeout(timer);
        timer = setTimeout(() => {
          invoke("update_settings", { newSettings: newValues });
        }, 50);
      };
    })(),
    []
  );

  return (
    <main className="container">
      <h3>Visual Control Panel</h3>

      {/* Particle Size */}
      <div className="control">
        <label>Particle Size: {particleSize}</label>
        <input
          type="range"
          min="5"
          max="100"
          step="1"
          value={particleSize}
          onChange={(e) => {
            const value = parseFloat(e.target.value);
            setParticleSize(value);
            updateSettings({ particle_size: value });
          }}
        />
      </div>

      {/* Gravity */}
      <div className="control">
        <label>Gravity Strength: {gravityStrength}</label>
        <input
          type="range"
          min="0.5"
          max="10"
          step="0.5"
          value={gravityStrength}
          onChange={(e) => {
            const value = parseFloat(e.target.value);
            setGravityStrength(value);
            updateSettings({ gravity_strength: value });
          }}
        />
      </div>

      {/* Hue Offset */}
      <div className="control">
        <label>Hue Offset: {hueOffset}</label>
        <input
          type="range"
          min="0"
          max="360"
          step="1"
          value={hueOffset}
          onChange={(e) => {
            const value = parseFloat(e.target.value);
            setHueOffset(value);
            updateSettings({ hue_offset: value });
          }}
        />
      </div>
      <button onClick={() => invoke("hide_control")}>Close</button>
    </main>
  );
}

export default App;