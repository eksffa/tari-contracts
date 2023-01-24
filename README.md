# tari-contracts
C-Voting.rs 

The C-Voting.rs is a proof-of-concept contract for a voting system that utilizes Rust programming language and is built for the Tari Network. It leverages the native transactions features of Tari and Monero to achieve anonymity and fungibility for a privacy-rich, fully auditable, integrity-rich, and authentication-rich voting system. The contract includes ring signature and zero proof features, making voting as confidential, privacy-rich, and untraceable as Monero transactions.

The auditing process of the voting system is simple, trustless and mathematically proven, similar to verifying a transaction on the Monero network. Declaring a winner for the race and the validation of integrity of votes is as reliable, secure, and tamper-proof as a Monero wallet balance. However, for full disclosure, transparency, and auditing, a public method exists to expose the balance of the wallets containing more tokens (voting tokens).

In this implementation, voting tokens are opaque, meaning that the voter is not aware of holding a voting token until they use it or attempt to vote again. The use of Native RandomX as a proof-of-work (PoW) algorithm, which is CPU-friendly, allows for any organization to run their nodes and be a part of the network, including private organizations, government entities, political parties, independent auditors, and international observatories.

Exposing the voting system on the internet or not doing so is a decision left to the voting authority. Additional levels of authentication can also be added between the wallet, such as mTLS, JWE, JWS, JTI, and 2/FA-Hash-Always-Present over a DIGEST/HMAC authenticated endpoint. It might also allow an API Gateway to adopt OAuth2 additional layers of controls, including those compatible with OpenAPI FAPI requirements, avoding direct exposition of the RPC Server to the wallet. Tari Wallet for iOS and Android can be used to vote, as it is as simple as sending an opaque token to one of the candidates.

A voting wallet can also be a digital electoral title emitted by the government or a voting machine connected to a Tari node on the localhost of the voting machine.

Once a copy of the voting state is made, the whole sum of all the voting machines' local nodes with their transactions must agree on the results, reaching virtually up to 100% consensus of the election results among all voting zones and sections in the country. An unseen / unequal level of trustless confidence ever possible on any traditional elections system. Calculating and syncying the blockchain network will act as this proof of consensus and proof of work mathematical verification of each vote, while an unsynced copy can always be kept copied.

The contract is fully open-source and the depending daemon nodes, servers, algorights and protocolos are also fully open-source, providing a high level of security auditability and transparency to the voting system.

Fully open-source contract. Fully open-source nodes, servers, daemons and networks. Security brought to physical reality straight from the universe of the mathematical and logical abstracts.

TODO:
- audit, look for reentrance security issues, even thought the code is was written with that in mind
- test real use-cases, build a playground (unit tests from it)
- (maybe) make voting token balance explicit (deopaque)
- (consider) adding auditing counters such as number of issued / claimed tokens vs number of used tokens which can be less the issued but never greater
- I dont think we need special mechanisms for "NULL" or "BLANK" votes, registering 0x000::d34d and 0x000::b33f wallets as candidates will do the trick but maybe a discussion brings the need for a specific procedure
- add the opportunity for the voter to register a string / sentence he wishes to express his deepest thoughts about this election, which can not be accounted by voting the lesser evil (#noconsent)
- (silent alert) maybe implementing a mechanism which will intentionally revert the transaction if a coercion passphrase is configured for the private key, in this case if somehow someone is coerced to vote by a gang of criminals other than the current government he can intentionally and anonymously inform the authorities, make sure his vote wont be considered valid, while he has a temporary proof of vote in that wallet / candidate, and once the vote is mixed in the ring signature system it will be invalid due to bad signature and reverted, we have a problem with the native revert mechanism which could allow a privacy violation of the coercer could have access to the voting wallets after the election. Needs futher invetigation.
- a paper (maybe)
