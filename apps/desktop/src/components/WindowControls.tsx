import { invoke } from "@tauri-apps/api/core";
import type { MouseEvent } from "react";

type WindowControlsProps = {
  variant?: "full" | "mini";
};

export function WindowControls({ variant = "full" }: WindowControlsProps) {
  function startWindowDrag(event: MouseEvent<HTMLElement>) {
    if (event.button !== 0) {
      return;
    }

    void invoke("start_window_drag").catch((error) => {
      console.error("Failed to start window drag", error);
    });
  }

  function minimizeWindow(event: MouseEvent<HTMLButtonElement>) {
    event.stopPropagation();
    void invoke("minimize_window").catch((error) => {
      console.error("Failed to minimize window", error);
    });
  }

  function closeWindow(event: MouseEvent<HTMLButtonElement>) {
    event.stopPropagation();
    void invoke("close_window").catch((error) => {
      console.error("Failed to close window", error);
    });
  }

  if (variant === "mini") {
    return (
      <div className="window-actions window-actions-mini" aria-label="Window controls">
        <button
          className="window-action window-action-mini"
          type="button"
          aria-label="Minimize"
          onMouseDown={(event) => event.stopPropagation()}
          onClick={minimizeWindow}
        >
          _
        </button>
        <button
          className="window-action window-action-mini"
          type="button"
          aria-label="Close"
          onMouseDown={(event) => event.stopPropagation()}
          onClick={closeWindow}
        >
          x
        </button>
      </div>
    );
  }

  return (
    <header className="window-controls" onMouseDown={startWindowDrag}>
      <span className="window-grip">Token Tamagotchi</span>
      <div className="window-actions">
        <button
          className="window-action"
          type="button"
          aria-label="Minimize"
          onMouseDown={(event) => event.stopPropagation()}
          onClick={minimizeWindow}
        >
          _
        </button>
        <button
          className="window-action"
          type="button"
          aria-label="Close"
          onMouseDown={(event) => event.stopPropagation()}
          onClick={closeWindow}
        >
          x
        </button>
      </div>
    </header>
  );
}
