import { memo } from 'react';
import { Handle, Position, type NodeProps } from 'reactflow';
import type { NodeData } from '../types/NarrativeGraph';

const DialogueNode = ({ data, id }: NodeProps<NodeData>) => {
    return (
        <div className="station-node">
            {/* Station Roof / Header */}
            <div className="station-header">
                <div className="station-title-group">
                    <div className="station-meta">
                        <span>🚉 Station</span>
                        <span className="station-id">ID: {id}</span>
                    </div>
                    <span className="station-title">{data.speaker}</span>
                </div>

                {/* Cognitive Load Indicator (Boiler Pressure) */}
                <div className="flex flex-col items-end">
                    <div className={`cognitive-load-indicator ${(data.choices?.length || 0) > 4 ? 'critical' :
                            (data.choices?.length || 0) === 4 ? 'warning' : 'safe'
                        }`}>
                        <span className="text-xs">📦</span>
                        <span className="load-count">{data.choices?.length || 0}/4</span>
                    </div>
                </div>
            </div>

            {/* Content Area */}
            <div className="station-content">
                <div className="line-clamp-3">
                    {data.text || <span className="italic text-slate-600">Empty station...</span>}
                </div>

                {/* Platform Footer */}
                <div className="station-footer">
                    <span>Platform 1</span>
                    <span>Zone 1</span>
                </div>
            </div>

            {/* Input Port */}
            <Handle
                type="target"
                position={Position.Left}
                id="input"
                className="react-flow__handle"
                style={{ left: '-12px' }}
            >
                <div className="port-indicator input" />
            </Handle>

            {/* Output Port */}
            <Handle
                type="source"
                position={Position.Right}
                id="output"
                className="react-flow__handle"
                style={{ right: '-12px' }}
            >
                <div className="port-indicator output" />
            </Handle>
        </div>
    );
};

export default memo(DialogueNode);
