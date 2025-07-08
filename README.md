# Denali

## TODO

- ~~remove H256 default~~
- ~~proper tx response written to block header~~
- store tx in db
- ~~respond to tx submission with a proper hash~~
- ~~discover existing plugins in directory~~
- version for plugin traits
- ~~microsecond timestamps~~
- timestamp phantoms
- create hello-world plugin
- builder (master / slave)
- plugin delete and upgrade
- HL API endpoints
- ~~config file~~
- merkle_tree_root
- ~~H256 -> String should be from, not try_from~~
- Result
- unwraps
- todos
- instrument
- println
- deploy two nodes, (need builder to config running as M or S)
- gossip
- consensus
- pub keys for programs

## Commands

Read the linker stuff:

```bash
objdump -j .comment -s target/debug/denali
```

Root:

```bash
curl -X GET 'http://localhost:3000/'
```

Get the chain count:

```bash
curl -X GET 'http://localhost:3000/chain-height'
```

Get the chain tip:

```bash
curl -X GET 'http://localhost:3000/tip'
```

Get the chain:

```bash
curl -X GET 'http://localhost:3000/get-chain'
```

Submit a vote transaction:

```bash
curl -X POST http://localhost:3000/submit -H "Content-Type: application/json" -d "{\"program\":\"vote\",\"payload\":{\"candidate\":\"candidate1\"}}"
```

Submit a fake transaction:

```bash
curl -X POST http://localhost:3000/submit -H "Content-Type: application/json" -d "{\"program\":\"fake\",\"payload\":{\"fake\":0}}"
```

```bash
curl -G http://localhost:3000/is-block -d "block_hash=66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925"
```

```bash
curl -G http://localhost:3000/is-block -d "block_hash=0000000000000000000000000000000000000000000000000000000000000000"
```

```bash
curl -G http://localhost:3000/get-block-header -d "block_hash=0000000000000000000000000000000000000000000000000000000000000000"
```

```bash
curl -G http://localhost:3000/get-block-transactions -d "block_hash=0000000000000000000000000000000000000000000000000000000000000000"
```

```bash
curl -X GET 'http://localhost:3000/events?start=1749207505632&end=1749207515632&log_type=xxx'
```
