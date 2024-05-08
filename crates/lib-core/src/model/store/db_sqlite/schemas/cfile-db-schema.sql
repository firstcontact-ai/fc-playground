CREATE TABLE IF NOT EXISTS conv_ref (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,

  conv_uid   TEXT, 

  -- timestamps
  ctime       INTEGER,
  mtime       INTEGER  
) STRICT;
CREATE INDEX IF NOT EXISTS idx_conv_ref_conv_uid ON conv_ref(conv_uid);

-- Message
CREATE TABLE IF NOT EXISTS msg (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  uid           TEXT, 

  conv_ref_id        INTEGER NOT NULL,

  orig_msg_id        INTEGER,       -- The eventual origin message of this msg (when was result of step task)

  -- TBD: prev_msg_id (to maintain the message chain)
  
  author_kind        TEXT NOT NULL, -- "User" | "Agent"
  author_agent_uid   TEXT,          -- agent uid when author_kind is Agent

  content            TEXT,
  c_type             TEXT, -- "Text" | "Json"

  start_time         INTEGER, 
  done_time          INTEGER, -- if null, not completed yet.

  err                TEXT,
 
  -- timestamps
  ctime              INTEGER,
  mtime              INTEGER,

  FOREIGN KEY (conv_ref_id) REFERENCES conv_ref(id) ON DELETE CASCADE
) STRICT;


CREATE TABLE IF NOT EXISTS stack_step (
  id             INTEGER PRIMARY KEY AUTOINCREMENT,
  uid            TEXT, 
 
  orig_msg_id    INTEGER NOT NULL,           -- The message that initiated this step and others of the same chains
  first_step_id  INTEGER,                    -- The initial step (will point to self.id if it is the first step)
  prev_step_id   INTEGER,                    -- The previous step (if not first step)
  closer         INTEGER NOT NULL DEFAULT 0, -- Is the closing step (empty stack, no output)
  
  resolve_tstart INTEGER, -- When it has been takend to be resolved
  resolve_tend   INTEGER, -- When the resolve was completed
  resolve_model  TEXT,    -- The model of the resolved agent of the stack (useful to determine if can run parallel)

  run_agent_uid  TEXT,    -- The agent that it was run with (should match the call stack, just for record)
  run_agent_name TEXT,    -- #cache# the name of the agent (to avoid join)

  run_tstart     INTEGER, -- When the run started
  run_tend       INTEGER, -- When the run completed
  run_terr       INTEGER, -- If there is an error

  -- Call Ctx
  call_stack    TEXT,   -- JSON for the chain call stack for this step (e.g., items [{cursor, agent_uuid}, ...] )
  call_out      TEXT,   -- Then, after processing, the call_out is the response
  call_err      TEXT,   -- If tehre is an error

  -- timestamps
  ctime         INTEGER,
  mtime         INTEGER,

  FOREIGN KEY (orig_msg_id) REFERENCES msg(id) ON DELETE CASCADE
) STRICT;
