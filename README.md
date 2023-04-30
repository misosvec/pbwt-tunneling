# PBWT-Tunneling
## Makefile
`generate_haplotypes SAMPLPE_SIZE=<?> REGION_IN_BASE_PAIRS=<?> OUTPUTFILE=<?>`\
It generates haplotypes with help of [MaCS](https://code.google.com/archive/p/macs/) project. To run this you have to download [MaCS](https://code.google.com/archive/p/macs/) and setup it. After that, you need to set `MACS_PATH` variable in the `Makefile`.

`encode_haplotypes input.txt output.txt`\
It encodes haplotypes from an input file and outputs it as a separate file.

`create_wg input.txt output.wg`\
It creates a wheeler graph from a file with help of [pfp_wg](https://github.com/miso01/pfp_wg) repository. Before running, you need to set `PFP_WG_PATH` variable in the `Makefile`.


