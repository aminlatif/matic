CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(256) UNIQUE NOT NULL,
    title VARCHAR(256) NOT NULL,
    parent_id UUID REFERENCES categories(id) ON DELETE SET NULL
);
