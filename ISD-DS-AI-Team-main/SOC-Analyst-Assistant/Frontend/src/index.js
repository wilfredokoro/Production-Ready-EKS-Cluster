import React from 'react';

// import './index.css';  // Optional CSS file for styling
import App from './App';  // Your main app component

import { createRoot } from 'react-dom/client';
const container = document.getElementById('root');
const root = createRoot(container);
root.render(<App />);
