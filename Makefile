save:
	git add * ; git commit -am "checkpoint from root" ; git push origin master:development -v
dbm:
	time docker build --build-arg nodeType="test" -t aos .
