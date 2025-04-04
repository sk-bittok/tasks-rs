-- Add up migration script here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    user_pid UUID REFERENCES users (pid), 
    pid UUID NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(256) NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
