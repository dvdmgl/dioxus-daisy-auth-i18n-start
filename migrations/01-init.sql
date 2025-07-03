CREATE EXTENSION "uuid-ossp";
CREATE TYPE user_role AS ENUM('admin', 'staff', 'user', 'guest', 'naughty');

CREATE TYPE app_user_permission AS ENUM(
    'deleteuser',
    'markasnaughty',
    'prodemoteuser',
    'edituserpermissions',
    'read'
);

CREATE TABLE IF NOT EXISTS app_user (
    id BIGSERIAL PRIMARY KEY,
    c_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    m_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    skey uuid UNIQUE NOT NULL DEFAULT uuid_generate_uuid()
);

CREATE TABLE IF NOT EXISTS app_groups_permissions (
    role user_role NOT NULL,
    permission app_user_permission NOT NULL,
    PRIMARY KEY (role, permission)
);

INSERT INTO
    app_groups_permissions (role, permission)
VALUES
    ('guest', 'read'),
    ('user', 'read'),
    ('staff', 'read'),
    ('staff', 'markasnaughty'),
    ('admin', 'deleteuser'),
    ('admin', 'markasnaughty'),
    ('admin', 'prodemoteuser'),
    ('admin', 'edituserpermissions'),
    ('admin', 'read');
