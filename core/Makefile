compile:
	RUST_BACKTRACE=1 cargo check --verbose;
alice: clean_alice
	RUST_BACKTRACE=1 cargo run node-name="alice's node" node-id=1 port=8081 peers=127.0.0.1:8082,127.0.0.1:8083 ip=127.0.0.1:8081  #> run.log;
bob: clean_bob
	RUST_BACKTRACE=1 cargo run node-name="bob's node" node-id=2 port=8082 peers=127.0.0.1:8081,127.0.0.1:8083 ip=127.0.0.1:8082 #> run.log;
cici: clean_cici
	RUST_BACKTRACE=1 cargo run node-name="cici's node" node-id=3 port=8083 peers=127.0.0.1:8082,127.0.0.1:8081 ip=127.0.0.1:8083  #> run.log;
reset_alice:
	rm storage/.DS_Store ; rm -r .DS_Store ; make clean_alice ; make get ; make alice
reset_bob:
	rm storage/.DS_Store ; rm -r .DS_Store ; make clean_bob ; make get ; make bob
reset_cici:
	rm storage/.DS_Store ; rm -r .DS_Store ; make clean_cici ; make get ; make cici
clean_alice:
	rm -f ./storage/proposals.db ;
	rm -rf ./storage/proposal/* ;
	rm -rf ./storage/chain/* ;
	rm -rf ./storage/chain.db ;
	rm -rf ./storage/transactions.db ;
	rm -rf ./storage/transaction/* ;
	rm -rf ./storage/state.db ;
	rm -rf ./storage/state/* ;
	make create_directories ;
clean_bob:
	rm -f ./storage/proposals.db ;
	rm -rf ./storage/proposal/* ;
	rm -rf ./storage/chain/* ;
	rm -rf ./storage/chain.db ;
	rm -rf ./storage/transactions.db ;
	rm -rf ./storage/transaction/* ;
	rm -rf ./storage/states.db ;
	rm -rf ./storage/state/* ;
	make create_directories ;
	#cargo clean ;
clean_cici:
	rm -f ./storage/proposals.db ;
	rm -rf ./storage/proposal/* ;
	rm -rf ./storage/chain/* ;
	rm -rf ./storage/chain.db ;
	rm -rf ./storage/transactions.db ;
	rm -rf ./storage/transaction/* ;
	rm -rf ./storage/states.db ;
	rm -rf ./storage/state/* ;
	make create_directories ;
	#cargo clean ;
create_directories:
	mkdir -p ./storage/proposal/
	mkdir -p ./storage/chain/
	mkdir -p ./storage/transaction/
	mkdir -p ./storage/state/
build:
	cargo build --verbose;
save:
	git add * -v; git commit -am ${M} -v  ; git push origin master:development -v
get:
	git fetch ; git pull origin master;
rms:
	rm ./storage/proposal/*
stress:
	while true; do make stress_a; sleep 60 ; make stress_b; sleep 60 ; make stress_c; sleep 60 ; done
p_alice:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" 127.0.0.1:8081/proposal/create/
p_bob:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" 127.0.0.1:8082/proposal/create/
p_cici:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" 127.0.0.1:8083/proposal/create/
output:
	curl -d 'TESTSTRING' --header "Origin: test_user" --header "User-Agent: e2a8aca48c5b24df14c6e0ab0b30df7ed50fa97bc22fd706c71a7eebe96a8b67 10" 127.0.0.1:8081/transaction/submit/output/ ;
input:
	curl -d 'TESTSTRING' --header "Origin: test_user_2" --header "User-Agent: test_user ${tx_hash} opEssZ7CaoYvtZJErFPqiB0L+lxwFm1/YT3tLZ+07fCnwWvuRcXtpwmo4esdNs05OItDBK6SZaxVPO+tKG22NC8R64DQj4J6CXpt4XMxtGJSUeY9MyZB6eyW8qYye7zascGv5+Eht4VJ5Zu9TX8Xl2+oyZA+3RYw5QKHvMgHyN0mpPU8PYpBDdVKg5Nglh4WOjqrvJF/EAdyqfeLN0CNJHeFwwjlkDaOz1x9LBOBf8c5HhDulgblSd4tlJ9zRA97SbnxmQtip/XDLweTtCx9vmjFd0tw/JTcfl2V87r+JgxL0r9EgEoFsexs3XkdqKZ2LzypPMvp0XqeoNEJ03g96A== MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB" 127.0.0.1:8081/transaction/submit/input/ ;
stress_output:
	while true; do make output; sleep 60; done
stress_input:
	while true; do tx_hash=cb2c68c3ca7d2ce1411d3f1c6f501e6e1605814603e70bbdf0fd9dc113267408 make output; sleep 60; done
stress_a:
	#curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" --header "User-Agent: 100,200,test_string" 127.0.0.1:8081/transaction/submit/output ;
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" --header "User-Agent: e2a8aca48c5b24df14c6e0ab0b30df7ed50fa97bc22fd706c71a7eebe96a8b67 10" 127.0.0.1:8081/transaction/submit/output/ ;
stress_b:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" --header "User-Agent: 100,200,test_string" 192.168.0.8:8082/transaction/submit/output/ ;
stress_c:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: 100,200,test_string" 192.168.0.8:8083/transaction/submit/output/ ;
api_height:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: NOT_USED" 127.0.0.1:8081/API/block/height/ ;
api_block_id:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: 2" 127.0.0.1:8081/API/block/get/ ;
api_latest_p:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: NOT_USED" 127.0.0.1:8081/API/proposal/latest/ ;
sat:
	for i in {1..2}; do curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" --header "User-Agent: 100,200,test_string" 127.0.0.1:8081/transaction/submit/output ; done
	for i in {1..2}; do curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" --header "User-Agent: 100,200,test_string" 192.168.0.8:8082/transaction/submit/output ; done
	for i in {1..2}; do curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: 100,200,test_string" 192.168.0.8:8083/transaction/submit/output ; done
sap:
	for i in {1..1}; do curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" 127.0.0.1:8081/proposal/create/ ; done
	for i in {1..1}; do curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" 192.168.0.8:8082/proposal/create/ ; done
	for i in {1..1}; do curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" 192.168.0.8:8083/proposal/create/ ; done
create_proposal:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" 127.0.0.1:8081/proposal/create/
test:
	cargo test ;
	make test_all ;
test_all: test_proposal test_network test_encode test_timestamp test_block test_hash test_signature test_transaction test_macros
	echo "Done testing all..."
test_main:
	cargo test ;
test_node:
	cd node/ ; cargo test ;
test_proposal:
	cd proposal; RUST_BACKTRACE=1 cargo test --verbose;
test_network:
	cd network; RUST_BACKTRACE=1 cargo test --verbose;
test_encode:
	cd encode; RUST_BACKTRACE=1 cargo test --verbose;
test_timestamp:
	cd timestamp; RUST_BACKTRACE=1 cargo test --verbose;
test_block:
	cd block; RUST_BACKTRACE=1 cargo test --verbose;
test_hash:
	cd hash; RUST_BACKTRACE=1 cargo test --verbose;
test_signature:
	cd signature; RUST_BACKTRACE=1 cargo test --verbose;
test_transaction:
	cd transaction; RUST_BACKTRACE=1 cargo test --verbose;
test_macros:
	cd macros; RUST_BACKTRACE=1 cargo test --verbose;
dbm:
	cd ../ ; make dbm ; cd ./core/ ; make racl #; make ldr
dball:
	make dba ;
	make dbb ;
	make dbc ;
dba:
	time docker build --force-rm --rm --network="host" --file "./DockerfileAlice" --build-arg nodeType=alice -t alice_node .
dbb:
	time docker build --force-rm --rm --network="host" --file "./DockerfileBob" --build-arg nodeType=bob -t bob_node .
dbc:
	time docker build --force-rm --rm --network="host" --file "./DockerfileCici" --build-arg nodeType=cici -t cici_node .
rac: # run all containers from docker hub
	docker run --log-opt max-size=10m -d -p 8081:8081 kuntalabs/africaos:latest ./core node-name="alices node" node-id=1 port=8081 peers=docker.for.mac.host.internal:8082,docker.for.mac.host.internal:8083 ip=docker.for.mac.host.internal:8081
	docker run --log-opt max-size=10m -d -p 8082:8082 kuntalabs/africaos:latest ./core node-name="bob's node" node-id=2 port=8082 peers=docker.for.mac.host.internal:8081,docker.for.mac.host.internal:8083 ip=docker.for.mac.host.internal:8082
	docker run --log-opt max-size=10m -d -p 8083:8083 kuntalabs/africaos:latest ./core node-name="cici's node" node-id=3 port=8083 peers=docker.for.mac.host.internal:8082,docker.for.mac.host.internal:8081 ip=docker.for.mac.host.internal:8083
racl: # run all containers local
	docker run --log-opt max-size=5m --log-opt max-file=10 -d -p 8081:8081 aos:latest ./core node-name="alices node" node-id=1 port=8081 peers=docker.for.mac.host.internal:8082,docker.for.mac.host.internal:8083 ip=docker.for.mac.host.internal:8081
	docker run --log-opt max-size=5m --log-opt max-file=10 -d -p 8082:8082 aos:latest ./core node-name="bob's node" node-id=2 port=8082 peers=docker.for.mac.host.internal:8081,docker.for.mac.host.internal:8083 ip=docker.for.mac.host.internal:8082
	docker run --log-opt max-size=5m --log-opt max-file=10 -d -p 8083:8083 aos:latest ./core node-name="cici's node" node-id=3 port=8083 peers=docker.for.mac.host.internal:8082,docker.for.mac.host.internal:8081 ip=docker.for.mac.host.internal:8083
# ldr: # local docker run
# 	cd ../ ; make dbm ; cd ./core ; make racl ;
new_keys:
	#cd ./signature/keys ; openssl genpkey -algorithm RSA -aes-256-cbc -outform PEM -out private_key.pem -pkeyopt rsa_keygen_bits:2048 ; chmod 0400 private_key.pem ; ssh-keygen -e -f private_key.pem > public_key.pem ; ls ;
	#cd ./signature/keys ; openssl genpkey -algorithm RSA -aes-256-cbc -outform PEM -out private_key.pem -pkeyopt rsa_keygen_bits:2048 ; chmod 0400 private_key.pem ; ssh-keygen -e -f private_key.pem > public_key.pem ; ls ;
	cd ./signature/keys ; rm make ./private.der	./private.pem	./public.der ./public.pem ; make ; echo "done making keys" ;
stress_aws:
	while true; do make stress_a_aws; sleep 60 ; make stress_b_aws; sleep 60 ; make stress_c_aws; sleep 60 ; done
stress_a_aws:
	curl -d 'TESTSTRING' --header "Origin: ${ALICE}:8081" --header "User-Agent: 100,200,test_string" ${ALICE}:8081/transaction/submit/output ;
stress_b_aws:
	curl -d 'TESTSTRING' --header "Origin: ${BOB}:8082" --header "User-Agent: 100,200,test_string" ${BOB}:8082/transaction/submit/output ;
stress_c_aws:
	curl -d 'TESTSTRING' --header "Origin: ${CICI}:8083" --header "User-Agent: 100,200,test_string" ${CICI}:8083/transaction/submit/output ;
aws_docker_commands:
	echo "docker run --log-opt max-size=5m --log-opt max-file=10 -d -p 8081:8081 kuntalabs/africaos:latest ./core node-name='alices node' node-id=1 port=8081 peers=${BOB}:8082,${CICI}:8083 ip=${ALICE}:8081"
	echo "docker run --log-opt max-size=5m --log-opt max-file=10 -d -p 8082:8082 kuntalabs/africaos:latest ./core node-name='bobs node' node-id=2 port=8082 peers=${ALICE}:8081,${CICI}:8083 ip=${BOB}:8082"
	echo "docker run --log-opt max-size=5m --log-opt max-file=10 -d -p 8083:8083 kuntalabs/africaos:latest ./core node-name='cicis node' node-id=3 port=8083 peers=${BOB}:8082,${ALICE}:8081 ip=${CICI}:8083"
