CREATE DATABASE `sandwich-recipes`;
USE `sandwich-recipes`;

CREATE TABLE sandwich (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(256) NOT NULL UNIQUE,
    ingredients VARCHAR(1024) NOT NULL,
    stars INT
);