CURRENT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
MACS_PATH = /Users/michalsvec/Desktop/macs

generate_haplotypes:
	cd $(MACS_PATH) && make && ./macs 10 2000 -r 0.001 -t 0.001 | grep "SITE:" | awk '{print $$NF}' > $(CURRENT_DIR)/haplotypes.txt