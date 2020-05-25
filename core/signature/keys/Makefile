build_keys: private public ders
	echo "done building keys"
private: # to reset key CAREFUL!
	#openssl genrsa -des3 -out private.pem 2048
	openssl genpkey -algorithm RSA -out private.pem
public:
	openssl rsa -in private.pem -outform PEM -pubout -out public.pem
ders: private_der public_der
	echo "done with ders"
private_der:
	#openssl x509 -inform pem -outform der -in certificate.pem -out openssl-certificate.der
	openssl rsa -inform pem -outform der -in private.pem -out private.der
public_der:
	openssl rsa -pubin -in public.pem -inform PEM -RSAPublicKey_out -outform DER -out public.der
