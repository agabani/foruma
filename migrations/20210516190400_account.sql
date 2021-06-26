create table account
(
    id        bigserial    not null,
    public_id varchar(36)  not null,
    created   timestamptz  not null,
    username  varchar(320) not null,
    constraint account_pk
        primary key (id)
);

create unique index account_public_id_uindex
    on account (public_id);

create unique index account_username_uindex
    on account (username);

create table account_authentication_password
(
    id            bigserial   not null,
    public_id     varchar(36) not null,
    created       timestamptz not null,
    account_id    bigserial   not null,
    password_hash text        not null,
    constraint account_authentication_password_pk
        primary key (id),
    constraint account_authentication_password_account_id_fk
        foreign key (account_id) references account
            on delete cascade
);

create unique index account_authentication_password_public_id_uindex
    on account_authentication_password (public_id);

create table account_authentication_session
(
    id          bigserial   not null,
    public_id   varchar(36) not null,
    created     timestamptz not null,
    account_id  bigserial   not null,
    last_active timestamptz not null,
    ip_address  inet,
    user_agent  text,
    constraint account_authentication_session_pk
        primary key (id),
    constraint account_authentication_session_account_id_fk
        foreign key (account_id) references account
            on delete cascade
);

create unique index account_authentication_session_public_id_uindex
    on account_authentication_session (public_id);

