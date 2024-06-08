-- 適当なユーザーを作成する
CREATE USER 'user'@'%' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON *.* TO 'user'@'%';
FLUSH PRIVILEGES;

-- practiceデータベースを作成する
CREATE DATABASE IF NOT EXISTS practice 
DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
