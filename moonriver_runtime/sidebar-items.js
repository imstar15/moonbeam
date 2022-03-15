initSidebarItems({"constant":[["DAYS",""],["GAS_PER_SECOND","Current approximation of the gas/s consumption considering EVM execution over compiled WASM (on 4.4Ghz CPU). Given the 500ms Weight, from which 75% only are used for transactions, the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000."],["HOURS",""],["MAXIMUM_BLOCK_WEIGHT","Maximum weight per block"],["MILLISECS_PER_BLOCK",""],["MINUTES",""],["VERSION","This runtime version. The spec_version is composed of 2x2 digits. The first 2 digits represent major changes that can’t be skipped, such as data migration upgrades. The last 2 digits represent minor changes which can be skipped."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""],["WEEKS",""],["WEIGHT_PER_GAS","Approximate ratio of the amount of Weight per Gas. u64 works for approximations because Weight is a very small unit compared to gas."]],"enum":[["Call",""],["Event",""],["OriginCaller",""],["ProxyType","The type used to represent the kinds of proxying allowed."]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"mod":[["api",""],["currency","MOVR, the native token, uses 18 decimals of precision."],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core datastructures."],["xcm_config","XCM configuration for Moonbase."]],"struct":[["AdjustmentVariable","The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to change the fees more rapidly. This low value causes changes to occur slowly over time."],["AnnouncementDepositBase",""],["AnnouncementDepositFactor",""],["ApprovalDeposit",""],["AssetAccountDeposit",""],["AssetDeposit",""],["AssetRegistrar",""],["AssetRegistrarMetadata",""],["AssetsStringLimit",""],["BaseFeeThreshold",""],["BasicDeposit",""],["BlockGasLimit",""],["BlockHashCount",""],["BlockLength","We allow for 5 MB blocks."],["BlockWeights","We allow for one half second of compute with a 6 second average block time. These values are dictated by Polkadot for the parachain."],["CandidateBondLessDelay","Rounds before the candidate bond increase/decrease can be executed"],["CooloffPeriod",""],["CouncilMaxMembers","The maximum number of council members."],["CouncilMaxProposals","The maximum number of Proposlas that can be open in the council at once."],["CouncilMotionDuration","The maximum amount of time (in blocks) for council members to vote on motions. Motions may end in fewer blocks if enough votes are cast to determine the result."],["DealWithFees",""],["DefaultBaseFeePerGas",""],["DefaultBlocksPerRound","Blocks per round"],["DefaultCollatorCommission","Default fixed percent a collator takes off the top of due rewards"],["DefaultParachainBondReservePercent","Default percent of inflation set aside for parachain bond every round"],["DelegationBondLessDelay","Rounds before the delegator bond increase/decrease can be executed"],["DepositAmount",""],["EnactmentPeriod",""],["ExecutiveBody",""],["ExistentialDeposit",""],["FastTrackVotingPeriod",""],["FieldDeposit",""],["FindAuthorAdapter","The author inherent provides a AccountId20, but pallet evm needs an H160. This simple adapter makes the conversion."],["FixedGasPrice",""],["GenesisAccount","Account definition used for genesis block construction."],["GenesisConfig",""],["InflationInfo",""],["InitializationPayment",""],["Initialized",""],["InstantAllowed",""],["IsActive",""],["LaunchPeriod",""],["LeaveCandidatesDelay","Rounds before the collator leaving the candidates request can be executed"],["LeaveDelegatorsDelay","Rounds before the delegator exit can be executed"],["MaintenanceDmpHandler",""],["MaintenanceFilter","Maintenance mode Call filter"],["MaintenanceHooks","The hooks we wantt to run in Maintenance Mode"],["MaintenanceXcmpHandler",""],["MaxAdditionalFields",""],["MaxApprovals",""],["MaxBottomDelegationsPerCandidate","Maximum bottom delegations per candidate"],["MaxDelegationsPerDelegator","Maximum delegations per delegator"],["MaxInitContributorsBatchSizes",""],["MaxLocks",""],["MaxPending",""],["MaxProposals",""],["MaxProxies",""],["MaxRegistrars",""],["MaxReserves",""],["MaxScheduledPerBlock",""],["MaxSubAccounts",""],["MaxTopDelegationsPerCandidate","Maximum top delegations per candidate"],["MaxVotes",""],["MaximumSchedulerWeight",""],["MetadataDepositBase",""],["MetadataDepositPerByte",""],["MinBlocksPerRound","Minimum round length is 2 minutes (10 * 12 second block times)"],["MinCandidateStk","Minimum stake required to be reserved to be a candidate"],["MinCollatorStk","Minimum stake required to become a collator"],["MinDelegatorStk","Minimum stake required to be reserved to be a delegator"],["MinSelectedCandidates","Minimum collators selected per round, default at genesis and minimum forever after"],["MinimumDeposit",""],["MinimumMultiplier","Minimum amount of the multiplier. This value cannot be too low. A test case should ensure that combined with `AdjustmentVariable`, we can recover from the minimum. See `multiplier_can_grow_from_zero` in integration_tests.rs. This value is currently only used by pallet-transaction-payment as an assertion that the next multiplier is always > min value."],["MinimumPeriod",""],["MinimumReward",""],["MoonbeamGasWeightMapping",""],["NormalFilter","Normal Call Filter We dont allow to create nor mint assets, this for now is disabled We only allow transfers. For now creation of assets will go through asset-manager, while minting/burning only happens through xcm messages This can change in the future"],["OperationalFeeMultiplier",""],["Origin","The runtime origin type representing the origin of a call."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["PrecompilesValue",""],["PreimageByteDeposit",""],["ProposalBond",""],["ProposalBondMinimum",""],["ProxyDepositBase",""],["ProxyDepositFactor",""],["Range",""],["RelaySignaturesThreshold",""],["ReservedDmpWeight",""],["ReservedXcmpWeight",""],["RevokeDelegationDelay","Rounds before the delegator revocation can be executed"],["RewardPaymentDelay","Rounds before the reward is paid"],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["SS58Prefix",""],["SignatureNetworkIdentifier",""],["SpendPeriod",""],["SubAccountDeposit",""],["TargetBlockFullness","The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less than this will decrease the weight and more will increase."],["TechCommitteeMaxMembers","The maximum number of technical committee members."],["TechCommitteeMaxProposals","The maximum number of Proposlas that can be open in the technical committee at once."],["TechCommitteeMotionDuration","The maximum amount of time (in blocks) for technical committee members to vote on motions. Motions may end in fewer blocks if enough votes are cast to determine the result."],["TransactionByteFee",""],["TransactionConverter",""],["TreasuryId",""],["Version",""],["VoteLockingPeriod",""],["VotingPeriod",""],["WeightToFee",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them, but you never know…"],["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["AssetId","AssetId type"],["AssetManager",""],["Assets",""],["AssetsForceOrigin","We allow root and Chain council to execute privileged asset operations."],["AuthorFilter",""],["AuthorFilterConfig",""],["AuthorInherent",""],["AuthorMapping",""],["AuthorMappingConfig",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["BaseFee",""],["BaseFeeConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["BlockNumber","An index to a block."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CouncilCollective",""],["CouncilCollectiveConfig",""],["CrowdloanRewards",""],["CrowdloanRewardsConfig",""],["CumulusXcm",""],["Democracy",""],["DemocracyConfig",""],["DigestItem","Digest item type."],["DmpQueue",""],["EVM",""],["EVMConfig",""],["Ethereum",""],["EthereumChainId",""],["EthereumChainIdConfig",""],["EthereumConfig",""],["Executive","Executive: handles dispatch to the various pallets."],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Identity",""],["Index","Index of a transaction in the chain."],["MaintenanceMode",""],["MaintenanceModeConfig",""],["Migrations",""],["MigrationsConfig",""],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainStaking",""],["ParachainStakingConfig",""],["ParachainSystem",""],["PolkadotXcm",""],["PolkadotXcmConfig",""],["Precompiles",""],["Proxy",""],["ProxyGenesisCompanion",""],["ProxyGenesisCompanionConfig",""],["RandomnessCollectiveFlip",""],["Scheduler",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["SlowAdjustingFeeUpdate","Parameterized slow adjusting fee updated based on https://w3f-research.readthedocs.io/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism // editorconfig-checker-disable-line"],["System",""],["SystemConfig",""],["TechCommitteeCollective",""],["TechCommitteeCollectiveConfig",""],["Timestamp",""],["TransactionPayment",""],["Treasury",""],["TreasuryConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""],["XTokens",""],["XcmTransactor",""],["XcmpQueue",""]]});