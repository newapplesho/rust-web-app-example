DROP TABLE IF EXISTS Todo_Status;

CREATE TABLE Todo_Status
(
    status_id int DEFAULT 1 NOT NULL UNIQUE,
    name varchar NOT NULL UNIQUE,
    -- 作成日時
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    -- 更新日時
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (status_id)
) WITHOUT OIDS;

alter table todo
    add status_id int default 1 not null;

ALTER TABLE Todo
    ADD FOREIGN KEY (status_id)
        REFERENCES Todo_Status (status_id)
        ON UPDATE RESTRICT
        ON DELETE RESTRICT
;

INSERT INTO public.todo_status (status_id, name) VALUES (1, 'Todo');
INSERT INTO public.todo_status (status_id, name) VALUES (2, 'Done');
