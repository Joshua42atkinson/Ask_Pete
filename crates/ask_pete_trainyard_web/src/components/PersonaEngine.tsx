

interface PersonaEngineProps {
    onClose: () => void;
}

export default function PersonaEngine({ onClose }: PersonaEngineProps) {
    return (
        <div className="modal-overlay">
            <div className="modal-content large">
                {/* Header */}
                <div className="modal-header">
                    <h2 className="modal-title">
                        <span>🎭</span>
                        Persona Engine
                    </h2>
                    <button className="close-btn" onClick={onClose} aria-label="Close">✕</button>
                </div>

                {/* Body */}
                <div className="modal-body">
                    <div className="persona-placeholder">
                        <div className="text-4xl mb-4">🤖</div>
                        <h3 className="text-lg font-bold text-white mb-2">Persona System Under Construction</h3>
                        <p className="text-sm text-slate-400">
                            The Persona Engine will allow you to define AI archetypes, emotional states, and memory banks for your characters.
                        </p>
                    </div>

                    <div className="persona-grid">
                        <div className="persona-card">
                            <h4>Archetypes</h4>
                            <p>Define core personality traits and behavioral patterns.</p>
                        </div>
                        <div className="persona-card">
                            <h4>Memory Banks</h4>
                            <p>Manage shared knowledge and character-specific history.</p>
                        </div>
                    </div>
                </div>

                {/* Footer */}
                <div className="modal-footer">
                    <button className="modal-btn-cancel" onClick={onClose}>Close</button>
                </div>
            </div>
        </div>
    );
}
