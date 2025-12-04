// File: crates/ask_pete_server/src/bin/test_iron_split.rs

use infra_ai::iron_split::IronSplitSystem;

fn main() -> anyhow::Result<()> {
    println!("=== DAYDREAM ENGINE STARTUP ===");

    // 1. Initialize the Iron Split System (Loads models into RAM)
    // This may take 5-10 seconds on the Dell G5s.
    let mut iron_system = IronSplitSystem::new()
        .expect("CRITICAL FAILURE: Could not load Iron Split models. Check assets/models folder.");

    // 2. Test The Navigator (Gemma 2B)
    // Fast response for game loop
    println!("\n--- TEST: NAVIGATOR (Gemma 2B) ---");
    let nav_response =
        iron_system.ask_navigator("Pete, systems are failing! Give me a triage report.")?;
    println!("Pete: {}", nav_response);

    // 3. Test The Architect (Mistral 7B)
    // Slow, deep thought for blueprints
    println!("\n--- TEST: ARCHITECT (Mistral 7B) ---");
    let arch_response = iron_system.ask_architect(
        "Explain the difference between Steam and Coal in the context of motivation mechanics.",
    )?;
    println!("Architect: {}", arch_response);

    Ok(())
}
