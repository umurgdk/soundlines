## Dependencies

* openssl 1.1.0

## Authorization

All requests made from client should include `Authorization` header set with
JWT token sent by the `/users/register` endpoint. To be able to acquire an auth
token, client should make a `POST` request to `/users/register` endpoint with
`Authorization` header set JWT token with payload `{ action: 'register' }`. JWT
token should be encoded with the same secret key with the server. And response 
will include the `user_id` and `token` fields. `token` value returned by 
`/users/register` should be sent with all later requests as untouched.

### WARNING ###

With new JWT system data reading endpoints will ignore `user_id` field sent by
the client. And will overwrite `user_id` as the authenticated user's id.

```
# Registering the device (getting a user id and jwt token)
POST /users/register
Headers:
	Authorization: Bearer JWT token with { action: 'register' } payload
Response:
	{ 
		user_id: integer,
		token: string
	}
```

All other requests should send `Authorization` header
```
# For example /data/sound
POST /data/sound
Headers:
	Authorization Bearer <token sent by /users/register>
Body:
	{ level: 0.11 }
```
