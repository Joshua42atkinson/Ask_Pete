# Ask Pete - LDT Capstone Report Guide

## Executive Summary

**Title:** Ask Pete: A Sovereign, Headless Instructional Engine for Transformational Learning
**Thesis:** "Ask Pete" bridges the "Edutainment Gap" by combining the pedagogical rigor of an LMS with the narrative immersion of a video game, all within a privacy-first, institutionally sovereign architecture.

---

## Part 1: The Problem & Philosophy (The "Why")

### 1.1 The Edutainment Gap

* **The Conflict:** Current EdTech forces a choice between **Pedagogy** (Canvas/Blackboard - rigorous but boring) and **Engagement** (Minecraft/Roblox - fun but hard to assess/control).
* **The Solution:** A "Headless" engine that separates the *learning logic* (pedagogy) from the *presentation layer* (graphics), allowing for "Gamutainment" - entertainment-grade engagement with rigorous learning outcomes.

### 1.2 Pedagogical Foundations

* **Constructivism (Vygotsky):** Learning occurs through social interaction and scaffolding. "Pete" (the AI) acts as the *More Knowledgeable Other (MKO)*.
* **Projective Identity Dissonance (James Paul Gee):** Learning happens when a student must bridge the gap between their real self and a projected identity (e.g., "I am a novice" vs. "I am an ethical engineer").
* **Self-Determination Theory (Ryan & Deci):**
  * **Autonomy:** Student chooses their path/archetype.
  * **Competence:** "Vocabulary as a Mechanic" provides tangible proof of growth.
  * **Relatedness:** Interaction with NPCs and the "AI Mirror".

### 1.3 The "AI as a Mirror" Concept

* **Metacognition:** Instead of giving answers, the AI reflects the student's thinking back to them.
* **Socratic Method:** The AI asks probing questions ("Why did you choose the Utilitarian option?") to force deep reflection.
* **Identity Work:** The system tracks not just *what* the student knows (grades), but *who* they are becoming (Virtue Topology).

---

## Part 2: Stakeholder Perspectives (The "Who")

### 2.1 The Instructional Designer (The Author)

* **Pain Point:** "I'm not a game developer. I can't code C# or C++."
* **The Solution:** The **Generative Seed Workflow**.
  * **Input:** The ID writes a simple text file (TOML) defining the Theme, Learning Goals, and Vocabulary.
  * **Process:** The Engine (AI) expands this seed into a full narrative world (Quests, NPCs, Dialogue).
  * **Benefit:** "Force Multiplier" - One ID can create a AAA-quality experience in hours, not months.

### 2.2 The Learner (The Player)

* **Experience:** "It feels like a game, but I'm learning."
* **Flow State:** The system manages **Cognitive Load** dynamically. If the student is confused (high load), the AI simplifies the language. If bored (low load), it introduces complexity.
* **Mechanic:** **Vocabulary-as-a-Mechanic (VaaM)**. To unlock a door or persuade an NPC, the student must correctly use a specific vocabulary word in context.

### 2.3 The Technologist (The Developer)

* **Architecture:** **"Headless" & "Split-Brain"**.
  * **Headless:** The game logic (Bevy ECS) runs on the server. The graphics (Leptos WASM) run in the browser.
  * **Split-Brain:** Heavy AI processing happens on a powerful server; the mobile app is just a lightweight remote control.
* **Stack:** Rust (Performance/Safety) + WebAssembly (Portability) + Local AI (Privacy).

* **Aesthetic:** Inspired by Purdue's engineering roots.
* **UI Elements:** Chamfered edges, mechanical switches, pressure gauges.
* **Philosophy:** The interface is a "tool," not a "toy." It respects the user's intelligence.

### 3.2 The AI Orchestrator

* **Local Inference:** Uses `candle` and `rocm-rs` to run LLMs (like Gemma 27B) on university hardware.
* **Orchestration:** Manages multiple AI "agents" (Narrator, NPC, Socratic Tutor) to create a coherent experience.

---

## Part 4: Future Roadmap

* **Phase 1 (Now):** Text-based Socratic interaction + Basic 3D navigation.
* **Phase 2:** Multi-modal interaction (Voice input/output).
* **Phase 3:** AR/VR integration for "in-situ" training (e.g., Firefighter training).

---

## Appendix: Key Terminology for the Report

* **Gamutainment:** The fusion of rigorous education and immersive entertainment.
* **Sovereign AI:** AI models owned and operated by the institution, not rented from a corporation.
* **Headless Engine:** A game engine that runs logic on the server and streams state to the client.
* **Virtue Topology:** A data visualization of a student's ethical/identity alignment over time.
