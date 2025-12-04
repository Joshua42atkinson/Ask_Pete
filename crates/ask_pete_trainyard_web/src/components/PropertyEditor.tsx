import { useState } from 'react';
import type { Node } from 'reactflow';
import type { NodeData, NarrativeChoice, StoryGraph, BlueprintResponse } from '../types/NarrativeGraph';

interface PropertyEditorProps {
    selectedNode: Node | null;
    onNodeChange: (id: string, data: NodeData) => void;
    onImportGraph: (graph: StoryGraph) => void;
    onClose: () => void;
}

export default function PropertyEditor({ selectedNode, onNodeChange, onImportGraph, onClose }: PropertyEditorProps) {
    const [speaker, setSpeaker] = useState(selectedNode?.data.speaker || '');
    const [text, setText] = useState(selectedNode?.data.text || '');
    const [choices, setChoices] = useState<NarrativeChoice[]>(selectedNode?.data.choices || []);
    const [isGenerating, setIsGenerating] = useState(false);

    const handleSave = () => {
        if (selectedNode) {
            onNodeChange(selectedNode.id, {
                ...selectedNode.data,
                speaker,
                text,
                choices
            });
        }
    };

    const addChoice = () => {
        setChoices([...choices, { text: 'New Choice', conditions: [] }]);
    };

    const updateChoice = (index: number, field: keyof NarrativeChoice, value: string) => {
        const newChoices = [...choices];
        newChoices[index] = { ...newChoices[index], [field]: value };
        setChoices(newChoices);
    };

    const removeChoice = (index: number) => {
        const newChoices = choices.filter((_, i) => i !== index);
        setChoices(newChoices);
    };

    const handleGenerateBlueprint = async () => {
        if (!selectedNode || !selectedNode.data.literary_device) return;

        setIsGenerating(true);
        try {
            const response = await fetch('/api/architect/blueprint', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    subject: selectedNode.data.speaker || "General",
                    focus: 0.5,
                    literary_device: selectedNode.data.literary_device,
                    vocabulary: []
                })
            });

            if (response.ok) {
                const data: BlueprintResponse = await response.json();
                onImportGraph(data.graph);
                alert(`Blueprint Generated: ${data.reasoning}`);
            } else {
                const err = await response.text();
                alert(`Failed to generate blueprint: ${err}`);
            }
        } catch (error) {
            console.error(error);
            alert('Error generating blueprint');
        } finally {
            setIsGenerating(false);
        }
    };

    if (!selectedNode) return null;

    return (
        <div className="inspector-panel">
            <div className="inspector-header">
                <div className="inspector-title">Station Control</div>
                <button onClick={onClose} className="close-btn" aria-label="Close editor">×</button>
            </div>

            <div className="form-group">
                <label htmlFor="speaker-input" className="form-label">Speaker (Station Name)</label>
                <input
                    id="speaker-input"
                    type="text"
                    className="form-input"
                    value={speaker}
                    onChange={(e) => setSpeaker(e.target.value)}
                    onBlur={handleSave}
                />
            </div>

            <div className="form-group">
                <label htmlFor="text-input" className="form-label">Announcement (Content)</label>
                <textarea
                    id="text-input"
                    className="form-textarea"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                    onBlur={handleSave}
                    rows={4}
                />
            </div>

            <div className="form-group">
                <label htmlFor="literary-device-select" className="form-label">Blueprint (Lesson Structure)</label>
                <select
                    id="literary-device-select"
                    className="form-input"
                    value={selectedNode.data.literary_device || ''}
                    onChange={(e) => onNodeChange(selectedNode.id, { ...selectedNode.data, literary_device: e.target.value })}
                >
                    <option value="">None (Standard Linear)</option>
                    <option value="Blueprint: Hero's Journey">Hero's Journey (5 Chapters)</option>
                    <option value="Blueprint: Mystery">Mystery / Problem Solving (5 Chapters)</option>
                    <option value="Blueprint: Collaborative">Team Relay (5 Chapters)</option>
                    <option value="Blueprint: Conflict">Debate / Dialectic (5 Chapters)</option>
                    <option value="Blueprint: Stress Relief">Decompression Loop (5 Chapters)</option>
                    <option value="Blueprint: Intuition">Dark Territory (Heuristic Run)</option>
                    <option value="Blueprint: Self-Correction">Governor Recalibration (PID Loop)</option>
                </select>
                {selectedNode.data.literary_device && (
                    <button
                        onClick={handleGenerateBlueprint}
                        disabled={isGenerating}
                        className={`mt-2 w-full py-1 px-2 rounded text-xs font-bold uppercase tracking-wider ${isGenerating ? 'bg-slate-600 cursor-not-allowed' : 'bg-blue-600 hover:bg-blue-500 text-white'}`}
                    >
                        {isGenerating ? 'Architecting...' : 'Generate Blueprint'}
                    </button>
                )}
            </div>

            {selectedNode.data.quest && (
                <div className="form-group p-3 bg-slate-900 border border-slate-700 rounded">
                    <div className="text-xs font-bold text-yellow-500 uppercase mb-2">Active Quest Data</div>
                    <div className="text-sm text-white mb-1"><span className="text-slate-400">Title:</span> {selectedNode.data.quest.title}</div>
                    <div className="text-sm text-white mb-1"><span className="text-slate-400">Theme:</span> {selectedNode.data.quest.chapter_theme}</div>
                    <div className="text-xs text-slate-400 italic">{selectedNode.data.quest.description}</div>
                </div>
            )}

            <div className="choices-section">
                <div className="inspector-header mb-2 border-none">
                    <label className="form-label mb-0">Routes (Choices)</label>
                    <button onClick={addChoice} className="btn-small" aria-label="Add choice">+ Add Route</button>
                </div>
                <div className="choices-list">
                    {choices.map((choice, index) => (
                        <div key={index} className="choice-item">
                            <input
                                type="text"
                                className="form-input"
                                value={choice.text}
                                onChange={(e) => updateChoice(index, 'text', e.target.value)}
                                onBlur={handleSave}
                                aria-label={`Choice ${index + 1} text`}
                            />
                            <button onClick={() => removeChoice(index)} className="btn-icon btn-remove-choice" aria-label={`Remove choice ${index + 1}`}>🗑️</button>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}
