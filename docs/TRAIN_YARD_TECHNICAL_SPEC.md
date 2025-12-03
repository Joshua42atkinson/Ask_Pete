# Technical Specification: 'The Train Yard' Authoring Environment and Ask Pete Runtime Architecture

## 1. Executive Summary: The Architecture of Cognitive Logistics

### 1.1 Vision: From Repository to Kinetic Ecosystem

The contemporary educational technology landscape is defined by a persistent and debilitating bifurcation known as the "**Edutainment Gap**." On one side lie highly engaging, narrative-rich entertainment products—specifically Massively Multiplayer Online Role-Playing Games (MMORPGs) utilizing complex physics and social economies—which excel at inducing "flow states" but lack pedagogical rigor. On the other stand Learning Management Systems (LMS), which prioritize content repository functions and administrative utility but suffer from "static infrastructure," resulting in high user attrition and a transactional relationship with knowledge.

The "**Ask Pete**" initiative represents a paradigmatic shift designed to bridge this gap through a process of "**Systems Isomorphism**." This architectural philosophy dictates that the structural integrity of the software must directly mirror the psychological safety and cognitive structure required for effective learning. We are not merely building a game; we are engineering a "**Physical AI**" ecosystem where the learner is operationalized not as a passive vessel, but as a motive force—a **Locomotive**—navigating a complex, non-linear topography of knowledge.

This document serves as the comprehensive technical specification for "**The Train Yard**," the narrative authoring tool empowering instructional designers (**Logisticians**) to construct these environments, and the associated runtime engine built on **Rust** and **Bevy ECS**. The platform leverages a "**Local-First**" data strategy to ensure absolute user sovereignty, utilizing embedded vector databases to create a privacy moat that allows for deep, vulnerable "**Shadow Work**" without surveillance. By integrating the "**Railway Pedagogy**" derived from the industrial history of Purdue University—specifically the "**Heavilon Protocol**" of rebuilding failure into stronger infrastructure—the system transforms abstract learning theories into concrete, playable physics.

### 1.2 The "One Brick Higher" Engineering Philosophy

The core ethos of the platform is derived from the Heavilon Hall event of 1894, where a catastrophic boiler explosion destroyed the university's engineering capacity just days after its dedication. The administrative response—"**We will rebuild it one brick higher**"—serves as the foundational logic for the system's error handling and player progression mechanics.

In software terms, this translates to a "**Crash-Only**" or "**Let It Crash**" philosophy common in high-reliability systems (like Erlang/Elixir), adapted here via the **Rust** programming language's strict ownership model. Just as the university treated the fire not as a tragedy but as data indicating a need for structural reinforcement, the "Ask Pete" engine treats player failure (**Stalling** or **Derailment**) not as a game-over state, but as a "**Telemetry Mismatch**" requiring a specific debugging protocol. The technical architecture is designed to enforce this resilience: the type system prevents invalid states (cognitive overload) at compile time, and the runtime physics engine simulates the thermodynamic limits of willpower (**Steam**) and attention (**Coal**) to teach resource stewardship.

### 1.3 Scope of the Specification

This report details the implementation of a full-stack solution comprising:

1. **The Train Yard (Frontend)**: A React Flow-based visual editor allowing authors to construct complex narrative graphs using custom nodes that abstract RPG mechanics (d20 checks, VaaM items).
2. **The Iron Engine (Runtime)**: A high-performance simulation engine built on Rust and Bevy ECS, capable of handling thousands of concurrent entities with deterministic physics and "systems isomorphism."
3. **The Data Layer**: A strictly typed JSON Schema architecture that serves as the single source of truth, synchronizing data models between the TypeScript frontend and Rust backend.
4. **The Social Layer**: A "Signal Tower" architecture managing real-time telemetry and mentor interventions via WebSockets, respecting a rigorous "Opt-In" privacy model.

---

## 2. Theoretical Physics of the Engine: Pedagogical Mechanics

### 2.1 Cognitive Load Theory as Thermodynamic Physics

