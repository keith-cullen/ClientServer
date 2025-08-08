#!/bin/bash

# remove a file if it exists
function rmcond {
    if [ -e $1 ]; then
	/bin/rm $1
    fi
}

rmcond ca.key
rmcond ca.crt
rmcond client.csr
rmcond client.crt
rmcond client.key
rmcond server.csr
rmcond server.crt
rmcond server.key

# generate the CA
# the -x509 option outputs a certificate instead of a certificate request
# the -config option specifies the configuration file
# the -extensions option specifies the section in the configuration file from which X.509 extensions are to be included in the generated certificate
# the -set_serial option specifies the serial number to assign to the generated certificate
# the -days option specifies the number of days from today to certify the generated certificate for
# the -out option specifies the file to write the generated certificate to
# the -newkey option generates a new private key
# the -nodes option specifies that the generated private key should not be encrypted
# the -keyout option specifies the file to write the generated key to
openssl req \
        -x509 \
        -config openssl.cnf \
        -extensions req_ca \
        -set_serial 01 \
        -days 3650 \
        -out ca.crt \
        -newkey rsa:4096 \
        -nodes \
        -keyout ca.key

# generate the client certificate request
# the absence of the -x509 option outputs a certificate request instead of a certificate
# the -config option specifies the configuration file
# the -newkey option generates a new private key
# the -nodes option specifies that the generated private key should not be encrypted
# the -keyout option specifies the file to write the generated key to
# the -out option specifies the file to write the generated certificate request to
openssl req \
        -config openssl.cnf \
        -newkey rsa:4096 \
        -nodes \
        -keyout client.key \
        -out client.csr \
        -sha256

# generate the server certificate request
# the absence of the -x509 option outputs a certificate request instead of a certificate
# the -config option specifies the configuration file
# the -newkey option generates a new private key
# the -nodes option specifies that the generated private key should not be encrypted
# the -keyout option specifies the file to write the generated key to
# the -out option specifies the file to write the generated certificate request to
openssl req \
        -config openssl.cnf \
        -newkey rsa:4096 \
        -nodes \
        -keyout server.key \
        -out server.csr \
        -sha256

# sign the client certificate request with the CA
# the -req option specifies that a certificate request is expected on input
# the -extfile option specifies the extensions file
# the -extensions option specifies the section in the extensions file from which X.509 extensions are to be included in the generated certificate
# the -set_serial option specifies the serial number to assign to the generated certificate
# the -days option specifies the number of days from today to certify the generated certificate for
# the -CA option specifies the certificate to use to sign the generated certificate
# the -CAkey option specifies the private key to use to sign the generated certificate
# the -in option specifies the input certificate request
# the -out option specifies the file to write the generated certificate to
openssl x509 \
        -req \
        -extfile openssl.cnf \
        -extensions req_client \
        -set_serial 01 \
        -sha256 \
        -days 3650 \
        -CA ca.crt \
        -CAkey ca.key \
        -in client.csr \
        -out client.crt

# sign the server certificate request with the CA
# the -req option specifies that a certificate request is expected on input
# the -extfile option specifies the extensions file
# the -extensions option specifies the section in the extensions file from which X.509 extensions are to be included in the generated certificate
# the -set_serial option specifies the serial number to assign to the generated certificate
# the -days option specifies the number of days from today to certify the generated certificate for
# the -CA option specifies the certificate to use to sign the generated certificate
# the -CAkey option specifies the private key to use to sign the generated certificate
# the -in option specifies the input certificate request
# the -out option specifies the file to write the generated certificate to
openssl x509 \
        -req \
        -extfile openssl.cnf \
        -extensions req_server \
        -CA ca.crt \
        -CAkey ca.key \
        -in server.csr \
        -out server.crt \
        -set_serial 01 \
        -sha256 \
        -days 3650
