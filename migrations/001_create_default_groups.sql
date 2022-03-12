INSERT INTO user_groups
                (group_name, permissions) 
        VALUES  ("guest",   "");

INSERT INTO user_groups 
                (group_name, permissions) 
        VALUES  ("admin",   "*");

INSERT INTO user_groups 
                (group_name, permissions) 
        VALUES  ("user",   "canComment");