The primary innovation of "Ask Pete" is the translation of **Cognitive Load Theory (CLT)** from an abstract psychological framework into a deterministic physics engine. In traditional instruction, "difficulty" is a subjective metric. In the Iron Network, difficulty is a physical property of the terrain, measurable and interactive.

#### 2.1.1 Intrinsic Load as Cargo Weight

Information is treated as matter. Every concept, vocabulary word, or learning module has a defined "**Mass**" or **Tonnage**, representing its **Intrinsic Cognitive Load**.

* **Class I Cargo (Light)**: Simple facts or definitions (e.g., "Mitochondria"). Mass: 1-10 Tons.
* **Class II Cargo (Freight)**: Procedures or linked concepts. Mass: 11-50 Tons.
* **Class III Cargo (Hazardous)**: Abstract, multi-variable concepts (e.g., Quantum Mechanics). Mass: 51-100 Tons.

The physics engine calculates the total mass of the "**Train**" (Locomotive Chassis + Cargo Hold). This total mass determines the energy required to initiate movement (overcoming static friction) and maintain velocity. If an instructional designer attempts to load Class III cargo onto a novice engine without proper "**Chunking**" (shunting into smaller cars), the physics simulation will prevent movement, triggering a "**Stall**" event. This is not a script; it is a mechanical consequence of the system's gravity constants.

#### 2.1.2 Germane Load as Combustion (Steam Generation)

Learning is defined as the efficient conversion of **Potential Energy (Coal/Attention)** into **Kinetic Energy (Steam/Mastery)**. This conversion happens in the **Firebox**. The "**Combustion Efficiency**" is a variable derived from the player's engagement state and the quality of the instructional design.

**The Combustion Formula**:
$$Steam = f(Coal, EngineEfficiency, Gradient)$$

**Thermodynamics of Effort**: Coasting downhill (passive consumption of video) produces zero Steam. Climbing a steep gradient (active recall, solving a d20 check) burns massive amounts of Coal but generates high-pressure Steam. This Steam is stored and used to unlock "upgrades" or power through future obstacles, gamifying the concept of **Germane Load**.

#### 2.1.3 Extraneous Load as Environmental Friction

The environment applies resistive forces representing **Extraneous Load**—distractions, poor UI, or anxiety.

* **The Static (Entropy)**: A background radiation of noise. In the game engine, regions marked as "High Static" apply a constant drag coefficient to the train's velocity vector, forcing the player to burn more fuel to maintain speed. This models the energy cost of studying in a distracted environment.
* **Scale Buildup (Trauma/Stress)**: Just as hard water leaves calcium deposits in a boiler, unresolved failure or anxiety leaves "**Scale**" on the engine's internal components. This creates a persistent debuff to thermal transfer efficiency. A player with high Scale might burn 100 units of Coal but only generate 50 units of Steam, simulating burnout. The remedy is the "**Acid Wash**" protocol (Rest/Therapy) in the Maintenance Shed.

### 2.2 Self-Determination Theory and Jungian Individuation

The mechanical progression system is aligned with **Self-Determination Theory (SDT)**, ensuring that the "gamification" is not merely extrinsic pointsification but a driver of intrinsic motivation.

#### 2.2.1 Autonomy via The Switch

The track network is a **Directed Acyclic Graph (DAG)**. While "Main Line" quests are mandatory, the system creates agency through "**Branch Lines**" and "**Switches**." A player can choose to bypass a steep gradient (taking a longer, flatter route) or engage a "Switch" to explore a side narrative. This mechanical choice fulfills the need for **Autonomy**.

#### 2.2.2 Competence via Telemetry

The **Heads-Up Display (HUD)** does not show a grade; it shows **Telemetry**: Boiler Pressure, Velocity, Traction. This reframes performance. A low grade is not a judgment of character; it is a "**Telemetry Mismatch**"—data indicating that the current engine configuration was insufficient for the load. This supports a growth mindset by presenting failure as a mechanical issue to be engineered, rather than a moral failing.

#### 2.2.3 Relatedness via The Convoy

