-- Add migration script here
CREATE TABLE post_votes (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
    vote_type SMALLINT NOT NULL,
    PRIMARY KEY (user_id, post_id)
);

CREATE TABLE comment_votes (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    vote_type SMALLINT NOT NULL,
    PRIMARY KEY (user_id, comment_id)
);