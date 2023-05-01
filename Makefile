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
	cd $(PFP_WG_PATH) && make tfm_index_construct.x && ./tfm_index_construct.x -i $(CURRENT_DIR)/$(firstword $(filter-out $@,$(MAKECMDGOALS))) -o $(CURRENT_DIR)/$(word 2,$(filter-out $@,$(MAKECMDGOALS)))

visualize_haplotypes_sizes:
	for i in 5000 25000 50000 100000 200000 500000 1000000 2000000 5000000 10000000; do \
		make generate_haplotypes SAMPLE_SIZE=100 REGION_IN_BASE_PAIRS=$${i} OUTPUT_FILE=haplotypes_$${i}.txt; \
		make create_wg haplotypes_$${i}.txt wg_$${i}.txt; \
		make encode_haplotypes haplotypes_$${i}.txt haplotypes_encoded_$${i}.txt; \
		make create_wg haplotypes_encoded_$${i}.txt wg_from_encoded_$${i}.txt; \
	done
	python3 ./src/visualize.py
	for i in 5000 25000 50000 100000 200000 500000 1000000 2000000 5000000 10000000; do \
		rm -rf haplotypes_$${i}.txt haplotypes_encoded_$${i}.txt wg_$${i}.txt wg_from_encoded_$${i}.txt; \
	done