# Ballot Soroban contract
This repository contains a [soroban](https://soroban.stellar.org/) contract which allows users to participate in a ballot process.

## Build the contract

- Install Soroban and Rust SDK following the instructions [here](https://soroban.stellar.org/docs/getting-started/setup)
- Test the contract. To do it, execute the following command from the contract root folder:

```
cargo test
```
This will compile and execute tests. After checking they are successful, you can generate the wasm file
  
- To generate the wasm file, execute the following from the contract root folder

```shell
soroban contract build
```

After that, you will be able to deploy and install it. You can see how to use soroban-cli to deploy contracts in the [docs](https://soroban.stellar.org/docs/getting-started/hello-world). 
Before deploying the contract you will need to create a user. You can do it using "soroban config":

```shell
soroban config identity generate --global <your_user_name>
```
The above simple command will create a user. If you want to see the user's public key you can execute:

```shell
soroban config identity address <your_user_name>
```
For showing the private key you can use:

```shell
soroban config identity show <your_user_name>
```
When you have the user ready, add a network using the following command:

```shell
soroban config network add --global futurenet --rpc-url https://rpc-futurenet.stellar.org:443 --network-passphrase "Test SDF Future Network ; October 2022"
```

And now, you can deploy as follows:

```shell
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/<your_wams_file>.wasm --source the_owner --network futurenet
```
## Contract functions

**configure**: This function receives the contract admin (should be the same address which deployed the contract) and the ballot start and end timestamps. The, it stores the those timestamps using the *Config* struct so they can be checked later.

**vote**: This function receives the contract admin, the voter and the candidate. Both voter and candidate are received as a symbols so the contract invoker should be in charge of managing the voters identities in its side. The *vote* function has some rules:
   * You cannot not vote when the current timestamp is out of range (the range between *ts_start* and *ts_end* configured by which *configure* function).
   * You cannot vote if you have delegated your vote in another voter
   * You cannot vote if you have already voted

If the voter can vote, then the candidate will receive 1 vote more. If the voter has delegated votes, then the candidate will receive 1 vote more for every voter delegated vote.

**delegate**: This function receives the contract admin, the voter who wants to delegate his/her vote (*o_voter*) and the voter to delegate the vote to (*d_voter*). This function also has some rules:
   * You cannot not delegate your vote when the current timestamp is out of range
   * You cannot delegate your vote if you have already voted
   * You cannot delegate your vote if the target voter has already voted
   * You cannot delegate your vote if your voter is already delegated
   * You cannot delegate your vote if the target voter vote is already delegated
   * You cannot delegate your vote if there are voters who have delegated you their votes

If the vote can be delegated, the delegation is stored so it can be take into account later.

**count**: This function counts the votes for each candidates and returns the results.
 
