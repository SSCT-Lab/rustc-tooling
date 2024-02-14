-- Your SQL goes here
CREATE TABLE dependencies (
    id INT AUTO_INCREMENT PRIMARY KEY,
    lhs_id INT NOT NULL,
    rhs_id INT NOT NULL,
    FOREIGN KEY (lhs_id) REFERENCES loc_info(id),
    FOREIGN KEY (rhs_id) REFERENCES loc_info(id)
);
