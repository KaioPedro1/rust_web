CREATE TABLE Rooms (
    id uuid PRIMARY KEY,
    name VARCHAR(100) NOT NULL
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