import React from 'react';
import ReactDOM from 'react-dom/client';
import { initializeMathCore } from './engine/axiomCore';
import { App } from './App';
import './index.css';

initializeMathCore().then(() => {
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
}).catch(() => {
  const root = document.getElementById('root')!;
  root.textContent = 'AXIOM could not load the mathematics engine. Your saved progress is unchanged. Check the connection and reload.';
});

if (import.meta.env.PROD && 'serviceWorker' in navigator && window.location.protocol.startsWith('http')) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js').catch((err) => {
      console.info('Service Worker registration skipped or failed:', err);
    });
  });
}
