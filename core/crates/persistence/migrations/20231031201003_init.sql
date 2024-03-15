CREATE TABLE IF NOT EXISTS triggers (
    trigger_id TEXT NOT NULL PRIMARY KEY,
    -- /file/created/<file-path> or /whatsapp/message/<message-id>
    event_name TEXT NOT NULL,
    payload json NOT NULL,
    metadata json,
    timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
);
CREATE TABLE IF NOT EXISTS events (
    event_id TEXT NOT NULL PRIMARY KEY,
    flow_id TEXT NOT NULL, -- the flow that was running UUID ( root flow name and stuff)
    flow_version_id TEXT NOT NULL, -- the version of the flow that was running UUID
    flow_version_name TEXT, -- the name of the flow version that was running example 0.0.1
    trigger_id TEXT NOT NULL, -- the trigger that caused the event
    trigger_session_id TEXT NOT NULL, -- anything that is triggered by a single trigger including nested flow runs
    flow_session_id TEXT NOT NULL, -- a single instance of a flow running
    name TEXT, --UNSURE WHY WE HAVE THIS. TODO: remove?
    context json, -- the bundle of args used for the action to process
    created_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP), --stats for action run time
    started_at timestamp with time zone, --stats for action run time
    ended_at timestamp with time zone, --stats for action run time
    debug_result json, -- debug info, a place where we can store extra data if we want like intermediate steps in the flow
    result json -- the result of the action
);
CREATE TABLE IF NOT EXISTS flows (
    flow_id TEXT PRIMARY KEY NOT NULL,
    flow_name TEXT NOT NULL,
    latest_version_id TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    UNIQUE (flow_name)
);
CREATE TABLE IF NOT EXISTS flow_versions (
    flow_id TEXT NOT NULL,
    flow_version_id TEXT NOT NULL,
    flow_version TEXT NOT NULL,
    description TEXT,
    checksum TEXT,
    updated_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    published BOOLEAN NOT NULL DEFAULT FALSE,
    flow_definition json NOT NULL,
    UNIQUE (flow_id, flow_version)
);
-- CREATE TABLE IF NOT EXISTS nodes (
--     node_id TEXT PRIMARY KEY NOT NULL,
--     flow_id TEXT NOT NULL,
--     node_type TEXT NOT NULL,
--     node_name TEXT NOT NULL,
--     node_description TEXT NOT NULL,
--     node_config json NOT NULL,
--     node_definition json NOT NULL,
--     UNIQUE (flow_id, node_name) };
-- );