CREATE TABLE honey (
    id Uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title Text NOT NULL
);

CREATE TABLE customer_order (
    id Uuid PRIMARY KEY DEFAULT gen_random_uuid()
);

CREATE TABLE order_item (
    id Uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id Uuid REFERENCES honey(id) NOT NULL,
    qty integer NOT NULL
);