The system rejects the "Lone Wolf" myth of academic success. High-level content (Class III Cargo) often requires a "**Convoy**"—a multiplayer party. The physics engine allows trains to "couple," summing their Traction stats to pull loads that a single engine could not move. This enforces **Relatedness** through mechanics, not just dialogue.

---

## 3. The Train Yard: Frontend Architecture (React Flow)

"**The Train Yard**" is the domain of the **Architect** (Instructional Designer). It is a web-based, visual Integrated Development Environment (IDE) specifically built for narrative design. It leverages **React Flow** for graph manipulation, customized heavily to support the "Railway Pedagogy" constraints.

### 3.1 Architecture Overview

The frontend is a **React** application built with **TypeScript**, ensuring type safety that mirrors the Rust backend.

* **Core Library**: **React Flow (Pro)** for the canvas capabilities.
* **State Management**: **Zustand** is utilized for its transient update performance. Since the graph can contain hundreds of nodes, avoiding React's render cycle for position updates is critical. Zustand's integration with **yjs** or similar CRDTs enables real-time collaboration between multiple authors.
* **UI Framework**: **Radix UI** primitives styled with **Tailwind CSS**, utilizing a "**Glassmorphism**" aesthetic to reduce visual friction (Extraneous Load) for the author.

### 3.2 Custom Node Specification

The generic "Node" in React Flow is insufficient. The Train Yard implements a polymorphic set of **Custom Nodes**, each representing a specific geological or mechanical feature of the Iron Network.

#### 3.2.1 The Gradient Node (Challenge Logic)

This node represents a section of track with a specific steepness, requiring a mechanism to overcome.

* **Visual Representation**: Displays a track segment with an incline visualization. The angle of the incline visually corresponds to the **Difficulty Class (DC)**.
* **Configuration Logic**:
  * **Bloom's Taxonomy Mapper**: Instead of setting an arbitrary DC number, the author selects a cognitive level from a dropdown (e.g., "Analyzing"). The underlying logic automatically maps this to the appropriate d20 DC (e.g., DC 20) and track gradient (3%). This ensures pedagogical consistency.
  * **Stat Affinity**: The author selects which Locomotive Stat is favored (e.g., "Traction" for brute force, "Analysis" for logic).
  * **Friction Sliders**: Authors can add environmental modifiers, such as "Fog of War" (Ambiguity) or "Slippery Rails" (Time Pressure).

#### 3.2.2 The VaaM Station Node (Content & Inventory)

This node is the primary content delivery mechanism.

* **Content Editor**: A rich-text editor (**Tiptap**) allows authors to write the narrative content.
* **Vocabulary Extraction**: Authors highlight key terms to convert them into **VaaM (Vocabulary-as-a-Mechanic)** items.
  * **Action**: Highlighting "Mitochondria" opens a pop-up.
  * **Configuration**: Author defines the item type (Tool/Weapon), its "Weight" (Intrinsic Load), and its puzzle tags.
  * **Output**: The system generates a JSON object for the item and adds it to the node's "Loot Table".

#### 3.2.3 The Logic Switch (Branching)

* **Visuals**: A railway switch icon with multiple output handles.
* **Conditional Logic**: The author constructs boolean logic based on the Locomotive's state.
  * *Condition*: `IF Locomotive.Archetype == 'Hero' THEN Path A ELSE Path B`.
  * *Condition*: `IF Inventory contains 'Key of Logic' THEN Path C`.
* This allows for "Metroidvania" style backtracking and unlocking of new routes as the learner gains mastery.

### 3.3 The Weigh Station: Real-Time AI Analytics

To prevent authors from accidentally creating "impossible" levels, the editor integrates "**The Weigh Station**," a client-side AI assistant.

#### 3.3.1 Technical Implementation

