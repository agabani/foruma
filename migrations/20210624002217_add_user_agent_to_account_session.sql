-- Add migration script here
alter table account_session
    add user_agent text;
