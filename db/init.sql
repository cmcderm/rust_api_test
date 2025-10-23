
create extension if not exists "uuid-ossp";

create table Users {
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
};

insert into users (
    username,
    email,
    admin
)
VALUES
('cmcderm', 'connormcderm@protonmail.com', true),
('toffee', 'boof@woof.com', false)
('mocha', 'bork@woof.com', false);

