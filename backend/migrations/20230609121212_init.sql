CREATE TABLE IF NOT EXISTS issues (
    Id SERIAL PRIMARY KEY,
    DiscordThreadId VARCHAR UNIQUE,
    DiscordThreadLink VARCHAR UNIQUE,
    Origin VARCHAR NOT NULL DEFAULT 'discord',
    SevCat SMALLINT NOT NULL DEFAULT 5 CHECK (SevCat >= 1 AND SevCat <= 5),
    OriginalPoster VARCHAR,
    InitialMessage VARCHAR,
    FirstResponseUser VARCHAR,
    ResolverUser VARCHAR,
    GithubLink VARCHAR UNIQUE,
    Locked BOOLEAN NOT NULL DEFAULT false,
    LockStatusChangeReason VARCHAR,
    Resolved BOOLEAN NOT NULL DEFAULT false,
    Categories JSONB,
    MessageCount INTEGER,
    UserCount INTEGER,
    Created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    LastUpdated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FirstResponseTimedate TIMESTAMP WITH TIME ZONE,
    ResolvedTimedate TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS feedback (
    Id SERIAL PRIMARY KEY,
    DiscordThreadId VARCHAR NOT NULL UNIQUE,
    Upvotes INT,
    DownVotes INT
);