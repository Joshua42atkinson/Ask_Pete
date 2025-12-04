import { useState } from 'react';

export interface WordDefinition {
    word: string;
    definition: string;
    power: string;
}

interface WordSmithyProps {
    onClose: () => void;
    onSave: (word: WordDefinition) => void;
}

export default function WordSmithy({ onClose, onSave }: WordSmithyProps) {
    const [word, setWord] = useState('');
    const [definition, setDefinition] = useState('');
    const [power, setPower] = useState('');

    const handleSave = () => {
        if (word.trim()) {
            onSave({ word, definition, power });
            onClose();
        }
    };

    return (
        <div className="modal-overlay">
            <div className="modal-content">
                {/* Header */}
                <div className="modal-header">
                    <h2 className="modal-title">
                        <span>⚒️</span>
                        The Word Smithy
                    </h2>
                    <button className="close-btn" onClick={onClose} aria-label="Close">✕</button>
                </div>

                {/* Body */}
                <div className="modal-body">
                    {/* Word */}
                    <div className="form-group">
                        <label className="form-label">Word</label>
                        <input
                            type="text"
                            className="form-input"
                            placeholder="e.g. Friction"
                            value={word}
                            onChange={(e) => setWord(e.target.value)}
                        />
                    </div>

                    {/* Definition */}
                    <div className="form-group">
                        <label className="form-label">Definition</label>
                        <textarea
                            className="form-textarea no-resize"
                            placeholder="The resistance that one surface or object encounters when moving over another."
                            value={definition}
                            onChange={(e) => setDefinition(e.target.value)}
                            rows={3}
                        />
                    </div>

                    {/* Power (Effect) */}
                    <div className="form-group">
                        <label className="form-label">Power (Effect)</label>
                        <input
                            type="text"
                            className="form-input"
                            placeholder="e.g. Slows movement by 50%"
                            value={power}
                            onChange={(e) => setPower(e.target.value)}
                        />
                        <p className="text-subtle">Define what happens when this word is invoked in the world.</p>
                    </div>
                </div>

                {/* Footer */}
                <div className="modal-footer">
                    <button className="modal-btn-cancel" onClick={onClose}>Cancel</button>
                    <button
                        className="modal-btn-primary"
                        disabled={!word.trim()}
                        onClick={handleSave}
                    >
                        Forge Word
                    </button>
                </div>
            </div>
        </div>
    );
}
