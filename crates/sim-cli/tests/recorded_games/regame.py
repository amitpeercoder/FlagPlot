# Plays every recorded game's requests (game.jsonl) again through `sim-cli mcp`
# and writes the answers and the saved game back as that game's transcript.jsonl
# and save.json. Run from the repository root after `cargo build --release`:
#
#     python3 crates/sim-cli/tests/recorded_games/regame.py
#
import json,subprocess,os,shutil,glob,tempfile
for d in sorted(glob.glob('crates/sim-cli/tests/recorded_games/*/')):
    lines=[l for l in open(d+'game.jsonl').read().splitlines() if l.strip()]
    h=json.loads(lines[0])
    tmp=tempfile.mkdtemp()
    args=['target/release/sim-cli','mcp',h['scenario'],'--seat',h['seat'],'--save',tmp+'/save.json','--transcript',tmp+'/transcript.jsonl']
    if h.get('seed') is not None: args+=['--seed',str(h['seed'])]
    inp=[json.dumps({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}),json.dumps({"jsonrpc":"2.0","method":"notifications/initialized"})]
    for i,l in enumerate(lines[1:]):
        c=json.loads(l)
        inp.append(json.dumps({"jsonrpc":"2.0","id":i+1,"method":"tools/call","params":{"name":c['tool'],"arguments":c.get('arguments',{})}}))
    r=subprocess.run(args,input="\n".join(inp)+"\n",capture_output=True,text=True)
    assert r.returncode==0,r.stderr
    shutil.copy(tmp+'/save.json',d+'save.json'); shutil.copy(tmp+'/transcript.jsonl',d+'transcript.jsonl')
