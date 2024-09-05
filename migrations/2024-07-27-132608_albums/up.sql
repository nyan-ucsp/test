-- Your SQL goes here

create table if not exists albums
(
    id integer primary key autoincrement not null,
    uuid text unique not null,
    title text not null,
    description text not null,
    completed boolean not null default 0,
    covers text not null,
    tags text,
    enable boolean not null default 1,
    min_age int not null default 0,
    url text not null,
    content_type text not null,
    width int not null,
    height int not null,
    bytes int not null,
    released_at text,
    broken_at text,
    created_at text,
    updated_at text
);