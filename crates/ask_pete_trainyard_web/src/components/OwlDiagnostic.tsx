import React, { useMemo } from 'react';
import { type Node } from 'reactflow';
import type { NodeData } from '../types/NarrativeGraph';

interface OwlDiagnosticProps {
    nodes: Node<NodeData>[];
    onClose: () => void;
}

interface Issue {
    id: string;
    title: string;
    message: string;
    colorClass: string;
}

const OwlDiagnostic: React.FC<OwlDiagnosticProps> = ({ nodes, onClose }) => {
    const { issues, systemIntegrity } = useMemo(() => {
        const currentIssues: Issue[] = [];
        let totalTorque = 0.0;
        let totalFriction = 0.0;

        nodes.forEach((node) => {
            const { speaker, text, choices } = node.data;

            // 1. Torque Check (Content Density)
            const wordCount = text ? text.split(/\s+/).length : 0;
            if (wordCount < 5) {
                currentIssues.push({
                    id: node.id,
                    title: speaker || 'Untitled',
                    message: 'Low Torque (Content too sparse)',
                    colorClass: 'bg-yellow-900/50 text-yellow-200 border-yellow-700',
                });
            } else if (wordCount > 100) {
                currentIssues.push({
                    id: node.id,
                    title: speaker || 'Untitled',
                    message: 'Scale Buildup (Too much text)',
                    colorClass: 'bg-red-900/50 text-red-200 border-red-700',
                });
                totalFriction += 1.0;
            } else {
                totalTorque += 1.0;
            }

            // 2. Friction Check (Complexity / Boiler Pressure)
            // Using choices length as a proxy for "Passenger Count" / Complexity
            const passengerCount = choices ? choices.length : 0;
            if (passengerCount > 4) {
                currentIssues.push({
                    id: node.id,
                    title: speaker || 'Untitled',
                    message: 'High Friction (Boiler Pressure Critical)',
                    colorClass: 'bg-red-900/50 text-red-200 border-red-700',
                });
                totalFriction += 2.0;
            }

            // 3. Signal-to-Noise (Empty Fields)
            if (!speaker || speaker.trim() === '' || speaker === 'New Station') {
                currentIssues.push({
                    id: node.id,
                    title: 'Untitled',
                    message: 'Weak Signal (Generic Title)',
                    colorClass: 'bg-blue-900/50 text-blue-200 border-blue-700',
                });
            }
        });

        let integrity = 100.0;
        if (nodes.length > 0) {
            const rawScore = totalTorque * 10.0 - totalFriction * 5.0;
            integrity = Math.max(0, Math.min(100, rawScore));
        }

        return { issues: currentIssues, systemIntegrity: integrity };
    }, [nodes]);

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm" onClick={onClose}>
            <div
                className="bg-slate-900 border-2 border-yellow-500 rounded-lg shadow-2xl w-[600px] max-h-[80vh] flex flex-col overflow-hidden"
                onClick={(e) => e.stopPropagation()}
            >
                {/* Header */}
                <div className="bg-slate-950 p-4 border-b border-slate-700 flex justify-between items-center">
                    <div className="flex items-center gap-3">
                        <div className="text-3xl">🦉</div>
                        <div>
                            <h2 className="text-xl font-bold text-yellow-500 font-mono">O.W.L. Diagnostic</h2>
                            <p className="text-xs text-slate-400 uppercase tracking-widest">Sector HSI // Optimization Matrix</p>
                        </div>
                    </div>
                    <button
                        className="text-slate-400 hover:text-white transition-colors text-xl"
                        onClick={onClose}
                    >
                        ✖
                    </button>
                </div>

                {/* Dashboard */}
                <div className="p-6 overflow-y-auto custom-scrollbar flex-1">
                    {/* System Integrity Gauge */}
                    <div className="mb-8">
                        <div className="flex justify-between text-sm mb-2 text-slate-300 font-mono">
                            <span>System Integrity</span>
                            <span>{systemIntegrity.toFixed(0)}%</span>
                        </div>
                        <div className="h-4 bg-slate-800 rounded-full overflow-hidden border border-slate-700">
                            <div
                                className="h-full transition-all duration-1000 ease-out bg-gradient-to-r from-red-500 via-yellow-500 to-green-500"
                                style={{ width: `${systemIntegrity}%` }}
                            />
                        </div>
                    </div>

                    {/* Issues List */}
                    <div className="space-y-3">
                        <h3 className="text-sm font-bold text-slate-400 uppercase tracking-wider mb-2">Diagnostic Log</h3>
                        {issues.length === 0 ? (
                            <div className="p-4 bg-green-900/20 border border-green-500/30 rounded text-green-300 flex items-center gap-3">
                                <span className="text-xl">✅</span>
                                <div>
                                    <div className="font-bold">All Systems Nominal</div>
                                    <div className="text-sm opacity-80">No friction detected. Boiler pressure stable.</div>
                                </div>
                            </div>
                        ) : (
                            issues.map((item, index) => (
                                <div key={`${item.id}-${index}`} className={`p-3 rounded border flex justify-between items-center ${item.colorClass}`}>
                                    <div className="flex flex-col">
                                        <span className="font-bold text-sm font-mono">{item.title}</span>
                                        <span className="text-xs opacity-80">{item.message}</span>
                                    </div>
                                    <button className="px-2 py-1 bg-black/20 hover:bg-black/40 rounded text-xs uppercase tracking-wider transition-colors">
                                        Locate
                                    </button>
                                </div>
                            ))
                        )}
                    </div>
                </div>

                {/* Footer */}
                <div className="bg-slate-950 p-3 border-t border-slate-700 text-center">
                    <p className="text-[10px] text-slate-500 font-mono">
                        "You can build the fastest engine in the world, but if the Operator's Manual is written in gibberish, the train goes nowhere."
                    </p>
                </div>
            </div>
        </div>
    );
};

export default OwlDiagnostic;
