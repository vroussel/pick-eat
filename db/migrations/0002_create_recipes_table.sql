CREATE TABLE recipes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    prep_time INT NOT NULL,
    cook_time INT NOT NULL,
    image_stem UUID,
    image_extension TEXT,

    CHECK ((image_stem IS NULL) = (image_extension IS NULL))
)
