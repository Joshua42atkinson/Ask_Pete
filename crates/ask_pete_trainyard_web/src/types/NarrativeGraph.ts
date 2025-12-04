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
    literary_device?: string;
    quest?: Quest; // [NEW] Syncs Node with Quest System
    position?: NodePosition;
}

export interface Quest {
    title: string;
    chapter_theme: string;
    description: string;
    starting_step: string;
    completion_reward: QuestReward;
    steps: Record<string, QuestStep>;
}

export interface QuestReward {
    type: string;
    value?: number;
    details?: string;
    name?: string;
}

export interface QuestStep {
    description: string;
    choices?: NarrativeChoice[];
    trigger_condition: string;
    next_step?: string;
    step_reward?: QuestReward;
    is_major_plot_point?: boolean;
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

export type NodeData = Omit<NarrativeNode, 'id' | 'position'>;

// Backend Types (matching ask_pete_core::expert::StoryGraph)
export interface StoryGraph {
    id: string;
    title: string;
    nodes: BackendStoryNode[];
    connections: StoryConnection[];
}

export interface BackendStoryNode {
    id: string;
    title: string;
    content: string;
    x: number;
    y: number;
    passenger_count: number;
    complexity_level: number;
    learner_profiles: string[];
    gardens_active: string[];
    required_stats: Record<string, number>;
    logic: any; // LogicBlock
    style: any; // NodeStyle
    quest?: Quest;
}

export interface StoryConnection {
    id: string;
    from_node: string;
    to_node: string;
}

export interface BlueprintResponse {
    graph: StoryGraph;
    reasoning: string;
}
