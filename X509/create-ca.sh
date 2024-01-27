## Create a private key for CA
# openssl genpkey -out ca-key -algorithm Ed25519 #Ed25519 key
## Create a self-signed certificate for CA
# openssl req -new -x509 -nodes -days 3650 -key ca-key -out ca-cert.crt -subj "/O=Crasite, Inc./CN=Crasite Root CA"

## Create a key for server
openssl genrsa -out server-key 2048
openssl req -new -key server-key -out server.csr -subj "/O=Crasite, Inc./CN=crasite.com"
## use this to create a key with request
#openssl req -newkey rsa:2048 -nodes -days 365  -keyout server-key.pem  -out server-req.pem -subj "/O=Crasite, Inc./CN=crasite.com"
## Signing Certificate
openssl x509 -req -days 365 \
	-in server.csr \
	-out server.crt \
	-CA ca-cert.crt \
	-CAkey ca-key \
	-extensions SAN \
	-extfile extraInfo.txt
