import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";
import AnimationLayerProvider from "./animation/provider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AnimationLayerProvider>
      <App />
    </AnimationLayerProvider>
  </React.StrictMode>,
);
