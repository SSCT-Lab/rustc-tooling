-- Your SQL goes here
CREATE TABLE loc_info (
    id INT AUTO_INCREMENT PRIMARY KEY,
    ident VARCHAR(255) NOT NULL,
    line_num INT NOT NULL,
    col_num INT NOT NULL,
    file_path VARCHAR(255) NOT NULL
);
