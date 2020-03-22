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
	git add * ; git commit -am "saved from core" ; git push origin master:development -v
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
stress_a:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" --header "User-Agent: 100,200,test_string" 127.0.0.1:8081/transaction/submit/output ;
stress_b:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" --header "User-Agent: 100,200,test_string" 192.168.0.8:8082/transaction/submit/output ;
stress_c:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: 100,200,test_string" 192.168.0.8:8083/transaction/submit/output ;
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
test_all: test_proposal test_network test_encode test_timestamp test_block test_hash test_signature
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
rac: # run all containers
	docker run -d -p 8081:8081 aos:latest ./core node-name="alices node" node-id=1 port=8081 peers=docker.for.mac.host.internal:8082,docker.for.mac.host.internal:8083 ip=docker.for.mac.host.internal:8081
	docker run -d -p 8082:8082 aos:latest ./core node-name="bob's node" node-id=2 port=8082 peers=docker.for.mac.host.internal:8081,docker.for.mac.host.internal:8083 ip=docker.for.mac.host.internal:8082
	docker run -d -p 8083:8083 aos:latest ./core node-name="cici's node" node-id=3 port=8083 peers=docker.for.mac.host.internal:8082,docker.for.mac.host.internal:8081 ip=docker.for.mac.host.internal:8083
