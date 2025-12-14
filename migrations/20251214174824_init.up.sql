CREATE TABLE honey (
    id Uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title Text NOT NULL
)
