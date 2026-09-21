# Interpretation notes

Source audit after the frozen matrix was scheduled found that EncodeStream::push_long
and EncodeStream::flush explicitly pass use_shared_cache=false. GPT-2, Qwen-3 and
GPT-OSS use these direct fused paths for these configurations. Their rows must
not be sold as evidence that the shared-cache mutex, shared key representation,
or shared hasher improved. They remain useful controls, and their raw results
are retained in the predeclared equal-weight aggregate. Gemma-3 uses the ordinary
BPE path, where append_bpe_ids performs an exact-token shortcut, then a local
cache lookup, then shared-cache lookup on a local miss. The relevant two-cell
Gemma summary is explanatory/exploratory, not a silently substituted promotion
score. The local long-key hasher can affect both ordinary and fused paths.

The protocol's RAYON_NUM_THREADS controls the global pool. The separate BPE pool
in src/pre_tokenized.rs uses min(available_parallelism,8), independently of that
environment variable; this host has eight logical CPUs. Gemma scalar LongBench
may use that eight-thread pool; outer batches at four threads disable inner BPE
parallelism. The one-global-thread extra cell is GPT-OSS/LongBench (direct fused
path). It is not an uncontended shared-mutex test. No thread policy was changed.

Same-source A/A calibration cannot resolve two-percent changes on this busy host,
especially the sub-millisecond warm direct-fused calls. All rounds and outliers
are retained. A confidence interval overlapping parity means this screen did not
establish a performance difference, not that implementations are equivalent.

The atomics candidate uses an Acquire/Release spinlock per shard. The HashMap
contents remain non-atomic and protected by exclusive ownership. This does not
test a lock-free table, atomic publication of immutable snapshots, or reclamation
schemes; those would be different representations and different experiments.
No Miri, Loom or sanitizer run has been performed. The focused runtime stress
and poison tests do not exhaust possible concurrent executions.
