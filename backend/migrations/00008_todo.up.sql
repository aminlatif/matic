CREATE TYPE todo_status AS ENUM ('pending', 'in_progress', 'done', 'canceled');

CREATE TABLE todo_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(256) UNIQUE NOT NULL,
    title VARCHAR(256) NOT NULL,
    parent_id UUID REFERENCES categories(id) ON DELETE SET NULL
);

CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(256) NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    status todo_status NOT NULL DEFAULT 'pending',
    parent_id UUID REFERENCES todos(id) ON DELETE SET NULL
);

CREATE TABLE todo_todo_groups (
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    todo_group_id UUID REFERENCES todo_groups(id) ON DELETE CASCADE,
    PRIMARY KEY (todo_id, todo_group_id)
);

CREATE TABLE project_todos (
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    sort_order INT NOT NULL DEFAULT 0,
    PRIMARY KEY (project_id, todo_id)
);
