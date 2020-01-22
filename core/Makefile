compile:
	RUST_BACKTRACE=1 cargo check --verbose;
alice: clean_alice
	RUST_BACKTRACE=1 cargo run node-name="alice's node" node-id=1 port=8081 peers=127.0.0.1:8082,127.0.0.1:8083 ip=127.0.0.1:8081 ; #> run.log;
bob: clean_bob
	RUST_BACKTRACE=1 cargo run node-name="bob's node" node-id=2 port=8082 peers=127.0.0.1:8081,127.0.0.1:8083 ip=127.0.0.1:8082 ; #> run.log;
cici: clean_cici
	RUST_BACKTRACE=1 cargo run node-name="cici's node" node-id=3 port=8083 peers=127.0.0.1:8082,127.0.0.1:8081 ip=127.0.0.1:8083 ; #> run.log;
reset_alice:
	rm storage/.DS_Store  ; make clean_alice ; make get ; make alice
reset_bob:
	rm storage/.DS_Store  ; make clean_bob ; make get ; make bob
reset_cici:
	rm storage/.DS_Store  ; make clean_cici ; make get ; make cici
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
	git add * ; git commit -am "saved..." ; git push origin master
get:
	git fetch ; git pull origin master;
rms:
	rm ./storage/proposal/*
stress:
	while true; do make stress_a; sleep 600 ; make stress_b; sleep 600 ; make stress_c; sleep 600 ; done
stress_a:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" --header "User-Agent: 100,200,test_string" 127.0.0.1:8081/transaction/submit/output ;
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" 127.0.0.1:8081/proposal/create/
stress_b:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" --header "User-Agent: 100,200,test_string" 192.168.0.8:8082/transaction/submit/output ;
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8082" 192.168.0.8:8082/proposal/create/ ;
stress_c:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" --header "User-Agent: 100,200,test_string" 192.168.0.8:8083/transaction/submit/output ;
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8083" 192.168.0.8:8083/proposal/create/ ;
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
txo:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8081" --header "transaction_data: 100,200,test_string" 127.0.0.1:8081/transaction/submit/output
txi:
	curl -d 'TESTSTRING' --header "Origin: 127.0.0.1:8080\nTXDATA: 100,200,test_string" 127.0.0.1:8080/transaction/submit/input
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
