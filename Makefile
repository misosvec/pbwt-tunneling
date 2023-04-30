CURRENT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
MACS_PATH = /Users/michalsvec/Desktop/macs
PFP_WG_PATH = /Users/michalsvec/Desktop/pfp_wg

clear:
	rm -rf output/*

generate_haplotypes:
	cd $(MACS_PATH) && make && ./macs $(SAMPLE_SIZE) $(REGION_IN_BASE_PAIRS) -r 0.001 -t 0.001 | grep "SITE:" | awk '{print $$NF}' > $(CURRENT_DIR)/$(OUTPUT_FILE)

encode_haplotypes:
	cargo run --bin encode_haplotypes $(filter-out $@,$(MAKECMDGOALS))

create_wg:
	cd $(PFP_WG_PATH) && make tfm_index_construct.x && ./tfm_index_construct.x -w 2 -i $(CURRENT_DIR)/$(firstword $(filter-out $@,$(MAKECMDGOALS))) -o $(CURRENT_DIR)/$(word 2,$(filter-out $@,$(MAKECMDGOALS)))
