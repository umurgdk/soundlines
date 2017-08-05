-- Your SQL goes here
create table wifi_readings (
    id         serial primary key,
    user_id    serial,
    created_at timestamptz not null,
    channel_id varchar not null,
    strength   real not null
);
