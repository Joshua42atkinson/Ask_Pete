# Testing & Verification Guide

> "Trust, but verify."

This document outlines how to test the core features of the *Ask Pete* system, including the new "Gap Filling" mechanics (Teleport, Quest Log, Voice).

## 1. The Trinity (System Launch)

Before testing, ensure the entire system is running.

**Windows:**

```powershell
./start_trinity.bat
```

**Manual:**

- Backend: `http://localhost:3000`
- Researcher: `http://localhost:8081`
- Teacher: `http://localhost:8082`
- Player: `http://localhost:8083`

---

## 2. Feature Verification

### A. The Legs (Teleport & GPS)

**Goal:** Verify the player can move around the physical campus without walking.

1. **Open Player App:** Go to `http://localhost:8083`.
2. **Open DevTools:** Click the 🛠️ icon in the top-right corner.
3. **Enable Override:** Toggle "Manual Location Override" to **ON**.
4. **Teleport:** Click the "📍 The Bell Tower" button.
5. **Verify:**
    - The `GpsHud` (top center) should update coordinates.
    - The "Current Zone" text should change to "The Bell Tower" (green text).

### B. The Senses (Quest Log)

**Goal:** Verify the player can see their current objective.

1. **Open Player App:** Go to `http://localhost:8083`.
2. **Locate Log:** Look at the top-left corner for the "Current Objective" panel.
3. **Verify:**
    - It should display a Quest Title (e.g., "The Way is Shut").
    - It should show a Step Description.
    - It should show "SYNCED" in the header, indicating a live connection to the backend.

### C. The Voice (Voice Input)

**Goal:** Verify the player can speak to Pete.

1. **Enter Play Mode:** Navigate to a scenario (e.g., `/journey/1`) or click "Depart Station".
2. **Locate Mic:** Find the 🎙️ button in the bottom-left.
3. **Interact:** Click the button.
4. **Verify:**
    - The button turns **Red** and pulses.
    - Text "Listening..." appears.
    - After 3 seconds, a Toast Notification appears confirming the "Voice Command".

### D. The Memory (Database Persistence)

**Goal:** Verify that progress is saved.

1. **Check Logs:** In the backend terminal, look for `INSERT INTO word_usage_logs` messages when unlocking words.
2. **Restart Server:** Stop and restart the backend.
3. **Verify:** The Quest Log in the Player App should show the same progress as before (if the database is connected).

---

## 3. Troubleshooting

### "Connection Refused"

- Ensure the backend is running on port 3000.
- Check if `start_trinity.bat` launched all windows successfully.

### "Quest Log says 'Waiting...'"

- The frontend cannot reach the backend API.
- Check the browser console (F12) for network errors.
- Ensure the proxy in `Trunk.toml` matches the backend port (3000).

### "Google Login Fails"

- Ensure you are accessing the app via `localhost` on the correct port (8081, 8082, or 8083).
- Verify these ports are added to your Google Cloud Console "Authorized Redirect URIs".