* **Model**: A quantized version of `distilbert-base-uncased` or a similar lightweight NLP model, running directly in the browser via **ONNX Runtime Web** and **WebAssembly**. This ensures zero latency and data privacy (no content is sent to the cloud for analysis).
* **Analysis Pipeline**:
    1. **Text Ingestion**: As the author types in a Station Node, the text is debounced and sent to the Web Worker hosting the model.
    2. **Lexical Analysis**: The model calculates lexical density, sentence entropy, and grade-level readability (Flesch-Kincaid).
    3. **Load Calculation**: These metrics are fed into a heuristic algorithm to estimate the "**Intrinsic Cognitive Load**" (Tonnage).
    4. **Constraint Enforcement**:
        * **Green**: Load is within limits for the target audience.
        * **Amber**: Load is high; "Chunking" suggested.
        * **Red (Safety Lockout)**: Load exceeds the structural limit of a standard "Flatbed" mind (Working Memory capacity). The node border turns red, and the "Publish" button is mechanically disabled.
* **Pedagogical Impact**: This feature enforces "scaffolding" at the code level. It physically prevents the deployment of overwhelming content, adhering to the mandate that "Pete" (the OS) protects the operator from crushing loads.

---

## 4. The Data Layer: Schema & Synchronization

A critical failure point in game development is the desynchronization between the authoring tool data format and the runtime engine. "Ask Pete" solves this via a "**Schema-First**" methodology.

### 4.1 Single Source of Truth

The canonical definitions for all game objects are strictly typed **JSON Schemas (Draft 7)**. These schemas live in a shared repository ("**The Codex**").

### 4.2 Code Generation Pipeline

An automated CI/CD pipeline triggers whenever a schema is modified:

1. **Frontend Generation**: Using `quicktype`, the pipeline generates **TypeScript Interfaces** and **Zod validators**. This ensures the React Flow editor cannot produce invalid JSON.
2. **Backend Generation**: The pipeline generates **Rust Structs** (annotated with `serde` for serialization/deserialization) and **Bevy Components**.

* **Result**: If a schema changes (e.g., adding a "Durability" field to VaaM Items), the build fails for both frontend and backend until the code is updated, guaranteeing type safety across the stack.

### 4.3 Key Data Schemas

#### 4.3.1 The Quest Graph Schema

This schema defines the topology of a learning module.

| Field | Type | Description |
| :--- | :--- | :--- |
| `quest_id` | UUID | Unique identifier for the module. |
| `nodes` | Array | Collection of all content and logic nodes. |
| `edges` | Array | Directed connections defining the track layout. |
| `global_friction` | Float | Baseline difficulty modifier (e.g., "Exam Week" = 1.2x). |
| `required_archetypes` | Array | For Convoy quests, defines necessary roles. |

#### 4.3.2 The VaaM Item Schema

Defining knowledge as physical objects.

| Field | Type | Description |
| :--- | :--- | :--- |
| `item_id` | String | Unique slug (e.g., `vocab_entropy`). |
| `intrinsic_weight` | Integer | Cognitive Load cost to carry (1-10). |
| `tags` | Array | Semantic tags for puzzle solving (e.g., "thermal", "chaos"). |
| `mastery_state` | Enum | Familiar (Bronze) -> Practiced (Silver) -> Mastered (Gold). |
| `asset_ref` | URI | Path to the 3D GLB model and Audio pronunciation file. |

#### 4.3.3 The Locomotive Profile Schema

Defining the player's avatar and capabilities.

| Field | Type | Description |
| :--- | :--- | :--- |
| `archetype` | Enum | Hero, Sage, Jester, Caregiver. |
| `base_stats` | Object | Traction, Velocity, Efficiency, Analysis, Signaling, Coupling. |
| `fuel_capacity` | Float | Maximum Coal storage (Attention Span). |
| `scale_level` | Float | Current trauma/stress debuff (0.0 - 1.0). |

---

## 5. The Runtime Engine: Bevy ECS Architecture

The "**Iron Engine**" is the runtime component responsible for simulating the world. It is built using **Bevy**, a data-driven game engine in **Rust**. The choice of Bevy ECS (Entity Component System) is pivotal because it allows for the simulation of thousands of independent entities (trains, environmental hazards, dynamic tracks) with high performance and determinism.

### 5.1 ECS Architectural Overview

