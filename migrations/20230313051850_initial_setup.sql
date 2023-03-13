-- Add migration script here
CREATE TABLE Users(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL,
    avatar_id INT NOT NULL
);

CREATE TABLE Rooms (
    id uuid PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    room_capacity INT NOT NULL,
    created_at timestamptz NOT NULL
);
CREATE TABLE AvailableRooms (
    id uuid PRIMARY KEY,
    room_id uuid NOT NULL,
    number_of_players INT NOT NULL,
    is_open BOOLEAN NOT NULL,
    FOREIGN KEY (room_id) REFERENCES Rooms(id)
);

CREATE TABLE Connections (
    user_id uuid NOT NULL,
    room_id uuid NOT NULL,
    is_admin BOOLEAN NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id),
    FOREIGN KEY (room_id) REFERENCES Rooms(id),
    PRIMARY KEY (user_id, room_id)
);

CREATE OR REPLACE FUNCTION update_delete_room_players()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE AvailableRooms
    SET number_of_players = (SELECT COUNT(*) FROM Connections WHERE room_id = OLD.room_id)
    WHERE room_id = OLD.room_id;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_delete_room_players
AFTER DELETE ON Connections
FOR EACH ROW
EXECUTE FUNCTION update_delete_room_players();

CREATE OR REPLACE FUNCTION update_insertion_room_players()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE AvailableRooms
    SET number_of_players = (SELECT COUNT(*) FROM Connections WHERE room_id = NEW.room_id)
    WHERE room_id = NEW.room_id;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_insertion_room_players
AFTER INSERT ON Connections
FOR EACH ROW
EXECUTE FUNCTION update_insertion_room_players();




