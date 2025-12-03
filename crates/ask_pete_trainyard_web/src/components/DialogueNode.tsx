import { memo } from 'react';
import { Handle, Position, type NodeProps } from 'reactflow';
import type { NarrativeNode } from '../types/NarrativeGraph';

// We need to extend the data prop to match our NarrativeNode structure
// minus the things that React Flow handles (id, position)
type DialogueNodeData = Omit<NarrativeNode, 'id' | 'position'>;

const DialogueNode = ({ data, isConnectable }: NodeProps<DialogueNodeData>) => {
    return (
        <div className="dialogue-node-container">
            <Handle type="target" position={Position.Top} isConnectable={isConnectable} />

            <div className="dialogue-header">
                {data.speaker || 'Unknown Speaker'}
            </div>
            <div className="dialogue-text">
                {data.text || '...'}
            </div>

            {data.choices && data.choices.length > 0 ? (
                <div className="choices">
                    {data.choices.map((choice, index) => (
                        <div key={index} className="choice-item">
                            {choice.text}
                            <Handle
                                type="source"
                                position={Position.Right}
                                id={`choice-${index}`}
                                className="choice-handle"
                                isConnectable={isConnectable}
                            />
                        </div>
                    ))}
                </div>
            ) : (
                <Handle type="source" position={Position.Bottom} isConnectable={isConnectable} />
            )}
        </div>
    );
};

export default memo(DialogueNode);
