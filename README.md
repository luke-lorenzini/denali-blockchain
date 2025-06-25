# Denali

Root:

```bash
curl -X GET 'http://localhost:3000/'
```

Get the chain count:

```bash
curl -X GET 'http://localhost:3000/chain-height'
```

Submit a vote transaction:

```bash
curl -X POST http://localhost:3000/submit -H "Content-Type: application/json" -d "{\"program\":\"vote\",\"payload\":{\"candidate\":\"candidate1\"}}"
```

Submit a fake transaction:

```bash
curl -X POST http://localhost:3000/submit -H "Content-Type: application/json" -d "{\"program\":\"fake\",\"payload\":{\"fake\":0}}"
```
