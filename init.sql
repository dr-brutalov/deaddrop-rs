CREATE TABLE Users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user TINYTEXT NOT NULL,
    hash TEXT NOT NULL
);

CREATE TABLE Messages (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sender INTEGER NOT NULL REFERENCES Users(id),
    recipient INTEGER NOT NULL REFERENCES Users(id),
    hashed_message TEXT NOT NULL,
    data TEXT NOT NULL
);

CREATE TRIGGER sender
BEFORE UPDATE OF sender ON Messages
BEGIN
    SELECT raise(abort, "Attempted to alter the sender of a message.");
END;

CREATE TRIGGER msg_change
BEFORE UPDATE OF hashed_message ON Messages
BEGIN
    SELECT raise(abort, "Attempted to alter the hash of a message.");
END