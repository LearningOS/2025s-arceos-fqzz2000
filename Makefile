docker:
	docker run --privileged --rm -it\
		-v $(PWD):/mnt \
		arceos \
		/bin/bash


