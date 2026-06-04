
INSERT INTO products (id, code, description, kind) VALUES
    ('a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d', 'WIDGET-001', 'Widget', 'standard'),
    ('b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e', 'CHEESE-002', 'Cheese', 'expiring'),
    ('c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f', 'PISTOL-003', 'Pistol', 'dangerous');

INSERT INTO dangerous_products (id, max_temperature) VALUES
    ('c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f', 75.0);

INSERT INTO expiring_products (id, expiration_date) VALUES
    ('b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e', '2027-06-15T00:00:00Z');

INSERT INTO orders (id, customer_id, customer_name, customer_vat, total_amount, status) VALUES
    ('aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa', '11111111-1111-4111-8111-111111111111', 'Acme Srl (From Order)', 'IT01234567890', 150.00, 'Confirmed'),
    ('bbbb2222-bb22-4bb2-8bb2-bbbbbbbbbbbb', '11111111-1111-4111-8111-111111111111', 'Acme Srl (From Order)', 'IT01234567890', 42.50, 'Draft'),
    ('cccc3333-cc33-4cc3-8cc3-cccccccccccc', '22222222-2222-4222-8222-222222222222', 'Beta SpA (From Order)', 'IT09876543210', 320.75, 'Confirmed'),
    ('dddd4444-dd44-4dd4-8dd4-dddddddddddd', '33333333-3333-4333-8333-333333333333', 'Gamma Snc (From Order)', 'IT11223344556', 89.90, 'Deleted');

INSERT INTO order_lines (id, order_id, product_id, line_number, quantity, price, discount) VALUES
    ('10000000-0000-4000-8000-000000000001', 'aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d', 1, 5, 12.50, 2.00),
    ('10000000-0000-4000-8000-000000000002', 'aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa', 'c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f', 2, 10, 7.50, NULL),
    ('10000000-0000-4000-8000-000000000003', 'bbbb2222-bb22-4bb2-8bb2-bbbbbbbbbbbb', 'b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e', 1, 1, 42.50, 5.00),
    ('10000000-0000-4000-8000-000000000004', 'cccc3333-cc33-4cc3-8cc3-cccccccccccc', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d', 1, 3, 12.50, NULL),
    ('10000000-0000-4000-8000-000000000005', 'cccc3333-cc33-4cc3-8cc3-cccccccccccc', 'b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e', 2, 7, 35.00, 10.00),
    ('10000000-0000-4000-8000-000000000006', 'dddd4444-dd44-4dd4-8dd4-dddddddddddd', 'c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f', 1, 2, 44.95, NULL);


INSERT INTO customers (id, name, vat, email, phone) VALUES
    ('11111111-1111-4111-8111-111111111111', 'Acme Srl (From Customers)', 'IT01234567890', 'info@acme.it', '+39 02 1234567'),
    ('22222222-2222-4222-8222-222222222222', 'Beta SpA (From Customers)', 'IT09876543210', 'contatti@beta.it', '+39 06 7654321'),
    ('33333333-3333-4333-8333-333333333333', 'Gamma Snc (From Customers)', 'IT11223344556', 'admin@gamma.it', '+39 055 1122334');
