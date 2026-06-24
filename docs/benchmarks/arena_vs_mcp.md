# Arena vs MCP round-trip benchmark

**Exit witness (Phase 2):** in-process arena predict ≥ **5×** (stretch **10×**) vs one stdio MCP `tools/call` round-trip.

## Surfaces

| Surface | Boundary | When |
|---------|----------|------|
| MCP stdio | Cold | Agents, discovery, single-shot gate/predict |
| `load_arena(bytes)` | Warm | Owned buffer, parse once |
| `mmap_arena_path` (`feature = "mmap"`) | Warm | File-backed arena, zero-copy view |

## Harness (local)

```bash
cd umst-concrete-cartridge
cargo build -p umst-mcp --features agent-layer --release

# MCP baseline — one gate_check + predict (Python smoke pattern)
/usr/bin/time -p python3 -c "
import json, subprocess, time
from pathlib import Path
root = Path('.').resolve()
proc = subprocess.Popen(
    ['cargo', 'run', '-q', '-p', 'umst-mcp', '--features', 'agent-layer'],
    stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, cwd=root)
def rpc(p):
    p.stdin.write(json.dumps(p)+'\n'); p.stdin.flush()
    return json.loads(p.stdout.readline())
rpc({'jsonrpc':'2.0','id':1,'method':'initialize','params':{'protocolVersion':'2024-11-05','capabilities':{},'clientInfo':{'name':'bench','version':'0.1'}}})
mix = {'w_c':'9/20','temperature_k':'29315/100','aggregate_volume_fraction':'7/10'}
t0 = time.perf_counter()
for i in range(100):
    rpc({'jsonrpc':'2.0','id':2+i,'method':'tools/call','params':{'name':'umst_gate_check','arguments':{'mix':mix}}})
print('mcp_100_calls_sec', time.perf_counter()-t0)
proc.terminate()
"

# Arena path — parse-once loop (manifold crate)
cd ../umst-manifold
cargo test -p umst-runtime-arena --features mmap --release -- --nocapture
```

## Honest status (2026-06-24)

| Item | Status |
|------|--------|
| `mmap_arena_path` + UCRS `commit_stamp` read/write | **Shipped** (`feature = "mmap"`) |
| `examples/agent/06_arena_batch.py` | **Shipped** — in-process batch pattern doc |
| Published ≥5× ratio on CI hardware | **Partial** — harness doc only; run locally before Phase 2 exit |

Regenerate numbers after hardware change; paste wall-clock into `IMPLEMENTATION_EVIDENCE.md` P2 row.
