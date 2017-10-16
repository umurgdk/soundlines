create table weather(
    id serial primary key not null,
    temperature double precision not null,
    precip varchar(255)
);