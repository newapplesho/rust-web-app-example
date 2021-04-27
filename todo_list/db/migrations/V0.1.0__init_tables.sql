
/* Drop Tables */

DROP TABLE IF EXISTS Todo;
DROP TABLE IF EXISTS Todo_list;




/* Create Tables */

CREATE TABLE Todo
(
    -- todo ID
    todo_id uuid DEFAULT UUID_GENERATE_V4() NOT NULL UNIQUE,
    -- TODO list Id
    todo_list_id uuid DEFAULT UUID_GENERATE_V4() NOT NULL,
    -- TODO名
    name varchar NOT NULL,
    -- 説明
    description text,
    -- 作成日時
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    -- 更新日時
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (todo_id)
) WITHOUT OIDS;


CREATE TABLE Todo_list
(
    -- TODO list Id
    todo_list_id uuid DEFAULT UUID_GENERATE_V4() NOT NULL UNIQUE,
    name varchar NOT NULL,
    -- 作成日時
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    -- 更新日時
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (todo_list_id)
) WITHOUT OIDS;



/* Create Foreign Keys */

ALTER TABLE Todo
    ADD FOREIGN KEY (todo_list_id)
        REFERENCES Todo_list (todo_list_id)
        ON UPDATE RESTRICT
        ON DELETE RESTRICT
;



/* Comments */

COMMENT ON COLUMN Todo.todo_id IS 'todo ID';
COMMENT ON COLUMN Todo.todo_list_id IS 'TODO list Id';
COMMENT ON COLUMN Todo.name IS 'TODO名';
COMMENT ON COLUMN Todo.description IS '説明';
COMMENT ON COLUMN Todo.created_at IS '作成日時';
COMMENT ON COLUMN Todo.updated_at IS '更新日時';
COMMENT ON COLUMN Todo_list.todo_list_id IS 'TODO list Id';
COMMENT ON COLUMN Todo_list.created_at IS '作成日時';
COMMENT ON COLUMN Todo_list.updated_at IS '更新日時';