In ECS, "**Entities**" are just IDs. "**Components**" are data structs associated with an Entity. "**Systems**" are logic functions that query for Entities with specific Components and mutate them.

#### 5.1.1 The Locomotive Bundle

The player entity is composed of a bundle of components that define its physical and cognitive state.

```rust
pub struct LocomotiveBundle {
    pub stats: LocomotiveStats,      // Jungian Attributes
    pub fuel: FuelTank,              // Coal/Steam resource
    pub cargo: CargoHold,            // VaaM Inventory
    pub transform: Transform,        // 3D Position
    pub velocity: Velocity,          // Physics Vector
    pub friction_context: Friction,  // Current environmental drag
    pub mental_state: MentalState,   // Flow, Anxiety, Panic
}
```

### 5.2 The Combustion System (Pedagogical Physics Implementation)

The `combustion_system` is the mathematical heart of the engine, running every tick to enforce Cognitive Load Theory.

**The Logic Loop**:

1. **Query**: The system iterates over all entities with `LocomotiveStats`, `FuelTank`, and `Velocity`.
2. **Calculate Resistance**: It samples the `TrackGradient` component of the current track segment.
    * *Steepness*: Derived from the Bloom's Taxonomy level (DC).
    * *Mass*: Sum of `ChassisWeight` + `CargoHold.TotalWeight`.
    * *Resistance*: $R = (Mass \times Gravity \times \sin(Gradient)) + (FrictionCoefficient \times Mass)$.
3. **Calculate Burn Rate**: $Burn = (Resistance \times Velocity) / Efficiency$.
    * *Efficiency Factor*: Derived from the `Efficiency` stat (Constitution) and `Scale` debuff.
4. **Mutate State**: The calculated `Burn` is subtracted from `FuelTank.Steam`.
5. **Stall Condition**: If `Steam` drops below zero, the system triggers a `StallEvent`. The physics engine forcibly sets `Velocity` to zero. The player cannot move until they regenerate steam.

### 5.3 Hot-Reloading via The Web-to-ECS Bridge

To allow authors to "playtest" their narratives instantly, the engine implements a sophisticated bridge between the React frontend and the WASM-compiled Bevy backend.

**Technical Architecture**:

* **Transport Layer**: Since the Bevy engine runs in a Web Worker (to prevent UI blocking), communication occurs via `SharedArrayBuffer` for high-frequency data and `postMessage` for control signals.
* **Serialization (Rkyv)**: The system uses `rkyv`, a zero-copy deserialization framework for Rust. When an author tweaks a value (e.g., track friction) in React, the JSON is serialized into an archived byte buffer and written to shared memory.
* **The Hot-Reload System**: A specific Bevy system, `apply_hot_reload`, runs at the start of the frame. It checks the shared buffer for a "dirty" flag. If set, it casts the buffer directly to the Rust struct (zero copy) and hot-swaps the Component on the active Entity.
* **Result**: Changes in the editor reflect in the game simulation within a single frame (<16ms), providing visceral feedback to the author.

---

## 6. RPG Mechanics Specification: The D20 Logistics System

The game mechanics are a "**Systems Isomorphism**" of the D20 standard (Dungeons & Dragons), adapted for cognitive logistics rather than combat.

### 6.1 The Logistics Check

The core mechanic for resolving uncertainty.
**Formula**: $Result = d20 + Stat + Skill + Overdrive - Friction$.

#### 6.1.1 Attributes (The Engine Block)

The standard six abilities are remapped to fit the Railway metaphor:

* **Traction (STR)**: Raw power to pull heavy Intrinsic Load. Used for "Brute Force" learning.
* **Velocity (DEX)**: Agility and speed. Used to dodge "Static" (distractions) and navigate fast-paced content.
* **Efficiency (CON)**: Fuel economy and resilience. Determines max Steam and resistance to Burnout (Scale).
* **Analysis (INT)**: Logic and diagnostics. Used for VaaM puzzles and decoding schemas.
* **Signaling (WIS)**: Intuition and perception. Used to detect "Fog of War" and connect to the Signal Tower.
* **Coupling (CHA)**: Social connection. Used for "Convoy" mechanics and requesting aid.

