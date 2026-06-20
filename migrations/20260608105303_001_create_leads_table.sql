CREATE TABLE leads
(
    id             UUID PRIMARY KEY,
    name           VARCHAR(100),
    contact_method VARCHAR(20)  NOT NULL,
    contact_value  VARCHAR(255) NOT NULL,
    consent_given  BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);