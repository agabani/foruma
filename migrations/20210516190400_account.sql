create table account
(
    id        bigserial    not null,
    public_id varchar(36)  not null,
    created   timestamptz  not null,
    deleted   timestamptz,
    username  varchar(320) not null,
    constraint account_pk
        primary key (id)
);

create unique index account_public_id_uindex
    on account (public_id);

create unique index account_username_uindex
    on account (username);

create table account_password
(
    id            bigserial   not null,
    public_id     varchar(36) not null,
    created       timestamptz not null,
    deleted       timestamptz,
    account_id    bigserial   not null,
    password_hash text        not null,
    constraint account_password_pk
        primary key (id),
    constraint account_password_account_id_fk
        foreign key (account_id) references account
            on delete cascade
);

create unique index account_password_public_id_uindex
    on account_password (public_id);

create table account_session
(
    id         bigserial   not null,
    public_id  varchar(36) not null,
    created    timestamptz not null,
    deleted    timestamptz,
    account_id bigserial   not null,
    constraint account_session_pk
        primary key (id),
    constraint account_session_account_id_fk
        foreign key (account_id) references account
            on delete cascade
);

create unique index account_session_public_id_uindex
    on account_session (public_id);

