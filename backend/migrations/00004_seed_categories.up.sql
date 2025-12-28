INSERT INTO categories (name, title, parent_id)
VALUES
    ('home', 'Home', NULL),
    ('work', 'Work', NULL),
    ('education', 'Education', NULL),
    ('health', 'Health', NULL),
    ('leisure', 'Leisure', NULL);

INSERT INTO categories (name, title, parent_id)
    VALUES
        ('groceries', 'Groceries', (SELECT id FROM categories WHERE name = 'home')),
        ('cooking', 'Cooking',  (SELECT id FROM categories WHERE name = 'home')),
        ('laundry', 'Laundry',  (SELECT id FROM categories WHERE name = 'home')),
        ('cleaning', 'Cleaning',  (SELECT id FROM categories WHERE name = 'home'));

INSERT INTO categories (name, title, parent_id)
    VALUES
        ('commute', 'Commute', (SELECT id FROM categories WHERE name = 'work')),
        ('meetings', 'Meetings',  (SELECT id FROM categories WHERE name = 'work')),
        ('project', 'Project',  (SELECT id FROM categories WHERE name = 'work')),
        ('general', 'General & Maintenance',  (SELECT id FROM categories WHERE name = 'work'));

INSERT INTO categories (name, title, parent_id)
    VALUES
        ('books', 'Books', (SELECT id FROM categories WHERE name = 'education')),
        ('courses', 'Courses',  (SELECT id FROM categories WHERE name = 'education'));

INSERT INTO categories (name, title, parent_id)
    VALUES
        ('breathing', 'Breathing Gymnastics', (SELECT id FROM categories WHERE name = 'health')),
        ('walking', 'Walking', (SELECT id FROM categories WHERE name = 'health')),
        ('excercise', 'Excercise', (SELECT id FROM categories WHERE name = 'health'));

INSERT INTO categories (name, title, parent_id)
    VALUES
        ('movies', 'Movies', (SELECT id FROM categories WHERE name = 'leisure')),
        ('tv', 'TV',  (SELECT id FROM categories WHERE name = 'leisure')),
        ('books_leisure', 'Books',  (SELECT id FROM categories WHERE name = 'leisure')),
        ('traveling', 'Traveling',  (SELECT id FROM categories WHERE name = 'leisure')),
        ('games', 'Games',  (SELECT id FROM categories WHERE name = 'leisure'));
