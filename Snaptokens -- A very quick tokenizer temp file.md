TLDR; Snappy OSS tokenizer; ~40x faster than hf, ~2x faster than gigatoken on BPE, and ~60x faster than hf on Unigram.

![[Pasted image 20260917103712.png]]

Snaptokens is a BPE and Unigram tokenizer written in Rust, bit-identical to hugging-face tokenizersIt is it is 2.19× faster than Gigatoken, 13.06× faster than fastokens, and 46.41× faster than Hugging Face **To the best of my knowledge, that makes it the fastest open-source BPE and Unigram tokenizer available today.**

This blog post will dive into (some) internals, optimizations, and the development process. Assumes good fundamentals about language modeling and perf optimization.

## How do tokenizers work

The goal of a tokenizer is to take words and turn them into token IDs. These token IDs are used as indexes in an embedding table to get a pre-determined vector. This vector is then used as the initial hidden state of a transformer[^1]. 

The core requirement of a tokenizer is to create tokens which represent a coherent portion of a word, whilst being small enough to be an elementary unit. [^2]

Two obvious answers both fail:

- **One token per word.** Any word outside the table becomes an unknown token. The model cannot read it, and cannot write it.
- **One token per character.** Coverage is complete, but sequences become very long. Attention cost grows with the square of the sequence length, so this is expensive.

Thus, we come up with clever algorithms (BPE, Unigram, WordPiece, etc.) in order to get this task done.

```
"snaptokens outperforms everything"

 sn      apt     ok     ens     ␣outper    forms    ␣everything
 16184   2373    482    641     33597      23914    2279
```


Tokenization is split up into encoding (words->ids) and decoding(ids->words). 

| Encoding Stages | Purpose                                                                                                  | Example                                                               |
| --------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| Added Tokens    | Preserve special, immutable tokens for encoding. Mark them as tokens we are not allowed to interact with | "<\|endoftext\|> oranges" → [50256] + "oranges"                       |
| Normalizer      | Standardizes text (lowercasing, unicode normalization)                                                   | "Cafe´" → "café"                                                      |
| Pre-tokenizer   | Splits text into pieces the algorithm may never merge across                                             | "Schrodinger's Cat" -> ["Schrodinger's", "Cat"]                       |
| Apply algorithm | Apply an algorithm to turn our input format agnostic text into token IDs. We support BPE and Unicode     | ["Ġoutper", "forms"] → [33597, 23914]                                 |
| Post-processor  | Apply a pre-determined template to the output IDs.                                                       | [15496, 995] → [**1**, 15496, 995, **2**]  (Llama-style `<s>...</s>`) |

The decoder is simply mapping IDs back to standardized text. Tokenizer libraries do not handle stripping post-processor templates, so all decode does it apply the inverse of the algorithm chosen

# Tokenizer Algorithms explained

## 1. Byte Pair Encoding (BPE) [^3]


