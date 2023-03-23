#!/bin/bash

# Define the source directories for your contracts
contracts_dir_1=/lib/arbmod/contracts/
contracts_dir_2=/lib/portfolio/contracts/

# Loop through each directory and generate contracts
for contracts_dir in contracts_dir_1 contracts_dir_2
do
  echo "Generating contracts in $contracts_dir"
    forge bind \
        -C $contracts_dir 
    #   -b "/bindings/" \
    #   --crate-name bindings
done

echo "Done!"
