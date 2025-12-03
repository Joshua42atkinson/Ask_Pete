export interface NarrativeGraph {
    nodes: Record<string, NarrativeNode>;
    start_node_id: string;
    metadata?: Record<string, string>;
}

export interface NarrativeNode {
    id: string;
    speaker: string;
    text: string;
    choices: NarrativeChoice[];
    events: NarrativeEvent[];
    position?: NodePosition;
}

export interface NarrativeChoice {
    text: string;
    next_node_id?: string;
    conditions: NarrativeCondition[];
}

export interface NarrativeEvent {
    event_type: string;
    payload: Record<string, string>;
}

export interface NarrativeCondition {
    condition_type: string;
    parameters: Record<string, string>;
}

export interface NodePosition {
    x: number;
    y: number;
}