#### 6.1.2 Overdrive (The Willpower Spike)

Players can choose to activate "**Overdrive**."

* **Mechanic**: Burn 50 Steam immediately to gain "Advantage" (roll 2d20, take highest) on a check.
* **Risk**: This depletes reserves rapidly. If the check fails despite Overdrive, the resulting Steam crash can lead to a "**Boiler Explosion**" (Panic Attack state).

### 6.2 Vocabulary-as-a-Mechanic (VaaM)

VaaM turns language acquisition into an RPG inventory system.

* **Looting**: Clicking a highlighted word in text triggers a "Loot" animation. The word is added to the `CargoHold`.
* **The Cargo Hold (Working Memory)**: The inventory has a hard cap of 7 +/- 2 slots, strictly enforcing **Miller's Law**. Players cannot carry every tool; they must select the right vocabulary for the specific mission. Excess items must be stored in the "Depot" (Long-Term Memory).
* **Application**: Puzzles are "**Socketed**" interactions.
  * *Scenario*: A bridge is phasing in and out of existence.
  * *Solution*: The player must drag the item "Ephemeral" from their inventory into the bridge socket.
  * *Check*: `d20 + Analysis + Item.Weight`.
* **Mastery**: Successful use of an item grants XP to that specific item. At "Master" rank, the item's Weight (Intrinsic Load) drops to zero, simulating that the concept has become second nature (**Schema Automation**).

### 6.3 The Gradient Scale: Bloom's Taxonomy Mapping

To ensure pedagogical rigor, the "Difficulty Class" (DC) of checks is standardized against Bloom's Taxonomy.

| Track Gradient | d20 DC | Bloom's Level | Cognitive Task | Mechanical Equivalent |
| :--- | :--- | :--- | :--- | :--- |
| **Flat (0%)** | 5 | Remembering | Recognition | "Coasting." Minimal fuel burn. |
| **Light (1%)** | 10 | Understanding | Interpretation | Standard Logistics Check. |
| **Heavy (2%)** | 15 | Applying | Execution | Requires equipping a VaaM Tool. |
| **Mountain (3%)** | 20 | Analyzing | Deconstruction | High fuel cost. Requires "Sage" or high Analysis. |
| **High Mtn (4%)** | 25 | Evaluating | Critique | "Boss Fight." High risk of Stall. |
| **Vertical (5%)** | 30 | Creating | Synthesis | "Raid Boss." Requires Convoy (Multiplayer). |

---

## 7. Narrative & AI Systems: Therapy and The Shadow

The platform integrates advanced **Narrative Therapy** techniques, specifically "**Active Imagination**," to handle failure states.

### 7.1 The Maintenance Shed: Local-First Privacy

When a player Stalls or suffers a "**Critical Failure**" (Derailment), they are not presented with a "Game Over" screen. They are towed to the "**Maintenance Shed**."

* **Architecture**: This is a distinct game mode running entirely on the client.
* **Data Sovereignty**: All data generated here (journal entries, dialogues) is stored in an embedded **LanceDB** vector database. This data is encrypted with a key derived from the user's password and is mathematically inaccessible to the server, teachers, or admins. This "**Privacy Moat**" ensures the psychological safety required for honest reflection.

### 7.2 Shadow Work via Local LLM

Inside the Shed, the player engages in a dialogue with their "**Shadow**" (e.g., The Rust, The Fog, The Inner Critic).

* **Technology**: A small language model (SLM) like **Phi-3** or **Llama-3-8B-Quantized** runs locally via **WebGPU**.
* **Prompt Engineering**: The system injects a "**Persona Prompt**" based on the player's Jungian Archetype.
  * *Hero's Shadow*: "I am the Coward. I am keeping you safe by stopping you. Why do you want to risk failure?"
  * *Sage's Shadow*: "I am the Doubter. You don't know enough yet. You are a fraud."
