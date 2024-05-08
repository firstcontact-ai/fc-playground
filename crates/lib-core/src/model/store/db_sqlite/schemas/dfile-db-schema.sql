-- NOT USED FOR NOW - File the file that the part is from
CREATE TABLE IF NOT EXISTS ditem_ref (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,

  ditem_uid   TEXT, 

  -- timestamps
  ctime       INTEGER,
  mtime       INTEGER  
) STRICT;

CREATE INDEX IF NOT EXISTS idx_ditem_ref_ditem_uid ON ditem_ref(ditem_uid);

-- File the file that the part is from
-- Later
--   odr        INTEGER,
--   parent_id  INTEGER,
CREATE TABLE IF NOT EXISTS part (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  uid          TEXT, 
  
  ditem_ref_id INTEGER NOT NULL, 

  is_title     INTEGER NOT NULL DEFAULT 0, 
  level        INTEGER NOT NULL DEFAULT 0,
  "group"      INTEGER NOT NULL DEFAULT 0,
  line_num     INTEGER NOT NULL,
  content      TEXT,
 
  -- timestamps
  ctime     INTEGER,
  mtime     INTEGER,

  FOREIGN KEY (ditem_ref_id) REFERENCES ditem_ref(id) ON DELETE CASCADE
) STRICT;


-- `content='part'` option in fts5 means the content is from the part table (safe sapce)
-- Note: here the `fts5` option name `content` is the same as the column name, but just coincidence. 
CREATE VIRTUAL TABLE IF NOT EXISTS part_fts USING fts5(content, content='part');

-- Trigger to insert data into FTS table when inserting into the main table
CREATE TRIGGER part_ai AFTER INSERT ON part BEGIN
  INSERT INTO part_fts(rowid, content) VALUES (new.id, new.content);
END;

-- -- Trigger to update data in FTS table when updating the main table
CREATE TRIGGER part_au AFTER UPDATE ON part BEGIN
  INSERT INTO part_fts(part_fts, rowid, content, uid) VALUES('delete', old.id, old.content);
  INSERT INTO part_fts(rowid, content, uid) VALUES (new.id, new.content);
END;

-- Trigger to delete data from FTS table when deleting from the main table
CREATE TRIGGER part_ad AFTER DELETE ON part BEGIN
  INSERT INTO part_fts(part_fts, rowid, content, uid) VALUES('delete', old.id, old.content);
END;

