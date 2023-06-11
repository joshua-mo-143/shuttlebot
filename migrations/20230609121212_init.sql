CREATE TABLE IF NOT EXISTS issues (
    Id SERIAL PRIMARY KEY,
    DiscordThreadId VARCHAR NOT NULL,
    Origin VARCHAR NOT NULL DEFAULT 'discord',
    SevCat CHAR NOT NULL CHECK (SevCat >= 1 AND SevCat <= 5),
    InitialMessage VARCHAR,
    FirstResponseUser VARCHAR,
    ResolverUserId VARCHAR CHECK (Resolved = true),
    GithubLink VARCHAR,
    Locked BOOLEAN NOT NULL DEFAULT false,
    LockStatusChangeReason VARCHAR CHECK (Locked = true),
    Resolved BOOLEAN NOT NULL DEFAULT false,
    Created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    LastUpdated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FirstResponseTimedate TIMESTAMP WITH TIMEZONE,
    ResolvedTimedate TIMESTAMP WITH TIME ZONE CHECK (resolved = true)
);