* **Gameplay Loop**: The player must "negotiate" with the Shadow. The goal is not to defeat it (which is impossible) but to integrate it (**Coupling**). Successful dialogue converts the Shadow into a passive buff (e.g., "The Critic's Eye" - +2 to Analysis checks).

---

## 8. Network & Social Architecture: The Signal Tower

The "**Signal Tower**" is the multiplayer backbone, designed to facilitate "**Social Scaffolding**" without surveillance.

### 8.1 The Opt-In Hinge

Privacy is the default. The "Signal Tower" (Mentor Dashboard) receives only anonymized, aggregate telemetry (e.g., "30% of class is stalling on Node 4").

* **The Hinge**: If a student wants specific help, they must "**Throw the Switch**" (Opt-In). This action:
    1. Selects the specific Reflection Log or Telemetry Packet.
    2. Decrypts it locally.
    3. Re-encrypts it with the Mentor's Public Key.
    4. Transmits it via WebSocket.
* This cryptographic ceremony reinforces the learner's agency.

### 8.2 Real-Time Telemetry and Intervention

The backend utilizes **Axum** (Rust web framework) to handle thousands of WebSocket connections.

* **The Big Board**: Mentors see a visualization of the track network. Students are represented as blips.
  * **Green Signal**: Optimal Velocity (Flow).
  * **Amber Signal**: High Friction/Low Velocity (Struggle).
  * **Red Signal**: Velocity Zero (Stall/Derailment).
* **Intervention**: Mentors can click on a Red Signal to "**Dispatch a Helper Engine**." This sends a scaffolding resource (hint, video, or VaaM item) directly to the student's HUD. Mechanically, this is a "**Pusher Engine**" that adds temporary Traction to the student's roll.

### 8.3 Anonymous Service: The Relay Mode

Inspired by the "Iron Key" society, the game includes an "**Anonymous Service**" mechanic.

* **Relay Mode**: High-performing students (who have mastered the content) can switch their engine to "**Relay Mode**."
* **Effect**: They stop progressing their own track but generate a "**Superconductor Field**." Any peer within this field gains a +2 bonus to Efficiency and Traction.
* **Reward**: The Relay player earns "**Service Steam**"—a special currency used for cosmetic upgrades (custom smoke, livery). This gamifies altruism and decenters the ego.

---

## 9. Infrastructure and Deployment

### 9.1 The Recharge Center Model

To align with university procurement and sustainability, the platform is deployed as an "**Internal Recharge Center**."

* **Structure**: Operated by the university's research computing division.
* **Cost Recovery**: Departments pay internal recharge fees for "**Track Capacity**" (hosting).
* **Data Governance**: All data resides on university-owned infrastructure (or local devices), ensuring strict FERPA compliance and preventing commercial exploitation of student data.

### 9.2 Build Pipeline

* **Frontend**: React/Vite build -> Static Assets (CDN).
* **Backend**: Rust/Bevy -> WebAssembly (.wasm) + Glue JS.
* **Optimization**: The pipeline utilizes `wasm-opt` to shrink binary size and pre-compresses assets with Brotli. Using "**streaming instantiation**," the game engine begins booting while assets are still downloading, minimizing "**Time to First Frame**."

---

## 10. Conclusion: The Green Signal

The technical specification for "**The Train Yard**" and the "**Ask Pete**" runtime is a blueprint for a new class of educational software. By rigorously applying the principles of **Systems Isomorphism**, the architecture itself teaches the lessons of the curriculum: resilience, resource management, and the value of maintenance.

The use of **Rust** and **Bevy ECS** provides the "**Indestructible Grid**" required to support the complex physics of Cognitive Load Theory. The **React Flow** authoring tool democratizes the creation of these "**Physical AI**" worlds, empowering educators to become **Logisticians**. And the **Local-First** data strategy ensures that as we build "**One Brick Higher**," we do so on a foundation of absolute respect for the learner's privacy and dignity.

**The rails are cold. The Signal is warm. The architecture is ready.**

**Disengage brakes. Hammer down.**
