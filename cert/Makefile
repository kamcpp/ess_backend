.PHONY: all
all: gen-root-ca-cert gen-admin-pfx gen-server-cert

.PHONY: clean
clean:
	 rm -rf *.crt *.csr *.key *.srl *.key *.pfx

.PHONY: gen-root-ca-keypair
gen-root-ca-keypair:
	openssl genrsa -out root-ca.key 4096

.PHONY: gen-root-ca-cert
gen-root-ca-cert: gen-root-ca-keypair
	openssl req -x509 -new -nodes -key root-ca.key -subj "/C=US/ST=FL/O=Encryptizer, Inc./CN=encryptizer.com"-sha256 -days 1024 -out root-ca.crt

.PHONY: gen-admin-keypair
gen-admin-keypair:
	openssl genrsa -out admin.ess.encryptizer.com.key 3072

.PHONY: gen-admin-csr
gen-admin-csr: gen-admin-keypair
	openssl req -new -sha256 -key admin.ess.encryptizer.com.key -subj "/C=US/ST=FL/O=Encryptizer, Inc./CN=admin.ess.encryptizer.com" -out admin.ess.encryptizer.com.csr

.PHONY: gen-admin-cert
gen-admin-cert: gen-admin-csr
	openssl x509 -req -in admin.ess.encryptizer.com.csr -CA root-ca.crt -CAkey root-ca.key -CAcreateserial -out admin.ess.encryptizer.com.crt -days 500 -sha256

.PHONY: gen-admin-pfx
gen-admin-pfx: gen-admin-cert
	openssl pkcs12 -export -out admin.ess.encryptizer.com.pfx -inkey admin.ess.encryptizer.com.key -in admin.ess.encryptizer.com.crt -passin pass:password -passout pass:password

.PHONY: gen-server-keypair
gen-server-keypair:
	openssl genrsa -out ess.encryptizer.com.key 3072

.PHONY: gen-server-csr
gen-server-csr: gen-server-keypair
	openssl req -new -sha256 -key ess.encryptizer.com.key -subj "/C=US/ST=FL/O=Encryptizer, Inc./CN=ess.encryptizer.com" -out ess.encryptizer.com.csr

.PHONY: gen-server-cert
gen-server-cert: gen-server-csr
	openssl x509 -req -in ess.encryptizer.com.csr -CA root-ca.crt -CAkey root-ca.key -CAcreateserial -out ess.encryptizer.com.crt -days 500 -sha256
