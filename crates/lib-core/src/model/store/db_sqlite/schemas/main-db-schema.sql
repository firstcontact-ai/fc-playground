-- Agent
CREATE TABLE IF NOT EXISTS agent (
  id               INTEGER PRIMARY KEY AUTOINCREMENT,
  uid              TEXT NOT NULL, 

  kind             TEXT DEFAULT 'Ai',               -- Ai, Logic

  space_default    INTEGER NOT NULL DEFAULT false,  -- Bool to say if it is the default for new space
  
  name             TEXT NOT NULL,
  "desc"           TEXT,

  -- Ai Props
  provider         TEXT, 
  model            TEXT,
  inst             TEXT,
  prompt_tmpl      TEXT,
  chain            TEXT, -- json, might become blob for jsonb
  out_format       TEXT, -- "Text" | "Json"

  -- Logic Props
  logic_tool       TEXT, -- e.g. "list_files"

  -- timestamps (unix_utc_us)
  ctime            INTEGER,
  mtime            INTEGER
) STRICT;

CREATE UNIQUE INDEX idx_agent_kind_name ON agent(kind, name);


-- Drive
CREATE TABLE IF NOT EXISTS drive (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  uid       TEXT NOT NULL, 

  name      TEXT NOT NULL,

  -- timestamps (unix_utc_us)
  ctime     INTEGER,
  mtime     INTEGER
) STRICT;


-- Space
CREATE TABLE IF NOT EXISTS space (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  uid       TEXT NOT NULL, 

  -- Fks
  agent_id  INTEGER,

  -- Props
  name      TEXT,

  -- Commons
  last_open INTEGER, -- unix_utc_us

  -- timestamps (unix_utc_us) 
  ctime     INTEGER,
  mtime     INTEGER
) STRICT;
CREATE INDEX idx_space_last_open ON space(last_open);


-- Conv
CREATE TABLE IF NOT EXISTS conv (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  uid       TEXT NOT NULL, 
  
  -- Fks
  space_id  INTEGER NOT NULL,

  -- cfile_id created on demand (one-to-many)
  cfile_id    INTEGER,

  -- cfile_db conv_ref.id maching this conv ("cache" to avoid uid lookup)
  conv_ref_id INTEGER, 

  work_tnew   INTEGER, -- When some new work was identified
  work_tdone  INTEGER, -- When all of the work was last completed (if tnew > tdone, then, more work is needed)

  -- Props
  title       TEXT,

  -- Commons
  last_open   INTEGER, -- unix_utc_us

  -- timestamps (unix_utc_us)
  ctime       INTEGER,
  mtime       INTEGER
) STRICT;


-- CFile - 
-- - Conv File that will have the conversation content (i.e. messages)
-- - Might contain multi conversations messages. 
CREATE TABLE IF NOT EXISTS cfile (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  uid         TEXT,

  -- timestamps (unix_utc_us)
  ctime     INTEGER,
  mtime     INTEGER 
) STRICT;


-- SpaceDrive
CREATE TABLE IF NOT EXISTS space_drive (
  id             INTEGER PRIMARY KEY AUTOINCREMENT,

  space_id       INTEGER, 
  drive_id       INTEGER,
  space_default  INTEGER,

  -- timestamps (unix_utc_us)
  ctime     INTEGER,
  mtime     INTEGER
) STRICT;


-- Data Source
--  typ: `File` | `Folder` | `GhRepo` | `GgDoc`
CREATE TABLE IF NOT EXISTS dsource (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  uid       TEXT NOT NULL, 
  kind      TEXT NOT NULL,

  drive_id  INTEGER NOT NULL, -- TODO: many-to-many, move to dsource_drive table
  
  name      TEXT NOT NULL, -- e.g., file_name (or dir name)
  rref      TEXT NOT NULL, -- e.g., The full path
  detail    TEXT,          -- json (will be jsonb)

  -- timestamps (unix_utc_us)
  ctime     INTEGER,
  mtime     INTEGER
) STRICT;
CREATE UNIQUE INDEX idx_dsource_drive_rref ON dsource (drive_id, rref);


-- Data Source Item (like sub file sub folders)
--  kind: Md, Pdf.
CREATE TABLE IF NOT EXISTS ditem (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  uid               TEXT, 
  kind              TEXT NOT NULL,
  
  -- File props
  folder_path       TEXT NOT NULL,

  file_path         TEXT NOT NULL,
  file_mtime        INTEGER,
  file_size         INTEGER,
  file_ext          TEXT,

  -- DFile props
  proc_time         INTEGER, -- When ditem was profile. Nothing to do !NULL or > file_mtime
  dfile_id          INTEGER,

  -- timestamps (unix_utc_us)
  ctime             INTEGER,
  mtime             INTEGER 
) STRICT;

CREATE UNIQUE INDEX idx_item_file_path ON ditem (file_path);


-- Data Source Item (like sub file sub folders)
--  kind: Md, Pdf.
CREATE TABLE IF NOT EXISTS ditem_dsource (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,

  ditem_id          INTEGER NOT NULL,
  dsource_id        INTEGER NOT NULL,

  -- timestamps (unix_utc_us)
  ctime             INTEGER,
  mtime             INTEGER, 

  FOREIGN KEY (ditem_id)   REFERENCES ditem(id)   ON DELETE CASCADE, 
  FOREIGN KEY (dsource_id) REFERENCES dsource(id) ON DELETE CASCADE
) STRICT;

CREATE UNIQUE INDEX idx_ditem_dsource_pks ON ditem_dsource (dsource_id, ditem_id);

-- Data File that will have more or file content
CREATE TABLE IF NOT EXISTS dfile (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  uid         TEXT,


  -- Potential relation
  main_dsource_id INTEGER, -- main dfile for the integer
  
  -- timestamps (unix_utc_us)
  ctime     INTEGER,
  mtime     INTEGER 
) STRICT;