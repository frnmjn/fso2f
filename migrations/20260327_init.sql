CREATE TABLE products (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL,
    description TEXT NOT NULL,
    kind TEXT NOT NULL DEFAULT 'standard'
);

CREATE TABLE dangerous_products (
    id TEXT PRIMARY KEY REFERENCES products(id) ON DELETE CASCADE,
    max_temperature DOUBLE PRECISION NOT NULL
);

CREATE TABLE expiring_products (
    id TEXT PRIMARY KEY REFERENCES products(id) ON DELETE CASCADE,
    expiration_date TIMESTAMPTZ NOT NULL
);

CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    customer_name TEXT NOT NULL,
    customer_vat TEXT NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_lines (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    line_number INT NOT NULL,
    quantity INT NOT NULL,
    price DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    discount DOUBLE PRECISION,
    UNIQUE(order_id, line_number)
);

CREATE TABLE customers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    vat TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL
);

