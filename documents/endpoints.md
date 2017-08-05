# Common properties
* All requests should be POST
* All requests can return 401 Unauthorized if the token sent is not valid
* Data posting requests may return 405 Not Allowed if you post more frequently

============================================
## POST /data/register

Register endpoint will return an authentication token, client should save that data. Data collection
endpoints requires that token in Authentication header. Please check https://jwt.io/

Client and server knows a secret key, it should be somehow included in register

Req:
    body: { somedata }
Responses:
    = 200 OK if secret key agreed by the server
        "{ authToken: string }"
    = 401 Unauthorized if server rejects
        ""

============================================
## POST /data/wifi

Handles wifi data posts

Req:
    headers: { Authentication: JWT_TOKEN }
    body: json array [
        {
            channelId: string,
            strength: float
        },
        {
            channelId: string,
            strength: float
        },
        ...
    ]
Responses:
    = 200 OK

============================================
## POST /data/sound

It should be sent in hourly periods, if client sends sound data more than once in an hour, server
will response "405 Not Allowed". Sound amount should be a number in range of: 0.0 - 1.0

Req:
    headers: { Authentication: JWT_TOKEN }
    body: { sound: float }
Responses
    = 200 OK
    = 405 Not allowed if the request made too quick

============================================
## POST /data/light

It should be sent in hourly periods, if client sends light data more than once in an hour, server
will response "405 Not Allowed". Light amount should be a number in range of: 0.0 - 1.0

Req:
    headers: { Authentication: JWT_TOKEN }
    body: { light: float }
Responses:
    = 200 OK
    = 405 Not allowed if the request made too quick

============================================
## POST /data/gps

It should be sent in periods of 3 seconds, if the client sends gps data more than once in 3 seconds
server will response "405 Not Allowed". Latitude and logitude will be kept as 64 bit floating points
on the server for accuracy.

Everytime the client sends gps location, server will send back other users' locations who are in the
same cell. If no other users present at the moment, others field will be an empty array.

If user moved into a new cell, the entities (trees, etc.) located in the new cell  as well as the
cell's id will be added to response in the cell field. Otherwise cell field will be null, that means
user is still in the same cell.

Req:
    headers: { Authentication: JWT_TOKEN }
    body: { latitude: float, longitude: float }
Responses:
    = 200 OK
        {
            others: [{latitude: float, longitude: float}] // array of other users locations,
            cell: null || {
                id: string,
                entities: [
                    { to be decided}
                ]
            }
        }
    = 405 OK Not allowed if the request made too quick