BPE was not designed originally designed for language. [Philip Gage published it in 1994](https://www.derczynski.com/papers/archive/BPE_Gage.pdf) in The C Users Journal as a compression method. His version replaced the commonest pair of bytes with a byte value that the data did not use, wrote the substitution table beside the compressed data, and repeated. He reported compression close to LZW with a faster and smaller expansion routine, which suited machines with little memory.

Byte-pair encoding merges the most frequent character pairs until we hit a pre-determined vocab size.

The following is pseudocode to train a BPE tokenizer.

```
count the words in the corpus            # word -> frequency
split each word into symbols             # characters, or bytes
repeat until the vocabulary is full:     # **outer loop**
    count every adjacent symbol pair, weighted by word frequency
    take the pair with the highest count
    append that pair to the merge list
    replace that pair with one symbol everywhere
```

Training produces one artifact--an **ordered list of merges**. 

Here is a snippet of the gpt-2 merge table:

| Rank | Left | Right  | Merge      |
| ---- | ---- | ------ | ---------- |
| 5378 | ol   | ved    | olved      |
| 5379 | ␣p   | owers  | ␣powers    |
| 5380 | ␣th  | r      | ␣thr       |
| 5381 | ␣rem | aining | ␣remaining |
| 5382 | ␣W   | ater   | ␣Water     |
| 5383 | L    | C      | LC         |

During encode, we are given a merge table, and a vocabulary (token -> id)

<iframe src="https://naman.world/embeds/bpe.html" title="GPT-2 BPE tokenization" sandbox="allow-scripts allow-forms" referrerpolicy="no-referrer" style="width:100%; height:980px; border:0; border-radius:8px;"></iframe>


There are a few key details that make BPE appealing:
- Counts are weighted by word frequency. A word that appears 900 times contributes 900 to each of its pairs.
- When constructing character pairs, we never cross a word boundary. Intra-word merging is not allowed, keeping separation between tokens cleaner.
- As a result, BPE is natively parallel. The **outer loop** is not, as the next merge depends on the current one.
- It is entirely deterministic and very simple.

BPE's characteristics make it a very prominent tokenizer algorithm, with virtually all modern models trained for BPE.
## Unigram

Unigram is a less mainstream, albeit a very interesting tokenizer algorithm.

[Kudo (2018)](https://aclanthology.org/P18-1007/) introduced it alongside subword regularization. The core difference compared to BPE is that  BPE builds a vocabulary by merging upward and segments greedily, and Unigram starts with too large a vocabulary, prunes downward, and segments by finding the most probable path.

### Training Unigram

The training process is the following:

#### 1. Find substrings

We start with an exhaustive vocabulary, containing every possible character, and a large amount of substrings. To compute the substrings, we do the following:
- Find all the suffixes of a word, and sort them alphabetically. For example with `banana`, we get `["a", "ana", "anana", "banana", "na", "nana"]`
- For each suffix, we will compute the LCP (Longest Common Prefix). This value represents how many leading characters a suffix shares with a suffix an index before it. For example, `anana` and `ana` share all of `ana`, so `anana` gets the value 3, for the 3 chars. Similarly, `nana` gets LCP of 2, since it shares `na` with `na`
- The amount of letters at the start of each suffix that two adjacent suffixes have in common, represents their depth. All suffixes that agree to a certain depth, form a block. For example:
	- `ana` and `anana` share 3 letters--`ana`. Thus, we can define `ana` and `anana` as a block of depth 3. 
	- `a`, `ana`, and `anana` have a depth=1. So we can declare a block over these three suffixes.[^4]

We then walk the suffixes(`"a", "ana", "anana", "banana", "na", "nana"`), finding blocks. Once the suffix we are looking at dosen't have the same first characters as the ones part of the current block, we close the block.

After each block is closed, we store the shared prefix for the block along with the size of the block (amount of suffixes involved in the block)

Here is how we walk the graph: 

| Step | Row | Suffix   | LCP | What happens             | Stack    | Output   |
| ---- | --- | -------- | --- | ------------------------ | -------- | -------- |
| 1    | 1   | `ana`    | 1   | open depth 1             | `[1]`    |          |
| 2    | 2   | `anana`  | 3   | open depth 3             | `[1, 3]` |          |
| 3    | 3   | `banana` | 0   | close depth 3 — rows 1–2 | `[1]`    | `ana` ×2 |
| 4    | 3   |          |     | close depth 1 — rows 0–2 | `[]`     | `a` ×3   |
| 5    | 4   | `na`     | 0   | nothing open             | `[]`     |          |
| 6    | 5   | `nana`   | 2   | open depth 2             | `[2]`    |          |
| 7    | end | —        | 0   | close depth 2 — rows 4–5 | `[]`     | `na` ×2  |

We are now left with something like this:

|Substring|Count|
|---|---|
|`a`|3|
|`ana`|2|
|`na`|2|

#### 2. Score substrings
Then, we score each substring. $\text{Score} = \text{Count} \times \text{Length}$ [^5]

| Substring | Count | Length | Score |
| --------- | ----- | ------ | ----- |
| `ana`     | 2     | 3      | **6** |
| `na`      | 2     | 2      | 4     |
| `a`       | 3     | 1      | 3     |
Longer substrings are ranked higher since they compress information better. We keep the top _N_ by score. Then, regardless of score, we add every required character, all 256 byte pieces, if byte fallback is on, and all special tokens we allocated by hand.

#### Initialize probabilities 
We now give each substring a starting probability: $-\log \frac{c(x)}{\sum_{y \in V} c(y)}$ where $c(\dots)$ is substring frequency. 

| Substring | Count | Share | log prob |
| --------- | ----- | ----- | -------- |
| `a`       | 3     | 3/7   | −0.85    |
| `ana`     | 2     | 2/7   | −1.25    |
| `na`      | 2     | 2/7   | −1.25    |
We count how many times each piece occurs as a substring, and turns those counts into probabilities. This is intuitively quite crude. Probability should reflect how often a piece actually gets used, but raw counts assume every appearance is used, and appearances overlap and compete, so only one can win. The probs in this case also don't form a coherent distribution.

Thankfully, these probs are meant to be replaced.

4. Our goal is to turn these naive probs into something more logical

There is a circular dependency:

- To know a piece's probability, you need to know how often it appears in segments of words .
- To choose segmentations, you need substring probabilities.

To solve this, we use EM, in wwhich we alternate between expectation and maximization steps in order to converge onto accurate probabilities

#### Expectation–Maximization
##### **Expectation:**

For each word, enumerate its segmentations[^6]. Score each one as the product of its substrings' probabilities, and normalize those scores so they sum to 1. We are guaranteed that segmentations are created of substrings, because we use the same corpus to create both, segmentations, and substrings.

The output is an **expected count** per piece: how often the piece is used, averaged over all segmentations, weighted by how likely each is.

bandana -> band|ana -> $\frac{p(\texttt{band})\,p(\texttt{ana})}{\sum_{s} \prod_{x \in s} p(x)}$

The normalizer is the sum of the scores of **every** segmentation of that word.


Here is a walk-through example of Expectation.
```
banana ×3

banana              0.743  →  banana +2.23
ban|ana             0.134  →  ban +0.40,  ana +0.40
ban|an|a            0.034  →  ban +0.10,  an +0.10,  a +0.10
ban|a|na            0.027  →  ban +0.08,  a +0.08,   na +0.08
b|an|ana            0.018  →  b +0.05,    an +0.05,  ana +0.05
... 12 more splits share the remaining 0.044


bandana ×2

band|ana            0.404  →  band +0.81,  ana +0.81
ban|dana            0.253  →  ban +0.51,   dana +0.51
band|an|a           0.102  →  band +0.20,  an +0.20,  a +0.20
band|a|na           0.082  →  band +0.16,  a +0.16,   na +0.16
b|an|dana           0.034  →  b +0.07,     an +0.07,  dana +0.07
... 22 more splits share the remaining 0.125
```

This produces:

| Substring                              | Expected count |
| -------------------------------------- | -------------- |
| `banana`                               | 2.230          |
| `ana`                                  | 1.459          |
| `band`                                 | 1.218          |
| `ban`                                  | 1.193          |
| `a`                                    | 0.858          |
| `dana`                                 | 0.587          |
| `an`, `b`, `na`, `n`, `d`, `and`, `nd` | 1.536 combined |
| Total                                  | 9.082          |

Expected count is a substring's weighted usage across the corpus, averaged over all segmentations. We calculate expected count for the entire corpus, for all substrings, ending up with one value per substring.

###### **Maximization**

Maximization simply turns expected count values back into probabilities

$$p(x) = \frac{E[x]}{\sum_{y \in V} E[y]}$$
Thus, the table above becomes:

| Piece                                  | p(x)            |
| -------------------------------------- | --------------- |
| `banana`                               | 0.2455          |
| `ana`                                  | 0.1606          |
| `band`                                 | 0.1341          |
| `ban`                                  | 0.1314          |
| `a`                                    | 0.0945          |
| `dana`                                 | 0.0646          |
| `an`, `b`, `na`, `n`, `d`, `and`, `nd` | 0.1691 combined |
| Total                                  | 1.0000          |
#### 5. Pruning

Thus far, we have been quite generous with allowing a very large set of tokens. In the effort of fixing this, we prune tokens that are redundant. and whose work another piece can absorb costs almost nothing to delete.

1. First, we run Viterbi once per word to find the optimal segmentation. Using that information, we count how often each substring is used. Call that `freq[x]`.

Viterbi is the algorithm that finds the **single best path** through a set of choices, without checking every path.

To fill `best[k]`, look at every piece that **ends** at position _k_. Each one starts somewhere earlier, at _j_. The score is `best[j] + score(substring)`. 

For example, Viterbi to compute optimal segment for `bandana`:

| End | Prefix covered | Best score | Reached by     |
| --- | -------------- | ---------- | -------------- |
| 1   | `b`            | −3.0       | `b`            |
| 2   | `ba`           | −5.0       | `b` + `a`      |
| 3   | `ban`          | −2.1       | `ban`          |
| 4   | `band`         | −1.9       | `band`         |
| 5   | `banda`        | −3.9       | `band` + `a`   |
| 6   | `bandan`       | −4.1       | `band` + `an`  |
| 7   | `bandana`      | −3.5       | `band` + `ana` |

2. We can then delete every piece with `freq[x] = 0`. We have a new, smaller vocab set.
3. With the vocab we are left with, we find the best way to segment surviving substrings using the rest of the vocabulary. We then measure the cost of every replacement substring. [^8]

| Piece              | freq | Replacement | Loss                        |
| ------------------ | ---- | ----------- | --------------------------- |
| `ana`              | 2.0  | `an` `a`    | 3.511                       |
| `band`             | 2.0  | `ban` `d`   | 3.511                       |
| `banana`           | 3.0  | `ban` `ana` | 3.149                       |
How do we compute loss though?
$$\text{loss}(x) = \text{freq}(x) \cdot \left( \log p(x) - \sum_{a \in A(x)} \log p'(a) \right)$$
Three inputs, all already available:

- `freq(x)`: how often Viterbi used the piece, from step 1.
- `A(x)`: the replacement segmentation, from step 2.
- `loss(x)`: the price of deleting piece `x`

 The bracket measures how much score is lost after after replacing with replacement substrings. Multiply by the number of places this happens, and we get loss.

#### Looping during training

After doing initialization via steps 1-3, we repeat steps 4 and 5, tuning and pruning recursively, until we get to our desired vocab size.

For example:

```1,000,000 pieces
  → EM ×2 → cut 25% → 750,000
  → EM ×2 → cut 25% → 562,500
  → EM ×2 → cut 25% → 421,875
  → ...
  → 32,000  ← stop
```

### Unigram encoding

During encode we are given a vocabulary where substring maps to an ID and a log probability. The log probability scores how good that substring is, and using Viterbi we pick the segmentation who scores highest.


<iframe src="https://naman.world/embeds/unigram.html" title="T5 Unigram tokenization" sandbox="allow-scripts allow-forms" referrerpolicy="no-referrer" style="width:100%; height:1180px; border:0; border-radius:8px;"></iframe>
### tokenizers.json

Hugging Face’s tokenizer.json is the standardized implementation for storing information to use a tokenizer. It stores relevant data and metadata in one file.

The following example is for BPE:
```json
{
    "normalizer": {
      "type": "Lowercase"
    },

    "pre_tokenizer": {
      "type": "WhitespaceSplit"        // Discard whitespace and encode each piece separately.
    },

    "model": {
      "type": "BPE",
      "vocab": {                       // Token spelling → token ID; IDs don't set merge priority.
        "[UNK]": 0,
        "h": 1,
        "i": 2,
        "hi": 3,
        "t": 4,
        "e": 5,
        "r": 6,
        "th": 7,
        "there": 8,
        ...
      },
      "merges": [                      // Earlier entries have higher priority.
        ["h", "i"],                    // Each pair merges into its concatenated spelling.
        ["t", "h"],
        ["th", "e"],
        ["r", "e"],
        ...
      ],
      "unk_token": "[UNK]"             // Used when the vocabulary cannot represent input.
    },

    "added_tokens": [
      {
        "id": 0,
        "content": "[UNK]",            // Recognize this spelling without splitting it through BPE.
        "single_word": false,          // Matching doesn't require word boundaries.
        "lstrip": false,               // Don't absorb whitespace before the match.
        "rstrip": false,               // Don't absorb whitespace after the match.
        "normalized": false,           // Match before normalization.
        "special": true                // Can be omitted when decoding with skip_special_tokens.
      }, 
      ...
    ],

    "post_processor": null,            // No additional processing, such as inserting special tokens.
    "decoder": null                    // No custom transformation to reverse ByteLevel or Metaspace.
  }
```

<span style="color: gray; font-size: 0.8em;">The optimization section will contain higher-level algorithmic stuff and the internal section will contain lower-level CPU perf stuff. The latter will show more code. </span>
# Optimizations
<span style="color: gray; font-size: 0.8em;">This section will discuss higher-level algorithmic optimizations to snaptokens, as well as context around how it is structured</span>

Snaptokens implements BPE and Unigram tokenization in Rust, with hugging face exact Python bindings. We will break down the parts that compose of snaptokens here.
### Scanners

Scanners (src/pre_tokenizers/scanner) split text into the chunks that BPE processes separately. For example, a scanner might split "hello world!" into "hello", " world", and "!". Most tokenization libraries accept regex to define how we split the input, and pass it to an external dependency to process.

Some examples of what our scanners do:
- The Qwen scanner (src/pre_tokenizers/scanner/mask_scanner.rs:914) 

However, there is a neat pattern we can take advantage of--most models use the same regex. Because of this, specialized scanners implement common regex patterns directly. Specialized scanners only need to handle one known pattern, so they can do less work than a general regex engine.

Modern regex engines construct a state machine (or in modern implementations, multiple) by compiling the regex, then walking through the input and providing results after working. [^10] They are required to support arbitrary regex, meaning the implementations are generalized, which in programming, often corresponds to slow.

On the other hand, we can exploit relationships between the specific splitting rules to avoid checks and redundancy, and perform other algorithmic optimizations, since our path is predetermined. We also can add CPU other optimizations (branch reduction, cache locality, etc). 

We also use SIMD for specialized scanners. SIMD lets us apply the same check to several bytes with one instruction. Instead of checking 16 characters individually for spaces, we check all 16 together.

For "cat dog", SIMD checks all seven bytes for letters together. Written in input order, the resulting mask is:

```
  Input:                c a t   d o g
  Is a letter:          1 1 1 0 1 1 1
  Previous is a letter: 0 1 1 1 0 1 1
  Word starts here:     1 0 0 0 1 0 0
```

Shifting the "Is a letter" mask right 1 place, gives the “previous is a letter” row (mask b). Combining the information from these two bitmasks, we get "Word starts here", which marks `c` and `d` accurately.


### Fusing common paths

There are two major things we can exploit (at an architectural level) to get huge speedups:
- A majority of BPE tokenizers use ByteLevel as their pre-tokenizer [^9]
- A majority of Unigram tokenizers use the WhitespaceSplit and Metaspace pre-tokenizers, back-to-back.

For reference:
- **WhitespaceSplit** splits at whitespace. For example, "hello  world" becomes ["hello", "world"].
- **Metaspace** replaces spaces with a visible marker, usually ▁ For example, "hello world" becomes ["▁hello", "▁world"]
- **ByteLevel**: With all 256 byte values in its starting vocabulary, a byte-level BPE tokenizer can represent any UTF-8 text, including characters absent from its training corpus. This also restricts BPE’s starting alphabet to 256 symbols, while learned merges can expand the final vocabulary far beyond that. For example, it represents the space byte as Ġ, so "hello world" can become ["hello", "Ġworld"].

We exploit these common combinations by joining stages that would otherwise build intermediate results for the next stage to consume. 

For ByteLevel + BPE, the usual pipeline splits the text into pieces, converts their bytes into ByteLevel’s character representation, then runs BPE on each piece to produce token IDs. 

Our fused path passes the scanner’s UTF-8 bytes directly to BPE, using a precomputed table to map them to initial token IDs before applying the merge rules. For example, a space maps directly to the initial ID associated with Ġ, without constructing a string containing Ġ. This saves intermediate storage, copying, and conversion work.

```
Normally:

original byte → ByteLevel character → BPE token ID
space         → Ġ                  → ID for Ġ

Our fused path combines those two mappings:

original byte → initial BPE token ID
space         → ID for Ġ
```

For Unigram, the usual pipeline splits the text at whitespace, then applies Metaspace to each word. Our fused path combines whitespace splitting with preparing the Metaspace pieces. For large inputs, we also divide the text at safe whitespace boundaries so threads can process independent portions.
  
```
  Normally:

  "hello world"
      → ["hello", "world"]       WhitespaceSplit
      → ["▁hello", "▁world"]     Metaspace
      → token IDs               Unigram

  Our fused parallel path:

  find "hello" → prepare "▁hello" → Unigram → append IDs
  find "world" → prepare "▁world" → Unigram → append IDs
```

### Distributing and parallelizing independent work

Revisiting the earlier section, we noted that doing computation for BPE and unigram is natively parallel, such that we can tokenize every word at the same time. 

We divide adjacent slices of data into groups and distribute those groups across threads. Each thread processes its group sequentially. When our number of groups is more than # threads, we keep groups in a pool, and the a current available thread will pick one up. The ordinary paths complete pre-tokenization before distributing pieces across threads. BPE targets one group per thread, while Unigram targets two. [^11]

The fused BPE path targets two groups per thread, giving the scheduler additional work to distribute when some groups finish sooner. Unigram has 6 groups per thread. 



Normalization rules can match multiple characters, such as replacing "a b" with "x". If we divide the text between "a " and "b", neither task sees the full match. We
  therefore normalize sections independently only when our boundary checks preserve the result; otherwise, we normalize before dividing the input.

We choose the group count and size to balance keeping threads busy against the overhead of creating and processing groups. 

Empirically, these group sizes have performed best:

$P$ = thread count, $N$ = piece count, $L$ = input length in bytes.

| Path             | Target groups                              | Group size                                |
| ---------------- | ------------------------------------------ | ----------------------------------------- |
| Ordinary BPE     | $P$                                        | $\operatorname{ceil}(N / P)$ pieces       |
| Fused BPE        | $2P$                                       | $\operatorname{ceil}(N / 2P)$ pieces      |
| Ordinary Unigram | $2P$                                       | $\operatorname{ceil}(N / 2P)$ pieces      |
| Fused Unigram    | Approximately $6P$, subject to size limits | $L / 6P$ bytes, clamped to $16$–$128$ KiB |

 For small inputs, we skip internal multithreading and process the input on one thread since scheduling overhead can outweigh the time saved by parallel execution. For large enough BPE batches, we parallelize across inputs rather than within them, since separate inputs already provide enough work to occupy the thread.

## Processing input JSON and the `.st` filetype

A `.st` file is a saved representation of a tokenizer. 

The optimizable surface we can discuss is that the tokenizer.json is not the representation our encoders use directly. This is best proven via example:

Consider a tiny BPE model:

```json
  {
    "vocab": {
      "h": 0,
      "i": 1,
      "hi": 2
    },
    "merges": [
      ["h", "i"]
    ]
  }
```

During JSON loading, Snaptokens must parse the strings, convert merge lists to IDs, and prepare other structures. The merge above becomes something equivalent to:

```
  Input token pair:  (0, 1)
  Merge priority:   0
  Result token ID:  2
```

If we store the precompute used data structures as opposed to computing them after loading the JSON, we can save time when loading tokenizer data.
### What the file contains

The file has an 52-byte header, followed by a binary payload:

```
  tokenizer.st
  ├── Header
  │   ├── File identifier   # Identifies filetypes, even if someone changes the file extension
  │   ├── Format version    # Tell loader whether we are loading BPE (v3) or Unigram (v4)
  │   ├── Payload length    
  │   └── Payload checksum  
  └── Payload
      ├── config_json  # Config data like normalization, padding, truncation, etc.
      └── BPE tables or Unigram model inputs  
```

For BPE, the payload stores vocabulary spellings together in one byte buffer. Suppose token IDs 0, 1, and 2 represent "snap", "token", and "s":
```
Byte index:    0  1  2  3  4  5  6  7  8  9
Stored byte:   s  n  a  p  t  o  k  e  n  s

Offsets:      [0, 4, 9, 10]
```

For token ID n, offsets[n] marks its start and offsets[n + 1] marks its end, excluding the ending position:
  
 ```
ID 0: bytes [0, 4)  → "snap"
ID 1: bytes [4, 9)  → "token"
ID 2: bytes [9, 10) → "s"
 ```
We then store the byte offsets and concatenated tokens in two vectors, in a struct called `PackedVocabulary`. This helps us convert from IDs -> words. This is how the struct looks:

```rust
  struct PackedVocabulary {
      bytes: Vec<u8>,
      offsets: Vec<u32>,
  }
```

However, we still need to convert token spellings into IDs. `PackedVocabulary` stores spellings in token-ID order, so finding an ID from a spelling would require searching through the vocabulary. 

To resolve this, we use `VocabLookup` to provide this reverse lookup efficiently.
```rust
 struct VocabLookup {
      mask: usize,
      hashes: Vec<u64>,
      ids: Vec<u32>,
  }
```

An eight-slot lookup could look like this:
  ```
Slot:     0   1   2   3   4   5   6   7
Hash:     0   9  17   0   0   0  22   0
Token ID: —   0   2   —   —   —   1   —

	mask = 7  # selects the bits of a hash that determine the array position
  ```

  To find "s":
  1. Compute its hash: suppose it is 17.
  2. Calculate the starting slot: 17 & 7 = 1. Since the capacity is a power of two, this works like 17 % 8.
  3. Slot 1 contains hash 9, so check the next slot.
  4. Slot 2 contains hash 17, pointing to token ID 2.
  5. Retrieve ID 2’s spelling from `PackedVocabulary` and compare its bytes with "s". They match, so return 2.

This turns out to be more performant than a naive hash map because we store token hashes as opposed to token strings.

We also save each entry’s slot position in .st, so loading can restore the table directly instead of searching for a position for each entry through repeated insertions. like so:

```
 In memory:
      Slot:     0      1      2      3
      Entry:  empty   A    empty    B

  Saved in .st:
      Capacity: 4
      Entries:  (slot 1, A), (slot 3, B)
```


- `RankedMergeMap` maps pairs of token IDs to their merge rank and resulting token ID.
```rust
  struct RankedMergeMap {
      mask: usize,
      keys: Vec<u64>,   // Two 32-bit token IDs packed into each key.
      values: Vec<u64>, // Merge rank and resulting token ID, each 32 bits.
  }
```

For any circumstance, when we load data, we generate and store the following:

- **Initial byte-pair table**: Looks up a merge directly from two input bytes. For example, "s" followed by "n" might return priority 7 and the token ID for "sn". It contains all 256 × 256 = 65,536 byte combinations, with pairs that cannot merge marked as such. We always build this table , using the saved byte-to-token mapping and RankedMergeMap.

Its purpose is to quickly find which neighboring bytes can merge when BPE starts processing a piece.

For "snap", the starting pairs are:
```
  "s" + "n"
  "n" + "a"
  "a" + "p"
```
The data structure looks like this:

```rust
byte_pair_initial: Vec<(u32, u32)>,
```

The table gives each pair’s merge priority and resulting token ID directly. BPE compares the priorities to decide which merge happens first.

 - **Dense merge table**: Looks up a merge directly from two inital token IDs, including tokens produced by earlier merges [^]: "sn" and "ap" might merge into "snap". Each pair is stored at `left_id * width + right_id`, where width is the number of token IDs covered on each side.  there are two different types of dense merge tables. 
 We build `dense_ranked_merge` if all 256 initial byte-token IDs are below 1,024. It is best explained through example:

Suppose our tokenizer has two merge rules:

| Merge            | Rank | Resulting token ID |
| ---------------- | ---- | ------------------ |
| "s" + "n" → "sn" | 0    | 256                |
| "a" + "p" → "ap" | 1    | 257                |

The first four rows and columns of `dense_ranked_merge` would look like this:

| Left token ↓ / Right token | 0: "s" | 1: "n" | 2: "a" | 3: "p" |
| -------------------------- | ------ | ------ | ------ | ------ |
| 0: "s"                     | -      | 256    | -      | -      |
| 1: "n"                     | -      | -      | -      | -      |
| 2: "a"                     | -      | -      | -      | 257    |
| 3: "p"                     | -      | -      | -      | -      |
Each cell contains the resulting token ID. 

Using `dense_ranked_merge` makes querying 

If that variant cannot be built, `dense_merge` will be built if all initial byte-token IDs are below 512. If neither qualifies, we build neither. Pairs outside a built table’s range use `MergeAdjacency`.




- Character-to-token tables: These map individual characters directly to their vocabulary IDs. If token ID 12 spells "s", the entry for "s" contains 12, avoiding a string-hash lookup. Loading always builds a table for Unicode code points below 65,536 and a smaller ASCII table. Characters without a matching vocabulary token remain marked “no token.”

- Token-length table: This helps check whether a matched token covers an entire input piece. For example, "snap" occupies four bytes, so it cannot cover a five-byte piece by itself. Loading calculates lengths by subtracting neighboring vocabulary offsets. Short lengths are stored directly in the table; longer tokens receive a marker that tells encoding to check their full length using the offsets.


The impact of having this information stored in .st is fairly minor. This work happens when a tokenizer is loaded, and encoding commands reuse the structures already in memory.

## Viterbi algorithimic optimizations


# Internals
<span style="color: gray; font-size: 0.8em;">This section will discuss abstractions used and lower-level CPU performance optimizations to snaptokens.</span>

![[Pasted image 20260918180858.png]]
[^12]


### Footnotes

[^1]: Tokenizers + embeddings are used in most sequential language models (ex. RNN, LSTM, and GRU, etc.)

[^2]: This tends to be a syllable

[^3]: This is meant to be a refresher. If you don't know what BPE is, [this is a useful resource]([https://huggingface.co/learn/llm-course/en/chapter6/5](https://huggingface.co/learn/llm-course/en/chapter6/5))

[^4]: Yes, the same suffix can be located in two blocks at the same time

[^5]: Char substrings are excluded from scoring, since all chars are added by default regardless. They all get a score of 0

[^6]: 
	`band | an | a` and `ban  | dana` are both segments of `bandana`

[^7]: Required characters and special tokens remain immutable.
	
[^8]: Substrings are generally longer than shown in our toy examples.

[^9]: ex. GLM-5.x, dsv2, v3.x, and 4.x series, gpt-4.x, Llama 3+, etc.

[^10]: My definition is an extreme oversimplification. [Great article]([https://blog.gistre.epita.fr/posts/alban.duval-mottet-2025-06-28-understanding-regular-expression-engines/](https://blog.gistre.epita.fr/posts/alban.duval-mottet-2025-06-28-understanding-regular-expression-engines/)) about how regex engines work. [Here is a more detailed one](https://burntsushi.net/regex-internals/#problem-composition-was-difficult) from burntsushi.
 
 


## Writers scrappy notes

1. substring and piece is used interchangably but never dfefined. piece is more accurate, but we never transitioned cleanly, so we ended up using substring for everythingf.
2. I use "we", but idk when that is good grammar or no
3. Run panagram, and keep it in the Links section at the bottom.
4. Viterbi explanation is shit.
5. Show a little bit more code in the optimizations section
6.  well let's finish the examples of scanners section in optimzatios


Next sections:

Optimizations
Internals
API Usage
Benchmarking
How tokenization affects inference speeds, broadly.
Links


 things remaining to talk about in the optimization section"
 

  3. Finding Unigram candidates and choosing the best sequence. Explain how our vocabulary matcher finds overlapping token candidates and feeds them into the best-path
     calculation, retaining the best score and predecessor at each position. Focus on how our representation avoids unnecessary intermediate work; the best-path algorithm
     itself isn’t unique to Snaptokens.
  4. Post-tokenization stuff, and optimizations for that.

[^11]: Recall that fused paths require fusing pre-tokenization and encoding, so the steps cannot be split

[^12]: I apologize for the lack of graphs, I am not a very visual person
