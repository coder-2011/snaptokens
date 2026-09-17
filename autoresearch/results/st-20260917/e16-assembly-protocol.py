import re,subprocess
from pathlib import Path
root=Path.home()/'st-campaign-20260917';out=root/'evidence/e16-assembly';out.mkdir()
for label,binary in [('parent',root/'combined-profile-st-eval'),('candidate',root/'e16-candidate-st-eval')]:
 symbols=subprocess.check_output(['nm','-S','-C',binary],text=True)
 matches=[line for line in symbols.splitlines() if line.endswith(' <snaptokens::models::bpe::Bpe>::from_native_tables')]
 (out/(label+'-symbols.txt')).write_text('\n'.join(matches)+'\n')
 for line in matches:
  words=line.split();start=int(words[0],16);size=int(words[1],16)
  asm=subprocess.check_output(['objdump','-dC','--no-show-raw-insn',f'--start-address={start}',f'--stop-address={start+size}',binary],text=True)
  (out/(label+'-native.asm')).write_text(asm)
  print(label,'bytes',size)
