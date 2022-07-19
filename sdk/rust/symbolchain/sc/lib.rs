// this is rust struct


class Amount(BaseValue):
	SIZE = 8

	def __init__(self, amount: int = 0):
		super().__init__(self.SIZE, amount, Amount)

	@classmethod
	def deserialize(cls, payload: ByteString) -> Amount:
		buffer = memoryview(payload)
		return Amount(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class BlockDuration(BaseValue):
	SIZE = 8

	def __init__(self, block_duration: int = 0):
		super().__init__(self.SIZE, block_duration, BlockDuration)

	@classmethod
	def deserialize(cls, payload: ByteString) -> BlockDuration:
		buffer = memoryview(payload)
		return BlockDuration(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class BlockFeeMultiplier(BaseValue):
	SIZE = 4

	def __init__(self, block_fee_multiplier: int = 0):
		super().__init__(self.SIZE, block_fee_multiplier, BlockFeeMultiplier)

	@classmethod
	def deserialize(cls, payload: ByteString) -> BlockFeeMultiplier:
		buffer = memoryview(payload)
		return BlockFeeMultiplier(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(4, byteorder='little', signed=False)


class Difficulty(BaseValue):
	SIZE = 8

	def __init__(self, difficulty: int = 0):
		super().__init__(self.SIZE, difficulty, Difficulty)

	@classmethod
	def deserialize(cls, payload: ByteString) -> Difficulty:
		buffer = memoryview(payload)
		return Difficulty(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class FinalizationEpoch(BaseValue):
	SIZE = 4

	def __init__(self, finalization_epoch: int = 0):
		super().__init__(self.SIZE, finalization_epoch, FinalizationEpoch)

	@classmethod
	def deserialize(cls, payload: ByteString) -> FinalizationEpoch:
		buffer = memoryview(payload)
		return FinalizationEpoch(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(4, byteorder='little', signed=False)


class FinalizationPoint(BaseValue):
	SIZE = 4

	def __init__(self, finalization_point: int = 0):
		super().__init__(self.SIZE, finalization_point, FinalizationPoint)

	@classmethod
	def deserialize(cls, payload: ByteString) -> FinalizationPoint:
		buffer = memoryview(payload)
		return FinalizationPoint(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(4, byteorder='little', signed=False)


class Height(BaseValue):
	SIZE = 8

	def __init__(self, height: int = 0):
		super().__init__(self.SIZE, height, Height)

	@classmethod
	def deserialize(cls, payload: ByteString) -> Height:
		buffer = memoryview(payload)
		return Height(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class Importance(BaseValue):
	SIZE = 8

	def __init__(self, importance: int = 0):
		super().__init__(self.SIZE, importance, Importance)

	@classmethod
	def deserialize(cls, payload: ByteString) -> Importance:
		buffer = memoryview(payload)
		return Importance(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class ImportanceHeight(BaseValue):
	SIZE = 8

	def __init__(self, importance_height: int = 0):
		super().__init__(self.SIZE, importance_height, ImportanceHeight)

	@classmethod
	def deserialize(cls, payload: ByteString) -> ImportanceHeight:
		buffer = memoryview(payload)
		return ImportanceHeight(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class UnresolvedMosaicId(BaseValue):
	SIZE = 8

	def __init__(self, unresolved_mosaic_id: int = 0):
		super().__init__(self.SIZE, unresolved_mosaic_id, UnresolvedMosaicId)

	@classmethod
	def deserialize(cls, payload: ByteString) -> UnresolvedMosaicId:
		buffer = memoryview(payload)
		return UnresolvedMosaicId(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class MosaicId(BaseValue):
	SIZE = 8

	def __init__(self, mosaic_id: int = 0):
		super().__init__(self.SIZE, mosaic_id, MosaicId)

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicId:
		buffer = memoryview(payload)
		return MosaicId(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class Timestamp(BaseValue):
	SIZE = 8

	def __init__(self, timestamp: int = 0):
		super().__init__(self.SIZE, timestamp, Timestamp)

	@classmethod
	def deserialize(cls, payload: ByteString) -> Timestamp:
		buffer = memoryview(payload)
		return Timestamp(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class UnresolvedAddress(ByteArray):
	SIZE = 24

	def __init__(self, unresolved_address: StrBytes = bytes(24)):
		super().__init__(self.SIZE, unresolved_address, UnresolvedAddress)

	@property
	def size(self) -> int:
		return 24

	@classmethod
	def deserialize(cls, payload: ByteString) -> UnresolvedAddress:
		buffer = memoryview(payload)
		return UnresolvedAddress(ArrayHelpers.get_bytes(buffer, 24))

	def serialize(self) -> bytes:
		return self.bytes


class Address(ByteArray):
	SIZE = 24

	def __init__(self, address: StrBytes = bytes(24)):
		super().__init__(self.SIZE, address, Address)

	@property
	def size(self) -> int:
		return 24

	@classmethod
	def deserialize(cls, payload: ByteString) -> Address:
		buffer = memoryview(payload)
		return Address(ArrayHelpers.get_bytes(buffer, 24))

	def serialize(self) -> bytes:
		return self.bytes


class Hash256(ByteArray):
	SIZE = 32

	def __init__(self, hash256: StrBytes = bytes(32)):
		super().__init__(self.SIZE, hash256, Hash256)

	@property
	def size(self) -> int:
		return 32

	@classmethod
	def deserialize(cls, payload: ByteString) -> Hash256:
		buffer = memoryview(payload)
		return Hash256(ArrayHelpers.get_bytes(buffer, 32))

	def serialize(self) -> bytes:
		return self.bytes


class Hash512(ByteArray):
	SIZE = 64

	def __init__(self, hash512: StrBytes = bytes(64)):
		super().__init__(self.SIZE, hash512, Hash512)

	@property
	def size(self) -> int:
		return 64

	@classmethod
	def deserialize(cls, payload: ByteString) -> Hash512:
		buffer = memoryview(payload)
		return Hash512(ArrayHelpers.get_bytes(buffer, 64))

	def serialize(self) -> bytes:
		return self.bytes


class PublicKey(ByteArray):
	SIZE = 32

	def __init__(self, public_key: StrBytes = bytes(32)):
		super().__init__(self.SIZE, public_key, PublicKey)

	@property
	def size(self) -> int:
		return 32

	@classmethod
	def deserialize(cls, payload: ByteString) -> PublicKey:
		buffer = memoryview(payload)
		return PublicKey(ArrayHelpers.get_bytes(buffer, 32))

	def serialize(self) -> bytes:
		return self.bytes


class VotingPublicKey(ByteArray):
	SIZE = 32

	def __init__(self, voting_public_key: StrBytes = bytes(32)):
		super().__init__(self.SIZE, voting_public_key, VotingPublicKey)

	@property
	def size(self) -> int:
		return 32

	@classmethod
	def deserialize(cls, payload: ByteString) -> VotingPublicKey:
		buffer = memoryview(payload)
		return VotingPublicKey(ArrayHelpers.get_bytes(buffer, 32))

	def serialize(self) -> bytes:
		return self.bytes


class Signature(ByteArray):
	SIZE = 64

	def __init__(self, signature: StrBytes = bytes(64)):
		super().__init__(self.SIZE, signature, Signature)

	@property
	def size(self) -> int:
		return 64

	@classmethod
	def deserialize(cls, payload: ByteString) -> Signature:
		buffer = memoryview(payload)
		return Signature(ArrayHelpers.get_bytes(buffer, 64))

	def serialize(self) -> bytes:
		return self.bytes


class Mosaic:
	TYPE_HINTS = {
		'mosaic_id': 'pod:MosaicId',
		'amount': 'pod:Amount'
	}

	def __init__(self):
		self._mosaic_id = MosaicId()
		self._amount = Amount()

	@property
	def mosaic_id(self) -> MosaicId:
		return self._mosaic_id

	@property
	def amount(self) -> Amount:
		return self._amount

	@mosaic_id.setter
	def mosaic_id(self, value: MosaicId):
		self._mosaic_id = value

	@amount.setter
	def amount(self, value: Amount):
		self._amount = value

	@property
	def size(self) -> int:
		size = 0
		size += self.mosaic_id.size
		size += self.amount.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Mosaic:
		buffer = memoryview(payload)
		mosaic_id = MosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]

		instance = Mosaic()
		instance._mosaic_id = mosaic_id
		instance._amount = amount
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._mosaic_id.serialize()
		buffer += self._amount.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += ')'
		return result


class UnresolvedMosaic:
	TYPE_HINTS = {
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'amount': 'pod:Amount'
	}

	def __init__(self):
		self._mosaic_id = UnresolvedMosaicId()
		self._amount = Amount()

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def amount(self) -> Amount:
		return self._amount

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@amount.setter
	def amount(self, value: Amount):
		self._amount = value

	@property
	def size(self) -> int:
		size = 0
		size += self.mosaic_id.size
		size += self.amount.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> UnresolvedMosaic:
		buffer = memoryview(payload)
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]

		instance = UnresolvedMosaic()
		instance._mosaic_id = mosaic_id
		instance._amount = amount
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._mosaic_id.serialize()
		buffer += self._amount.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += ')'
		return result


class LinkAction(Enum):
	UNLINK = 0
	LINK = 1

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> LinkAction:
		buffer = memoryview(payload)
		return LinkAction(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class NetworkType(Enum):
	MAINNET = 104
	TESTNET = 152

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> NetworkType:
		buffer = memoryview(payload)
		return NetworkType(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class TransactionType(Enum):
	ACCOUNT_KEY_LINK = 16716
	NODE_KEY_LINK = 16972
	AGGREGATE_COMPLETE = 16705
	AGGREGATE_BONDED = 16961
	VOTING_KEY_LINK = 16707
	VRF_KEY_LINK = 16963
	HASH_LOCK = 16712
	SECRET_LOCK = 16722
	SECRET_PROOF = 16978
	ACCOUNT_METADATA = 16708
	MOSAIC_METADATA = 16964
	NAMESPACE_METADATA = 17220
	MOSAIC_DEFINITION = 16717
	MOSAIC_SUPPLY_CHANGE = 16973
	MOSAIC_SUPPLY_REVOCATION = 17229
	MULTISIG_ACCOUNT_MODIFICATION = 16725
	ADDRESS_ALIAS = 16974
	MOSAIC_ALIAS = 17230
	NAMESPACE_REGISTRATION = 16718
	ACCOUNT_ADDRESS_RESTRICTION = 16720
	ACCOUNT_MOSAIC_RESTRICTION = 16976
	ACCOUNT_OPERATION_RESTRICTION = 17232
	MOSAIC_ADDRESS_RESTRICTION = 16977
	MOSAIC_GLOBAL_RESTRICTION = 16721
	TRANSFER = 16724

	@property
	def size(self) -> int:
		return 2

	@classmethod
	def deserialize(cls, payload: ByteString) -> TransactionType:
		buffer = memoryview(payload)
		return TransactionType(int.from_bytes(buffer[:2], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(2, byteorder='little', signed=False)
		return buffer


class Transaction:
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = 0
		self._network = NetworkType.MAINNET
		self._type_ = TransactionType.ACCOUNT_KEY_LINK
		self._fee = Amount()
		self._deadline = Timestamp()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Transaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]

		instance = Transaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += ')'
		return result


class EmbeddedTransaction:
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = 0
		self._network = NetworkType.MAINNET
		self._type_ = TransactionType.ACCOUNT_KEY_LINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]

		instance = EmbeddedTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += ')'
		return result


class AccountKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_KEY_LINK
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'linked_public_key': 'pod:PublicKey',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AccountKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AccountKeyLinkTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._linked_public_key = PublicKey()
		self._link_action = LinkAction.UNLINK
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def linked_public_key(self) -> PublicKey:
		return self._linked_public_key

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@linked_public_key.setter
	def linked_public_key(self, value: PublicKey):
		self._linked_public_key = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.linked_public_key.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		linked_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = AccountKeyLinkTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._linked_public_key = linked_public_key
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class EmbeddedAccountKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_KEY_LINK
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'linked_public_key': 'pod:PublicKey',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedAccountKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedAccountKeyLinkTransaction.TRANSACTION_TYPE
		self._linked_public_key = PublicKey()
		self._link_action = LinkAction.UNLINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def linked_public_key(self) -> PublicKey:
		return self._linked_public_key

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@linked_public_key.setter
	def linked_public_key(self, value: PublicKey):
		self._linked_public_key = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.linked_public_key.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedAccountKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		linked_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = EmbeddedAccountKeyLinkTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._linked_public_key = linked_public_key
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class NodeKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NODE_KEY_LINK
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'linked_public_key': 'pod:PublicKey',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = NodeKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = NodeKeyLinkTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._linked_public_key = PublicKey()
		self._link_action = LinkAction.UNLINK
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def linked_public_key(self) -> PublicKey:
		return self._linked_public_key

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@linked_public_key.setter
	def linked_public_key(self, value: PublicKey):
		self._linked_public_key = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.linked_public_key.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NodeKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		linked_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = NodeKeyLinkTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._linked_public_key = linked_public_key
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class EmbeddedNodeKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NODE_KEY_LINK
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'linked_public_key': 'pod:PublicKey',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedNodeKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedNodeKeyLinkTransaction.TRANSACTION_TYPE
		self._linked_public_key = PublicKey()
		self._link_action = LinkAction.UNLINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def linked_public_key(self) -> PublicKey:
		return self._linked_public_key

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@linked_public_key.setter
	def linked_public_key(self, value: PublicKey):
		self._linked_public_key = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.linked_public_key.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedNodeKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		linked_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = EmbeddedNodeKeyLinkTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._linked_public_key = linked_public_key
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class Cosignature:
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature'
	}

	def __init__(self):
		self._version = 0
		self._signer_public_key = PublicKey()
		self._signature = Signature()

	@property
	def version(self) -> int:
		return self._version

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@version.setter
	def version(self, value: int):
		self._version = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@property
	def size(self) -> int:
		size = 0
		size += 8
		size += self.signer_public_key.size
		size += self.signature.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Cosignature:
		buffer = memoryview(payload)
		version = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]

		instance = Cosignature()
		instance._version = version
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._version.to_bytes(8, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'version: 0x{self._version:X}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += ')'
		return result


class DetachedCosignature:
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'parent_hash': 'pod:Hash256'
	}

	def __init__(self):
		self._version = 0
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._parent_hash = Hash256()

	@property
	def version(self) -> int:
		return self._version

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def parent_hash(self) -> Hash256:
		return self._parent_hash

	@version.setter
	def version(self, value: int):
		self._version = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@parent_hash.setter
	def parent_hash(self, value: Hash256):
		self._parent_hash = value

	@property
	def size(self) -> int:
		size = 0
		size += 8
		size += self.signer_public_key.size
		size += self.signature.size
		size += self.parent_hash.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> DetachedCosignature:
		buffer = memoryview(payload)
		version = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		parent_hash = Hash256.deserialize(buffer)
		buffer = buffer[parent_hash.size:]

		instance = DetachedCosignature()
		instance._version = version
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._parent_hash = parent_hash
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._version.to_bytes(8, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature.serialize()
		buffer += self._parent_hash.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'version: 0x{self._version:X}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'parent_hash: {self._parent_hash.__str__()}, '
		result += ')'
		return result


class AggregateCompleteTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.AGGREGATE_COMPLETE
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'transactions_hash': 'pod:Hash256',
		'transactions': 'array[EmbeddedTransaction]',
		'cosignatures': 'array[Cosignature]'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AggregateCompleteTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AggregateCompleteTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._transactions_hash = Hash256()
		self._transactions = []
		self._cosignatures = []
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._aggregate_transaction_header_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def transactions_hash(self) -> Hash256:
		return self._transactions_hash

	@property
	def transactions(self) -> List[EmbeddedTransaction]:
		return self._transactions

	@property
	def cosignatures(self) -> List[Cosignature]:
		return self._cosignatures

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@transactions_hash.setter
	def transactions_hash(self, value: Hash256):
		self._transactions_hash = value

	@transactions.setter
	def transactions(self, value: List[EmbeddedTransaction]):
		self._transactions = value

	@cosignatures.setter
	def cosignatures(self, value: List[Cosignature]):
		self._cosignatures = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.transactions_hash.size
		size += 4
		size += 4
		size += ArrayHelpers.size(self.transactions, 8, skip_last_element_padding=False)
		size += ArrayHelpers.size(self.cosignatures)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AggregateCompleteTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		transactions_hash = Hash256.deserialize(buffer)
		buffer = buffer[transactions_hash.size:]
		payload_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		aggregate_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert aggregate_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({aggregate_transaction_header_reserved_1})'
		transactions = ArrayHelpers.read_variable_size_elements(buffer[:payload_size], EmbeddedTransactionFactory, 8, skip_last_element_padding=False)
		buffer = buffer[payload_size:]
		cosignatures = ArrayHelpers.read_array(buffer, Cosignature)
		buffer = buffer[ArrayHelpers.size(cosignatures):]

		instance = AggregateCompleteTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._transactions_hash = transactions_hash
		instance._transactions = transactions
		instance._cosignatures = cosignatures
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._transactions_hash.serialize()
		buffer += ArrayHelpers.size(self.transactions, 8, skip_last_element_padding=False).to_bytes(4, byteorder='little', signed=False)  # payload_size
		buffer += self._aggregate_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_variable_size_elements(self._transactions, 8, skip_last_element_padding=False)
		buffer += ArrayHelpers.write_array(self._cosignatures)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'transactions_hash: {self._transactions_hash.__str__()}, '
		result += f'transactions: {list(map(str, self._transactions))}, '
		result += f'cosignatures: {list(map(str, self._cosignatures))}, '
		result += ')'
		return result


class AggregateBondedTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.AGGREGATE_BONDED
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'transactions_hash': 'pod:Hash256',
		'transactions': 'array[EmbeddedTransaction]',
		'cosignatures': 'array[Cosignature]'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AggregateBondedTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AggregateBondedTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._transactions_hash = Hash256()
		self._transactions = []
		self._cosignatures = []
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._aggregate_transaction_header_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def transactions_hash(self) -> Hash256:
		return self._transactions_hash

	@property
	def transactions(self) -> List[EmbeddedTransaction]:
		return self._transactions

	@property
	def cosignatures(self) -> List[Cosignature]:
		return self._cosignatures

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@transactions_hash.setter
	def transactions_hash(self, value: Hash256):
		self._transactions_hash = value

	@transactions.setter
	def transactions(self, value: List[EmbeddedTransaction]):
		self._transactions = value

	@cosignatures.setter
	def cosignatures(self, value: List[Cosignature]):
		self._cosignatures = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.transactions_hash.size
		size += 4
		size += 4
		size += ArrayHelpers.size(self.transactions, 8, skip_last_element_padding=False)
		size += ArrayHelpers.size(self.cosignatures)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AggregateBondedTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		transactions_hash = Hash256.deserialize(buffer)
		buffer = buffer[transactions_hash.size:]
		payload_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		aggregate_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert aggregate_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({aggregate_transaction_header_reserved_1})'
		transactions = ArrayHelpers.read_variable_size_elements(buffer[:payload_size], EmbeddedTransactionFactory, 8, skip_last_element_padding=False)
		buffer = buffer[payload_size:]
		cosignatures = ArrayHelpers.read_array(buffer, Cosignature)
		buffer = buffer[ArrayHelpers.size(cosignatures):]

		instance = AggregateBondedTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._transactions_hash = transactions_hash
		instance._transactions = transactions
		instance._cosignatures = cosignatures
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._transactions_hash.serialize()
		buffer += ArrayHelpers.size(self.transactions, 8, skip_last_element_padding=False).to_bytes(4, byteorder='little', signed=False)  # payload_size
		buffer += self._aggregate_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_variable_size_elements(self._transactions, 8, skip_last_element_padding=False)
		buffer += ArrayHelpers.write_array(self._cosignatures)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'transactions_hash: {self._transactions_hash.__str__()}, '
		result += f'transactions: {list(map(str, self._transactions))}, '
		result += f'cosignatures: {list(map(str, self._cosignatures))}, '
		result += ')'
		return result


class VotingKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.VOTING_KEY_LINK
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'linked_public_key': 'pod:VotingPublicKey',
		'start_epoch': 'pod:FinalizationEpoch',
		'end_epoch': 'pod:FinalizationEpoch',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = VotingKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = VotingKeyLinkTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._linked_public_key = VotingPublicKey()
		self._start_epoch = FinalizationEpoch()
		self._end_epoch = FinalizationEpoch()
		self._link_action = LinkAction.UNLINK
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def linked_public_key(self) -> VotingPublicKey:
		return self._linked_public_key

	@property
	def start_epoch(self) -> FinalizationEpoch:
		return self._start_epoch

	@property
	def end_epoch(self) -> FinalizationEpoch:
		return self._end_epoch

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@linked_public_key.setter
	def linked_public_key(self, value: VotingPublicKey):
		self._linked_public_key = value

	@start_epoch.setter
	def start_epoch(self, value: FinalizationEpoch):
		self._start_epoch = value

	@end_epoch.setter
	def end_epoch(self, value: FinalizationEpoch):
		self._end_epoch = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.linked_public_key.size
		size += self.start_epoch.size
		size += self.end_epoch.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> VotingKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		linked_public_key = VotingPublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		start_epoch = FinalizationEpoch.deserialize(buffer)
		buffer = buffer[start_epoch.size:]
		end_epoch = FinalizationEpoch.deserialize(buffer)
		buffer = buffer[end_epoch.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = VotingKeyLinkTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._linked_public_key = linked_public_key
		instance._start_epoch = start_epoch
		instance._end_epoch = end_epoch
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._start_epoch.serialize()
		buffer += self._end_epoch.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'start_epoch: {self._start_epoch.__str__()}, '
		result += f'end_epoch: {self._end_epoch.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class EmbeddedVotingKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.VOTING_KEY_LINK
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'linked_public_key': 'pod:VotingPublicKey',
		'start_epoch': 'pod:FinalizationEpoch',
		'end_epoch': 'pod:FinalizationEpoch',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedVotingKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedVotingKeyLinkTransaction.TRANSACTION_TYPE
		self._linked_public_key = VotingPublicKey()
		self._start_epoch = FinalizationEpoch()
		self._end_epoch = FinalizationEpoch()
		self._link_action = LinkAction.UNLINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def linked_public_key(self) -> VotingPublicKey:
		return self._linked_public_key

	@property
	def start_epoch(self) -> FinalizationEpoch:
		return self._start_epoch

	@property
	def end_epoch(self) -> FinalizationEpoch:
		return self._end_epoch

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@linked_public_key.setter
	def linked_public_key(self, value: VotingPublicKey):
		self._linked_public_key = value

	@start_epoch.setter
	def start_epoch(self, value: FinalizationEpoch):
		self._start_epoch = value

	@end_epoch.setter
	def end_epoch(self, value: FinalizationEpoch):
		self._end_epoch = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.linked_public_key.size
		size += self.start_epoch.size
		size += self.end_epoch.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedVotingKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		linked_public_key = VotingPublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		start_epoch = FinalizationEpoch.deserialize(buffer)
		buffer = buffer[start_epoch.size:]
		end_epoch = FinalizationEpoch.deserialize(buffer)
		buffer = buffer[end_epoch.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = EmbeddedVotingKeyLinkTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._linked_public_key = linked_public_key
		instance._start_epoch = start_epoch
		instance._end_epoch = end_epoch
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._start_epoch.serialize()
		buffer += self._end_epoch.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'start_epoch: {self._start_epoch.__str__()}, '
		result += f'end_epoch: {self._end_epoch.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class VrfKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.VRF_KEY_LINK
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'linked_public_key': 'pod:PublicKey',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = VrfKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = VrfKeyLinkTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._linked_public_key = PublicKey()
		self._link_action = LinkAction.UNLINK
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def linked_public_key(self) -> PublicKey:
		return self._linked_public_key

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@linked_public_key.setter
	def linked_public_key(self, value: PublicKey):
		self._linked_public_key = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.linked_public_key.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> VrfKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		linked_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = VrfKeyLinkTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._linked_public_key = linked_public_key
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class EmbeddedVrfKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.VRF_KEY_LINK
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'linked_public_key': 'pod:PublicKey',
		'link_action': 'enum:LinkAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedVrfKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedVrfKeyLinkTransaction.TRANSACTION_TYPE
		self._linked_public_key = PublicKey()
		self._link_action = LinkAction.UNLINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def linked_public_key(self) -> PublicKey:
		return self._linked_public_key

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@linked_public_key.setter
	def linked_public_key(self, value: PublicKey):
		self._linked_public_key = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.linked_public_key.size
		size += self.link_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedVrfKeyLinkTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		linked_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[linked_public_key.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]

		instance = EmbeddedVrfKeyLinkTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._linked_public_key = linked_public_key
		instance._link_action = link_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._linked_public_key.serialize()
		buffer += self._link_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'linked_public_key: {self._linked_public_key.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += ')'
		return result


class HashLockTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.HASH_LOCK
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic': 'struct:UnresolvedMosaic',
		'duration': 'pod:BlockDuration',
		'hash': 'pod:Hash256'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = HashLockTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = HashLockTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic = UnresolvedMosaic()
		self._duration = BlockDuration()
		self._hash = Hash256()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic(self) -> UnresolvedMosaic:
		return self._mosaic

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def hash(self) -> Hash256:
		return self._hash

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic.setter
	def mosaic(self, value: UnresolvedMosaic):
		self._mosaic = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@hash.setter
	def hash(self, value: Hash256):
		self._hash = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.mosaic.size
		size += self.duration.size
		size += self.hash.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> HashLockTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic = UnresolvedMosaic.deserialize(buffer)
		buffer = buffer[mosaic.size:]
		duration = BlockDuration.deserialize(buffer)
		buffer = buffer[duration.size:]
		hash = Hash256.deserialize(buffer)
		buffer = buffer[hash.size:]

		instance = HashLockTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic = mosaic
		instance._duration = duration
		instance._hash = hash
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._mosaic.serialize()
		buffer += self._duration.serialize()
		buffer += self._hash.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += f'duration: {self._duration.__str__()}, '
		result += f'hash: {self._hash.__str__()}, '
		result += ')'
		return result


class EmbeddedHashLockTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.HASH_LOCK
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'mosaic': 'struct:UnresolvedMosaic',
		'duration': 'pod:BlockDuration',
		'hash': 'pod:Hash256'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedHashLockTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedHashLockTransaction.TRANSACTION_TYPE
		self._mosaic = UnresolvedMosaic()
		self._duration = BlockDuration()
		self._hash = Hash256()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def mosaic(self) -> UnresolvedMosaic:
		return self._mosaic

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def hash(self) -> Hash256:
		return self._hash

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@mosaic.setter
	def mosaic(self, value: UnresolvedMosaic):
		self._mosaic = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@hash.setter
	def hash(self, value: Hash256):
		self._hash = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.mosaic.size
		size += self.duration.size
		size += self.hash.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedHashLockTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		mosaic = UnresolvedMosaic.deserialize(buffer)
		buffer = buffer[mosaic.size:]
		duration = BlockDuration.deserialize(buffer)
		buffer = buffer[duration.size:]
		hash = Hash256.deserialize(buffer)
		buffer = buffer[hash.size:]

		instance = EmbeddedHashLockTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._mosaic = mosaic
		instance._duration = duration
		instance._hash = hash
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._mosaic.serialize()
		buffer += self._duration.serialize()
		buffer += self._hash.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += f'duration: {self._duration.__str__()}, '
		result += f'hash: {self._hash.__str__()}, '
		result += ')'
		return result


class LockHashAlgorithm(Enum):
	SHA3_256 = 0
	HASH_160 = 1
	HASH_256 = 2

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> LockHashAlgorithm:
		buffer = memoryview(payload)
		return LockHashAlgorithm(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class SecretLockTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.SECRET_LOCK
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:UnresolvedAddress',
		'secret': 'pod:Hash256',
		'mosaic': 'struct:UnresolvedMosaic',
		'duration': 'pod:BlockDuration',
		'hash_algorithm': 'enum:LockHashAlgorithm'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = SecretLockTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = SecretLockTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = UnresolvedAddress()
		self._secret = Hash256()
		self._mosaic = UnresolvedMosaic()
		self._duration = BlockDuration()
		self._hash_algorithm = LockHashAlgorithm.SHA3_256
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> UnresolvedAddress:
		return self._recipient_address

	@property
	def secret(self) -> Hash256:
		return self._secret

	@property
	def mosaic(self) -> UnresolvedMosaic:
		return self._mosaic

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def hash_algorithm(self) -> LockHashAlgorithm:
		return self._hash_algorithm

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: UnresolvedAddress):
		self._recipient_address = value

	@secret.setter
	def secret(self, value: Hash256):
		self._secret = value

	@mosaic.setter
	def mosaic(self, value: UnresolvedMosaic):
		self._mosaic = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@hash_algorithm.setter
	def hash_algorithm(self, value: LockHashAlgorithm):
		self._hash_algorithm = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.recipient_address.size
		size += self.secret.size
		size += self.mosaic.size
		size += self.duration.size
		size += self.hash_algorithm.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> SecretLockTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		secret = Hash256.deserialize(buffer)
		buffer = buffer[secret.size:]
		mosaic = UnresolvedMosaic.deserialize(buffer)
		buffer = buffer[mosaic.size:]
		duration = BlockDuration.deserialize(buffer)
		buffer = buffer[duration.size:]
		hash_algorithm = LockHashAlgorithm.deserialize(buffer)
		buffer = buffer[hash_algorithm.size:]

		instance = SecretLockTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._secret = secret
		instance._mosaic = mosaic
		instance._duration = duration
		instance._hash_algorithm = hash_algorithm
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address.serialize()
		buffer += self._secret.serialize()
		buffer += self._mosaic.serialize()
		buffer += self._duration.serialize()
		buffer += self._hash_algorithm.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'secret: {self._secret.__str__()}, '
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += f'duration: {self._duration.__str__()}, '
		result += f'hash_algorithm: {self._hash_algorithm.__str__()}, '
		result += ')'
		return result


class EmbeddedSecretLockTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.SECRET_LOCK
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'recipient_address': 'pod:UnresolvedAddress',
		'secret': 'pod:Hash256',
		'mosaic': 'struct:UnresolvedMosaic',
		'duration': 'pod:BlockDuration',
		'hash_algorithm': 'enum:LockHashAlgorithm'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedSecretLockTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedSecretLockTransaction.TRANSACTION_TYPE
		self._recipient_address = UnresolvedAddress()
		self._secret = Hash256()
		self._mosaic = UnresolvedMosaic()
		self._duration = BlockDuration()
		self._hash_algorithm = LockHashAlgorithm.SHA3_256
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def recipient_address(self) -> UnresolvedAddress:
		return self._recipient_address

	@property
	def secret(self) -> Hash256:
		return self._secret

	@property
	def mosaic(self) -> UnresolvedMosaic:
		return self._mosaic

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def hash_algorithm(self) -> LockHashAlgorithm:
		return self._hash_algorithm

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@recipient_address.setter
	def recipient_address(self, value: UnresolvedAddress):
		self._recipient_address = value

	@secret.setter
	def secret(self, value: Hash256):
		self._secret = value

	@mosaic.setter
	def mosaic(self, value: UnresolvedMosaic):
		self._mosaic = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@hash_algorithm.setter
	def hash_algorithm(self, value: LockHashAlgorithm):
		self._hash_algorithm = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.recipient_address.size
		size += self.secret.size
		size += self.mosaic.size
		size += self.duration.size
		size += self.hash_algorithm.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedSecretLockTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		recipient_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		secret = Hash256.deserialize(buffer)
		buffer = buffer[secret.size:]
		mosaic = UnresolvedMosaic.deserialize(buffer)
		buffer = buffer[mosaic.size:]
		duration = BlockDuration.deserialize(buffer)
		buffer = buffer[duration.size:]
		hash_algorithm = LockHashAlgorithm.deserialize(buffer)
		buffer = buffer[hash_algorithm.size:]

		instance = EmbeddedSecretLockTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._recipient_address = recipient_address
		instance._secret = secret
		instance._mosaic = mosaic
		instance._duration = duration
		instance._hash_algorithm = hash_algorithm
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._recipient_address.serialize()
		buffer += self._secret.serialize()
		buffer += self._mosaic.serialize()
		buffer += self._duration.serialize()
		buffer += self._hash_algorithm.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'secret: {self._secret.__str__()}, '
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += f'duration: {self._duration.__str__()}, '
		result += f'hash_algorithm: {self._hash_algorithm.__str__()}, '
		result += ')'
		return result


class SecretProofTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.SECRET_PROOF
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:UnresolvedAddress',
		'secret': 'pod:Hash256',
		'hash_algorithm': 'enum:LockHashAlgorithm',
		'proof': 'bytes_array'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = SecretProofTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = SecretProofTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = UnresolvedAddress()
		self._secret = Hash256()
		self._hash_algorithm = LockHashAlgorithm.SHA3_256
		self._proof = bytes()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> UnresolvedAddress:
		return self._recipient_address

	@property
	def secret(self) -> Hash256:
		return self._secret

	@property
	def hash_algorithm(self) -> LockHashAlgorithm:
		return self._hash_algorithm

	@property
	def proof(self) -> bytes:
		return self._proof

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: UnresolvedAddress):
		self._recipient_address = value

	@secret.setter
	def secret(self, value: Hash256):
		self._secret = value

	@hash_algorithm.setter
	def hash_algorithm(self, value: LockHashAlgorithm):
		self._hash_algorithm = value

	@proof.setter
	def proof(self, value: bytes):
		self._proof = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.recipient_address.size
		size += self.secret.size
		size += 2
		size += self.hash_algorithm.size
		size += len(self._proof)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> SecretProofTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		secret = Hash256.deserialize(buffer)
		buffer = buffer[secret.size:]
		proof_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		hash_algorithm = LockHashAlgorithm.deserialize(buffer)
		buffer = buffer[hash_algorithm.size:]
		proof = ArrayHelpers.get_bytes(buffer, proof_size)
		buffer = buffer[proof_size:]

		instance = SecretProofTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._secret = secret
		instance._hash_algorithm = hash_algorithm
		instance._proof = proof
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address.serialize()
		buffer += self._secret.serialize()
		buffer += len(self._proof).to_bytes(2, byteorder='little', signed=False)  # proof_size
		buffer += self._hash_algorithm.serialize()
		buffer += self._proof
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'secret: {self._secret.__str__()}, '
		result += f'hash_algorithm: {self._hash_algorithm.__str__()}, '
		result += f'proof: {hexlify(self._proof).decode("utf8")}, '
		result += ')'
		return result


class EmbeddedSecretProofTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.SECRET_PROOF
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'recipient_address': 'pod:UnresolvedAddress',
		'secret': 'pod:Hash256',
		'hash_algorithm': 'enum:LockHashAlgorithm',
		'proof': 'bytes_array'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedSecretProofTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedSecretProofTransaction.TRANSACTION_TYPE
		self._recipient_address = UnresolvedAddress()
		self._secret = Hash256()
		self._hash_algorithm = LockHashAlgorithm.SHA3_256
		self._proof = bytes()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def recipient_address(self) -> UnresolvedAddress:
		return self._recipient_address

	@property
	def secret(self) -> Hash256:
		return self._secret

	@property
	def hash_algorithm(self) -> LockHashAlgorithm:
		return self._hash_algorithm

	@property
	def proof(self) -> bytes:
		return self._proof

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@recipient_address.setter
	def recipient_address(self, value: UnresolvedAddress):
		self._recipient_address = value

	@secret.setter
	def secret(self, value: Hash256):
		self._secret = value

	@hash_algorithm.setter
	def hash_algorithm(self, value: LockHashAlgorithm):
		self._hash_algorithm = value

	@proof.setter
	def proof(self, value: bytes):
		self._proof = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.recipient_address.size
		size += self.secret.size
		size += 2
		size += self.hash_algorithm.size
		size += len(self._proof)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedSecretProofTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		recipient_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		secret = Hash256.deserialize(buffer)
		buffer = buffer[secret.size:]
		proof_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		hash_algorithm = LockHashAlgorithm.deserialize(buffer)
		buffer = buffer[hash_algorithm.size:]
		proof = ArrayHelpers.get_bytes(buffer, proof_size)
		buffer = buffer[proof_size:]

		instance = EmbeddedSecretProofTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._recipient_address = recipient_address
		instance._secret = secret
		instance._hash_algorithm = hash_algorithm
		instance._proof = proof
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._recipient_address.serialize()
		buffer += self._secret.serialize()
		buffer += len(self._proof).to_bytes(2, byteorder='little', signed=False)  # proof_size
		buffer += self._hash_algorithm.serialize()
		buffer += self._proof
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'secret: {self._secret.__str__()}, '
		result += f'hash_algorithm: {self._hash_algorithm.__str__()}, '
		result += f'proof: {hexlify(self._proof).decode("utf8")}, '
		result += ')'
		return result


class AccountMetadataTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_METADATA
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'target_address': 'pod:UnresolvedAddress',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AccountMetadataTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AccountMetadataTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._target_address = UnresolvedAddress()
		self._scoped_metadata_key = 0
		self._value_size_delta = 0
		self._value = bytes()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@property
	def scoped_metadata_key(self) -> int:
		return self._scoped_metadata_key

	@property
	def value_size_delta(self) -> int:
		return self._value_size_delta

	@property
	def value(self) -> bytes:
		return self._value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@scoped_metadata_key.setter
	def scoped_metadata_key(self, value: int):
		self._scoped_metadata_key = value

	@value_size_delta.setter
	def value_size_delta(self, value: int):
		self._value_size_delta = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.target_address.size
		size += 8
		size += 2
		size += 2
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountMetadataTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]
		scoped_metadata_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		value_size_delta = int.from_bytes(buffer[:2], byteorder='little', signed=True)
		buffer = buffer[2:]
		value_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = AccountMetadataTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._target_address = target_address
		instance._scoped_metadata_key = scoped_metadata_key
		instance._value_size_delta = value_size_delta
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._target_address.serialize()
		buffer += self._scoped_metadata_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._value_size_delta.to_bytes(2, byteorder='little', signed=True)
		buffer += len(self._value).to_bytes(2, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += f'scoped_metadata_key: 0x{self._scoped_metadata_key:X}, '
		result += f'value_size_delta: 0x{self._value_size_delta:X}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class EmbeddedAccountMetadataTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_METADATA
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'target_address': 'pod:UnresolvedAddress',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedAccountMetadataTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedAccountMetadataTransaction.TRANSACTION_TYPE
		self._target_address = UnresolvedAddress()
		self._scoped_metadata_key = 0
		self._value_size_delta = 0
		self._value = bytes()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@property
	def scoped_metadata_key(self) -> int:
		return self._scoped_metadata_key

	@property
	def value_size_delta(self) -> int:
		return self._value_size_delta

	@property
	def value(self) -> bytes:
		return self._value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@scoped_metadata_key.setter
	def scoped_metadata_key(self, value: int):
		self._scoped_metadata_key = value

	@value_size_delta.setter
	def value_size_delta(self, value: int):
		self._value_size_delta = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.target_address.size
		size += 8
		size += 2
		size += 2
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedAccountMetadataTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]
		scoped_metadata_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		value_size_delta = int.from_bytes(buffer[:2], byteorder='little', signed=True)
		buffer = buffer[2:]
		value_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = EmbeddedAccountMetadataTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._target_address = target_address
		instance._scoped_metadata_key = scoped_metadata_key
		instance._value_size_delta = value_size_delta
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._target_address.serialize()
		buffer += self._scoped_metadata_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._value_size_delta.to_bytes(2, byteorder='little', signed=True)
		buffer += len(self._value).to_bytes(2, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += f'scoped_metadata_key: 0x{self._scoped_metadata_key:X}, '
		result += f'value_size_delta: 0x{self._value_size_delta:X}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class MosaicMetadataTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_METADATA
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'target_address': 'pod:UnresolvedAddress',
		'target_mosaic_id': 'pod:UnresolvedMosaicId',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicMetadataTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicMetadataTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._target_address = UnresolvedAddress()
		self._scoped_metadata_key = 0
		self._target_mosaic_id = UnresolvedMosaicId()
		self._value_size_delta = 0
		self._value = bytes()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@property
	def scoped_metadata_key(self) -> int:
		return self._scoped_metadata_key

	@property
	def target_mosaic_id(self) -> UnresolvedMosaicId:
		return self._target_mosaic_id

	@property
	def value_size_delta(self) -> int:
		return self._value_size_delta

	@property
	def value(self) -> bytes:
		return self._value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@scoped_metadata_key.setter
	def scoped_metadata_key(self, value: int):
		self._scoped_metadata_key = value

	@target_mosaic_id.setter
	def target_mosaic_id(self, value: UnresolvedMosaicId):
		self._target_mosaic_id = value

	@value_size_delta.setter
	def value_size_delta(self, value: int):
		self._value_size_delta = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.target_address.size
		size += 8
		size += self.target_mosaic_id.size
		size += 2
		size += 2
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicMetadataTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]
		scoped_metadata_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		target_mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[target_mosaic_id.size:]
		value_size_delta = int.from_bytes(buffer[:2], byteorder='little', signed=True)
		buffer = buffer[2:]
		value_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = MosaicMetadataTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._target_address = target_address
		instance._scoped_metadata_key = scoped_metadata_key
		instance._target_mosaic_id = target_mosaic_id
		instance._value_size_delta = value_size_delta
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._target_address.serialize()
		buffer += self._scoped_metadata_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._target_mosaic_id.serialize()
		buffer += self._value_size_delta.to_bytes(2, byteorder='little', signed=True)
		buffer += len(self._value).to_bytes(2, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += f'scoped_metadata_key: 0x{self._scoped_metadata_key:X}, '
		result += f'target_mosaic_id: {self._target_mosaic_id.__str__()}, '
		result += f'value_size_delta: 0x{self._value_size_delta:X}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class EmbeddedMosaicMetadataTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_METADATA
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'target_address': 'pod:UnresolvedAddress',
		'target_mosaic_id': 'pod:UnresolvedMosaicId',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicMetadataTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicMetadataTransaction.TRANSACTION_TYPE
		self._target_address = UnresolvedAddress()
		self._scoped_metadata_key = 0
		self._target_mosaic_id = UnresolvedMosaicId()
		self._value_size_delta = 0
		self._value = bytes()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@property
	def scoped_metadata_key(self) -> int:
		return self._scoped_metadata_key

	@property
	def target_mosaic_id(self) -> UnresolvedMosaicId:
		return self._target_mosaic_id

	@property
	def value_size_delta(self) -> int:
		return self._value_size_delta

	@property
	def value(self) -> bytes:
		return self._value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@scoped_metadata_key.setter
	def scoped_metadata_key(self, value: int):
		self._scoped_metadata_key = value

	@target_mosaic_id.setter
	def target_mosaic_id(self, value: UnresolvedMosaicId):
		self._target_mosaic_id = value

	@value_size_delta.setter
	def value_size_delta(self, value: int):
		self._value_size_delta = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.target_address.size
		size += 8
		size += self.target_mosaic_id.size
		size += 2
		size += 2
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicMetadataTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]
		scoped_metadata_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		target_mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[target_mosaic_id.size:]
		value_size_delta = int.from_bytes(buffer[:2], byteorder='little', signed=True)
		buffer = buffer[2:]
		value_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = EmbeddedMosaicMetadataTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._target_address = target_address
		instance._scoped_metadata_key = scoped_metadata_key
		instance._target_mosaic_id = target_mosaic_id
		instance._value_size_delta = value_size_delta
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._target_address.serialize()
		buffer += self._scoped_metadata_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._target_mosaic_id.serialize()
		buffer += self._value_size_delta.to_bytes(2, byteorder='little', signed=True)
		buffer += len(self._value).to_bytes(2, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += f'scoped_metadata_key: 0x{self._scoped_metadata_key:X}, '
		result += f'target_mosaic_id: {self._target_mosaic_id.__str__()}, '
		result += f'value_size_delta: 0x{self._value_size_delta:X}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class NamespaceId(BaseValue):
	SIZE = 8

	def __init__(self, namespace_id: int = 0):
		super().__init__(self.SIZE, namespace_id, NamespaceId)

	@classmethod
	def deserialize(cls, payload: ByteString) -> NamespaceId:
		buffer = memoryview(payload)
		return NamespaceId(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class NamespaceRegistrationType(Enum):
	ROOT = 0
	CHILD = 1

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> NamespaceRegistrationType:
		buffer = memoryview(payload)
		return NamespaceRegistrationType(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class AliasAction(Enum):
	UNLINK = 0
	LINK = 1

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> AliasAction:
		buffer = memoryview(payload)
		return AliasAction(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class NamespaceMetadataTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NAMESPACE_METADATA
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'target_address': 'pod:UnresolvedAddress',
		'target_namespace_id': 'pod:NamespaceId',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = NamespaceMetadataTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = NamespaceMetadataTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._target_address = UnresolvedAddress()
		self._scoped_metadata_key = 0
		self._target_namespace_id = NamespaceId()
		self._value_size_delta = 0
		self._value = bytes()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@property
	def scoped_metadata_key(self) -> int:
		return self._scoped_metadata_key

	@property
	def target_namespace_id(self) -> NamespaceId:
		return self._target_namespace_id

	@property
	def value_size_delta(self) -> int:
		return self._value_size_delta

	@property
	def value(self) -> bytes:
		return self._value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@scoped_metadata_key.setter
	def scoped_metadata_key(self, value: int):
		self._scoped_metadata_key = value

	@target_namespace_id.setter
	def target_namespace_id(self, value: NamespaceId):
		self._target_namespace_id = value

	@value_size_delta.setter
	def value_size_delta(self, value: int):
		self._value_size_delta = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.target_address.size
		size += 8
		size += self.target_namespace_id.size
		size += 2
		size += 2
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NamespaceMetadataTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]
		scoped_metadata_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		target_namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[target_namespace_id.size:]
		value_size_delta = int.from_bytes(buffer[:2], byteorder='little', signed=True)
		buffer = buffer[2:]
		value_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = NamespaceMetadataTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._target_address = target_address
		instance._scoped_metadata_key = scoped_metadata_key
		instance._target_namespace_id = target_namespace_id
		instance._value_size_delta = value_size_delta
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._target_address.serialize()
		buffer += self._scoped_metadata_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._target_namespace_id.serialize()
		buffer += self._value_size_delta.to_bytes(2, byteorder='little', signed=True)
		buffer += len(self._value).to_bytes(2, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += f'scoped_metadata_key: 0x{self._scoped_metadata_key:X}, '
		result += f'target_namespace_id: {self._target_namespace_id.__str__()}, '
		result += f'value_size_delta: 0x{self._value_size_delta:X}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class EmbeddedNamespaceMetadataTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NAMESPACE_METADATA
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'target_address': 'pod:UnresolvedAddress',
		'target_namespace_id': 'pod:NamespaceId',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedNamespaceMetadataTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedNamespaceMetadataTransaction.TRANSACTION_TYPE
		self._target_address = UnresolvedAddress()
		self._scoped_metadata_key = 0
		self._target_namespace_id = NamespaceId()
		self._value_size_delta = 0
		self._value = bytes()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@property
	def scoped_metadata_key(self) -> int:
		return self._scoped_metadata_key

	@property
	def target_namespace_id(self) -> NamespaceId:
		return self._target_namespace_id

	@property
	def value_size_delta(self) -> int:
		return self._value_size_delta

	@property
	def value(self) -> bytes:
		return self._value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@scoped_metadata_key.setter
	def scoped_metadata_key(self, value: int):
		self._scoped_metadata_key = value

	@target_namespace_id.setter
	def target_namespace_id(self, value: NamespaceId):
		self._target_namespace_id = value

	@value_size_delta.setter
	def value_size_delta(self, value: int):
		self._value_size_delta = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.target_address.size
		size += 8
		size += self.target_namespace_id.size
		size += 2
		size += 2
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedNamespaceMetadataTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]
		scoped_metadata_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		target_namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[target_namespace_id.size:]
		value_size_delta = int.from_bytes(buffer[:2], byteorder='little', signed=True)
		buffer = buffer[2:]
		value_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = EmbeddedNamespaceMetadataTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._target_address = target_address
		instance._scoped_metadata_key = scoped_metadata_key
		instance._target_namespace_id = target_namespace_id
		instance._value_size_delta = value_size_delta
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._target_address.serialize()
		buffer += self._scoped_metadata_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._target_namespace_id.serialize()
		buffer += self._value_size_delta.to_bytes(2, byteorder='little', signed=True)
		buffer += len(self._value).to_bytes(2, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += f'scoped_metadata_key: 0x{self._scoped_metadata_key:X}, '
		result += f'target_namespace_id: {self._target_namespace_id.__str__()}, '
		result += f'value_size_delta: 0x{self._value_size_delta:X}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class MosaicNonce(BaseValue):
	SIZE = 4

	def __init__(self, mosaic_nonce: int = 0):
		super().__init__(self.SIZE, mosaic_nonce, MosaicNonce)

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicNonce:
		buffer = memoryview(payload)
		return MosaicNonce(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(4, byteorder='little', signed=False)


class MosaicFlags(Flag):
	NONE = 0
	SUPPLY_MUTABLE = 1
	TRANSFERABLE = 2
	RESTRICTABLE = 4
	REVOKABLE = 8

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicFlags:
		buffer = memoryview(payload)
		return MosaicFlags(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class MosaicSupplyChangeAction(Enum):
	DECREASE = 0
	INCREASE = 1

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicSupplyChangeAction:
		buffer = memoryview(payload)
		return MosaicSupplyChangeAction(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class MosaicDefinitionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_DEFINITION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'id': 'pod:MosaicId',
		'duration': 'pod:BlockDuration',
		'nonce': 'pod:MosaicNonce',
		'flags': 'enum:MosaicFlags'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicDefinitionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicDefinitionTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._id = MosaicId()
		self._duration = BlockDuration()
		self._nonce = MosaicNonce()
		self._flags = MosaicFlags.NONE
		self._divisibility = 0
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def id(self) -> MosaicId:
		return self._id

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def nonce(self) -> MosaicNonce:
		return self._nonce

	@property
	def flags(self) -> MosaicFlags:
		return self._flags

	@property
	def divisibility(self) -> int:
		return self._divisibility

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@id.setter
	def id(self, value: MosaicId):
		self._id = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@nonce.setter
	def nonce(self, value: MosaicNonce):
		self._nonce = value

	@flags.setter
	def flags(self, value: MosaicFlags):
		self._flags = value

	@divisibility.setter
	def divisibility(self, value: int):
		self._divisibility = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.id.size
		size += self.duration.size
		size += self.nonce.size
		size += self.flags.size
		size += 1
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicDefinitionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		id = MosaicId.deserialize(buffer)
		buffer = buffer[id.size:]
		duration = BlockDuration.deserialize(buffer)
		buffer = buffer[duration.size:]
		nonce = MosaicNonce.deserialize(buffer)
		buffer = buffer[nonce.size:]
		flags = MosaicFlags.deserialize(buffer)
		buffer = buffer[flags.size:]
		divisibility = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]

		instance = MosaicDefinitionTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._id = id
		instance._duration = duration
		instance._nonce = nonce
		instance._flags = flags
		instance._divisibility = divisibility
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._id.serialize()
		buffer += self._duration.serialize()
		buffer += self._nonce.serialize()
		buffer += self._flags.serialize()
		buffer += self._divisibility.to_bytes(1, byteorder='little', signed=False)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'id: {self._id.__str__()}, '
		result += f'duration: {self._duration.__str__()}, '
		result += f'nonce: {self._nonce.__str__()}, '
		result += f'flags: {self._flags.__str__()}, '
		result += f'divisibility: 0x{self._divisibility:X}, '
		result += ')'
		return result


class EmbeddedMosaicDefinitionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_DEFINITION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'id': 'pod:MosaicId',
		'duration': 'pod:BlockDuration',
		'nonce': 'pod:MosaicNonce',
		'flags': 'enum:MosaicFlags'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicDefinitionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicDefinitionTransaction.TRANSACTION_TYPE
		self._id = MosaicId()
		self._duration = BlockDuration()
		self._nonce = MosaicNonce()
		self._flags = MosaicFlags.NONE
		self._divisibility = 0
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def id(self) -> MosaicId:
		return self._id

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def nonce(self) -> MosaicNonce:
		return self._nonce

	@property
	def flags(self) -> MosaicFlags:
		return self._flags

	@property
	def divisibility(self) -> int:
		return self._divisibility

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@id.setter
	def id(self, value: MosaicId):
		self._id = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@nonce.setter
	def nonce(self, value: MosaicNonce):
		self._nonce = value

	@flags.setter
	def flags(self, value: MosaicFlags):
		self._flags = value

	@divisibility.setter
	def divisibility(self, value: int):
		self._divisibility = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.id.size
		size += self.duration.size
		size += self.nonce.size
		size += self.flags.size
		size += 1
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicDefinitionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		id = MosaicId.deserialize(buffer)
		buffer = buffer[id.size:]
		duration = BlockDuration.deserialize(buffer)
		buffer = buffer[duration.size:]
		nonce = MosaicNonce.deserialize(buffer)
		buffer = buffer[nonce.size:]
		flags = MosaicFlags.deserialize(buffer)
		buffer = buffer[flags.size:]
		divisibility = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]

		instance = EmbeddedMosaicDefinitionTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._id = id
		instance._duration = duration
		instance._nonce = nonce
		instance._flags = flags
		instance._divisibility = divisibility
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._id.serialize()
		buffer += self._duration.serialize()
		buffer += self._nonce.serialize()
		buffer += self._flags.serialize()
		buffer += self._divisibility.to_bytes(1, byteorder='little', signed=False)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'id: {self._id.__str__()}, '
		result += f'duration: {self._duration.__str__()}, '
		result += f'nonce: {self._nonce.__str__()}, '
		result += f'flags: {self._flags.__str__()}, '
		result += f'divisibility: 0x{self._divisibility:X}, '
		result += ')'
		return result


class MosaicSupplyChangeTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_SUPPLY_CHANGE
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'delta': 'pod:Amount',
		'action': 'enum:MosaicSupplyChangeAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicSupplyChangeTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicSupplyChangeTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_id = UnresolvedMosaicId()
		self._delta = Amount()
		self._action = MosaicSupplyChangeAction.DECREASE
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def delta(self) -> Amount:
		return self._delta

	@property
	def action(self) -> MosaicSupplyChangeAction:
		return self._action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@delta.setter
	def delta(self, value: Amount):
		self._delta = value

	@action.setter
	def action(self, value: MosaicSupplyChangeAction):
		self._action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.mosaic_id.size
		size += self.delta.size
		size += self.action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicSupplyChangeTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		delta = Amount.deserialize(buffer)
		buffer = buffer[delta.size:]
		action = MosaicSupplyChangeAction.deserialize(buffer)
		buffer = buffer[action.size:]

		instance = MosaicSupplyChangeTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_id = mosaic_id
		instance._delta = delta
		instance._action = action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._delta.serialize()
		buffer += self._action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'delta: {self._delta.__str__()}, '
		result += f'action: {self._action.__str__()}, '
		result += ')'
		return result


class EmbeddedMosaicSupplyChangeTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_SUPPLY_CHANGE
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'delta': 'pod:Amount',
		'action': 'enum:MosaicSupplyChangeAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicSupplyChangeTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicSupplyChangeTransaction.TRANSACTION_TYPE
		self._mosaic_id = UnresolvedMosaicId()
		self._delta = Amount()
		self._action = MosaicSupplyChangeAction.DECREASE
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def delta(self) -> Amount:
		return self._delta

	@property
	def action(self) -> MosaicSupplyChangeAction:
		return self._action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@delta.setter
	def delta(self, value: Amount):
		self._delta = value

	@action.setter
	def action(self, value: MosaicSupplyChangeAction):
		self._action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.mosaic_id.size
		size += self.delta.size
		size += self.action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicSupplyChangeTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		delta = Amount.deserialize(buffer)
		buffer = buffer[delta.size:]
		action = MosaicSupplyChangeAction.deserialize(buffer)
		buffer = buffer[action.size:]

		instance = EmbeddedMosaicSupplyChangeTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._mosaic_id = mosaic_id
		instance._delta = delta
		instance._action = action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._delta.serialize()
		buffer += self._action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'delta: {self._delta.__str__()}, '
		result += f'action: {self._action.__str__()}, '
		result += ')'
		return result


class MosaicSupplyRevocationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_SUPPLY_REVOCATION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'source_address': 'pod:UnresolvedAddress',
		'mosaic': 'struct:UnresolvedMosaic'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicSupplyRevocationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicSupplyRevocationTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._source_address = UnresolvedAddress()
		self._mosaic = UnresolvedMosaic()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def source_address(self) -> UnresolvedAddress:
		return self._source_address

	@property
	def mosaic(self) -> UnresolvedMosaic:
		return self._mosaic

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@source_address.setter
	def source_address(self, value: UnresolvedAddress):
		self._source_address = value

	@mosaic.setter
	def mosaic(self, value: UnresolvedMosaic):
		self._mosaic = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.source_address.size
		size += self.mosaic.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicSupplyRevocationTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		source_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[source_address.size:]
		mosaic = UnresolvedMosaic.deserialize(buffer)
		buffer = buffer[mosaic.size:]

		instance = MosaicSupplyRevocationTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._source_address = source_address
		instance._mosaic = mosaic
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._source_address.serialize()
		buffer += self._mosaic.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'source_address: {self._source_address.__str__()}, '
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += ')'
		return result


class EmbeddedMosaicSupplyRevocationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_SUPPLY_REVOCATION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'source_address': 'pod:UnresolvedAddress',
		'mosaic': 'struct:UnresolvedMosaic'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicSupplyRevocationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicSupplyRevocationTransaction.TRANSACTION_TYPE
		self._source_address = UnresolvedAddress()
		self._mosaic = UnresolvedMosaic()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def source_address(self) -> UnresolvedAddress:
		return self._source_address

	@property
	def mosaic(self) -> UnresolvedMosaic:
		return self._mosaic

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@source_address.setter
	def source_address(self, value: UnresolvedAddress):
		self._source_address = value

	@mosaic.setter
	def mosaic(self, value: UnresolvedMosaic):
		self._mosaic = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.source_address.size
		size += self.mosaic.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicSupplyRevocationTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		source_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[source_address.size:]
		mosaic = UnresolvedMosaic.deserialize(buffer)
		buffer = buffer[mosaic.size:]

		instance = EmbeddedMosaicSupplyRevocationTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._source_address = source_address
		instance._mosaic = mosaic
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._source_address.serialize()
		buffer += self._mosaic.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'source_address: {self._source_address.__str__()}, '
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += ')'
		return result


class MultisigAccountModificationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_ACCOUNT_MODIFICATION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'address_additions': 'array[UnresolvedAddress]',
		'address_deletions': 'array[UnresolvedAddress]'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MultisigAccountModificationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MultisigAccountModificationTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._min_removal_delta = 0
		self._min_approval_delta = 0
		self._address_additions = []
		self._address_deletions = []
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._multisig_account_modification_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def min_removal_delta(self) -> int:
		return self._min_removal_delta

	@property
	def min_approval_delta(self) -> int:
		return self._min_approval_delta

	@property
	def address_additions(self) -> List[UnresolvedAddress]:
		return self._address_additions

	@property
	def address_deletions(self) -> List[UnresolvedAddress]:
		return self._address_deletions

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@min_removal_delta.setter
	def min_removal_delta(self, value: int):
		self._min_removal_delta = value

	@min_approval_delta.setter
	def min_approval_delta(self, value: int):
		self._min_approval_delta = value

	@address_additions.setter
	def address_additions(self, value: List[UnresolvedAddress]):
		self._address_additions = value

	@address_deletions.setter
	def address_deletions(self, value: List[UnresolvedAddress]):
		self._address_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += 1
		size += 1
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.address_additions)
		size += ArrayHelpers.size(self.address_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MultisigAccountModificationTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		min_removal_delta = int.from_bytes(buffer[:1], byteorder='little', signed=True)
		buffer = buffer[1:]
		min_approval_delta = int.from_bytes(buffer[:1], byteorder='little', signed=True)
		buffer = buffer[1:]
		address_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		address_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		multisig_account_modification_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert multisig_account_modification_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({multisig_account_modification_transaction_body_reserved_1})'
		address_additions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, address_additions_count)
		buffer = buffer[ArrayHelpers.size(address_additions):]
		address_deletions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, address_deletions_count)
		buffer = buffer[ArrayHelpers.size(address_deletions):]

		instance = MultisigAccountModificationTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._min_removal_delta = min_removal_delta
		instance._min_approval_delta = min_approval_delta
		instance._address_additions = address_additions
		instance._address_deletions = address_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._min_removal_delta.to_bytes(1, byteorder='little', signed=True)
		buffer += self._min_approval_delta.to_bytes(1, byteorder='little', signed=True)
		buffer += len(self._address_additions).to_bytes(1, byteorder='little', signed=False)  # address_additions_count
		buffer += len(self._address_deletions).to_bytes(1, byteorder='little', signed=False)  # address_deletions_count
		buffer += self._multisig_account_modification_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._address_additions)
		buffer += ArrayHelpers.write_array(self._address_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'min_removal_delta: 0x{self._min_removal_delta:X}, '
		result += f'min_approval_delta: 0x{self._min_approval_delta:X}, '
		result += f'address_additions: {list(map(str, self._address_additions))}, '
		result += f'address_deletions: {list(map(str, self._address_deletions))}, '
		result += ')'
		return result


class EmbeddedMultisigAccountModificationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_ACCOUNT_MODIFICATION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'address_additions': 'array[UnresolvedAddress]',
		'address_deletions': 'array[UnresolvedAddress]'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMultisigAccountModificationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMultisigAccountModificationTransaction.TRANSACTION_TYPE
		self._min_removal_delta = 0
		self._min_approval_delta = 0
		self._address_additions = []
		self._address_deletions = []
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._multisig_account_modification_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def min_removal_delta(self) -> int:
		return self._min_removal_delta

	@property
	def min_approval_delta(self) -> int:
		return self._min_approval_delta

	@property
	def address_additions(self) -> List[UnresolvedAddress]:
		return self._address_additions

	@property
	def address_deletions(self) -> List[UnresolvedAddress]:
		return self._address_deletions

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@min_removal_delta.setter
	def min_removal_delta(self, value: int):
		self._min_removal_delta = value

	@min_approval_delta.setter
	def min_approval_delta(self, value: int):
		self._min_approval_delta = value

	@address_additions.setter
	def address_additions(self, value: List[UnresolvedAddress]):
		self._address_additions = value

	@address_deletions.setter
	def address_deletions(self, value: List[UnresolvedAddress]):
		self._address_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += 1
		size += 1
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.address_additions)
		size += ArrayHelpers.size(self.address_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMultisigAccountModificationTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		min_removal_delta = int.from_bytes(buffer[:1], byteorder='little', signed=True)
		buffer = buffer[1:]
		min_approval_delta = int.from_bytes(buffer[:1], byteorder='little', signed=True)
		buffer = buffer[1:]
		address_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		address_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		multisig_account_modification_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert multisig_account_modification_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({multisig_account_modification_transaction_body_reserved_1})'
		address_additions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, address_additions_count)
		buffer = buffer[ArrayHelpers.size(address_additions):]
		address_deletions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, address_deletions_count)
		buffer = buffer[ArrayHelpers.size(address_deletions):]

		instance = EmbeddedMultisigAccountModificationTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._min_removal_delta = min_removal_delta
		instance._min_approval_delta = min_approval_delta
		instance._address_additions = address_additions
		instance._address_deletions = address_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._min_removal_delta.to_bytes(1, byteorder='little', signed=True)
		buffer += self._min_approval_delta.to_bytes(1, byteorder='little', signed=True)
		buffer += len(self._address_additions).to_bytes(1, byteorder='little', signed=False)  # address_additions_count
		buffer += len(self._address_deletions).to_bytes(1, byteorder='little', signed=False)  # address_deletions_count
		buffer += self._multisig_account_modification_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._address_additions)
		buffer += ArrayHelpers.write_array(self._address_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'min_removal_delta: 0x{self._min_removal_delta:X}, '
		result += f'min_approval_delta: 0x{self._min_approval_delta:X}, '
		result += f'address_additions: {list(map(str, self._address_additions))}, '
		result += f'address_deletions: {list(map(str, self._address_deletions))}, '
		result += ')'
		return result


class AddressAliasTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ADDRESS_ALIAS
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'namespace_id': 'pod:NamespaceId',
		'address': 'pod:Address',
		'alias_action': 'enum:AliasAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AddressAliasTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AddressAliasTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._namespace_id = NamespaceId()
		self._address = Address()
		self._alias_action = AliasAction.UNLINK
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def namespace_id(self) -> NamespaceId:
		return self._namespace_id

	@property
	def address(self) -> Address:
		return self._address

	@property
	def alias_action(self) -> AliasAction:
		return self._alias_action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@namespace_id.setter
	def namespace_id(self, value: NamespaceId):
		self._namespace_id = value

	@address.setter
	def address(self, value: Address):
		self._address = value

	@alias_action.setter
	def alias_action(self, value: AliasAction):
		self._alias_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.namespace_id.size
		size += self.address.size
		size += self.alias_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AddressAliasTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[namespace_id.size:]
		address = Address.deserialize(buffer)
		buffer = buffer[address.size:]
		alias_action = AliasAction.deserialize(buffer)
		buffer = buffer[alias_action.size:]

		instance = AddressAliasTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._namespace_id = namespace_id
		instance._address = address
		instance._alias_action = alias_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._namespace_id.serialize()
		buffer += self._address.serialize()
		buffer += self._alias_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'namespace_id: {self._namespace_id.__str__()}, '
		result += f'address: {self._address.__str__()}, '
		result += f'alias_action: {self._alias_action.__str__()}, '
		result += ')'
		return result


class EmbeddedAddressAliasTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ADDRESS_ALIAS
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'namespace_id': 'pod:NamespaceId',
		'address': 'pod:Address',
		'alias_action': 'enum:AliasAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedAddressAliasTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedAddressAliasTransaction.TRANSACTION_TYPE
		self._namespace_id = NamespaceId()
		self._address = Address()
		self._alias_action = AliasAction.UNLINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def namespace_id(self) -> NamespaceId:
		return self._namespace_id

	@property
	def address(self) -> Address:
		return self._address

	@property
	def alias_action(self) -> AliasAction:
		return self._alias_action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@namespace_id.setter
	def namespace_id(self, value: NamespaceId):
		self._namespace_id = value

	@address.setter
	def address(self, value: Address):
		self._address = value

	@alias_action.setter
	def alias_action(self, value: AliasAction):
		self._alias_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.namespace_id.size
		size += self.address.size
		size += self.alias_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedAddressAliasTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[namespace_id.size:]
		address = Address.deserialize(buffer)
		buffer = buffer[address.size:]
		alias_action = AliasAction.deserialize(buffer)
		buffer = buffer[alias_action.size:]

		instance = EmbeddedAddressAliasTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._namespace_id = namespace_id
		instance._address = address
		instance._alias_action = alias_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._namespace_id.serialize()
		buffer += self._address.serialize()
		buffer += self._alias_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'namespace_id: {self._namespace_id.__str__()}, '
		result += f'address: {self._address.__str__()}, '
		result += f'alias_action: {self._alias_action.__str__()}, '
		result += ')'
		return result


class MosaicAliasTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_ALIAS
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'namespace_id': 'pod:NamespaceId',
		'mosaic_id': 'pod:MosaicId',
		'alias_action': 'enum:AliasAction'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicAliasTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicAliasTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._namespace_id = NamespaceId()
		self._mosaic_id = MosaicId()
		self._alias_action = AliasAction.UNLINK
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def namespace_id(self) -> NamespaceId:
		return self._namespace_id

	@property
	def mosaic_id(self) -> MosaicId:
		return self._mosaic_id

	@property
	def alias_action(self) -> AliasAction:
		return self._alias_action

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@namespace_id.setter
	def namespace_id(self, value: NamespaceId):
		self._namespace_id = value

	@mosaic_id.setter
	def mosaic_id(self, value: MosaicId):
		self._mosaic_id = value

	@alias_action.setter
	def alias_action(self, value: AliasAction):
		self._alias_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.namespace_id.size
		size += self.mosaic_id.size
		size += self.alias_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicAliasTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[namespace_id.size:]
		mosaic_id = MosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		alias_action = AliasAction.deserialize(buffer)
		buffer = buffer[alias_action.size:]

		instance = MosaicAliasTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._namespace_id = namespace_id
		instance._mosaic_id = mosaic_id
		instance._alias_action = alias_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._namespace_id.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._alias_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'namespace_id: {self._namespace_id.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'alias_action: {self._alias_action.__str__()}, '
		result += ')'
		return result


class EmbeddedMosaicAliasTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_ALIAS
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'namespace_id': 'pod:NamespaceId',
		'mosaic_id': 'pod:MosaicId',
		'alias_action': 'enum:AliasAction'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicAliasTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicAliasTransaction.TRANSACTION_TYPE
		self._namespace_id = NamespaceId()
		self._mosaic_id = MosaicId()
		self._alias_action = AliasAction.UNLINK
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def namespace_id(self) -> NamespaceId:
		return self._namespace_id

	@property
	def mosaic_id(self) -> MosaicId:
		return self._mosaic_id

	@property
	def alias_action(self) -> AliasAction:
		return self._alias_action

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@namespace_id.setter
	def namespace_id(self, value: NamespaceId):
		self._namespace_id = value

	@mosaic_id.setter
	def mosaic_id(self, value: MosaicId):
		self._mosaic_id = value

	@alias_action.setter
	def alias_action(self, value: AliasAction):
		self._alias_action = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.namespace_id.size
		size += self.mosaic_id.size
		size += self.alias_action.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicAliasTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[namespace_id.size:]
		mosaic_id = MosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		alias_action = AliasAction.deserialize(buffer)
		buffer = buffer[alias_action.size:]

		instance = EmbeddedMosaicAliasTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._namespace_id = namespace_id
		instance._mosaic_id = mosaic_id
		instance._alias_action = alias_action
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._namespace_id.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._alias_action.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'namespace_id: {self._namespace_id.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'alias_action: {self._alias_action.__str__()}, '
		result += ')'
		return result


class NamespaceRegistrationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NAMESPACE_REGISTRATION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'duration': 'pod:BlockDuration',
		'parent_id': 'pod:NamespaceId',
		'id': 'pod:NamespaceId',
		'registration_type': 'enum:NamespaceRegistrationType',
		'name': 'bytes_array'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = NamespaceRegistrationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = NamespaceRegistrationTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._duration = BlockDuration()
		self._parent_id = NamespaceId()
		self._id = NamespaceId()
		self._registration_type = NamespaceRegistrationType.ROOT
		self._name = bytes()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def parent_id(self) -> NamespaceId:
		return self._parent_id

	@property
	def id(self) -> NamespaceId:
		return self._id

	@property
	def registration_type(self) -> NamespaceRegistrationType:
		return self._registration_type

	@property
	def name(self) -> bytes:
		return self._name

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@parent_id.setter
	def parent_id(self, value: NamespaceId):
		self._parent_id = value

	@id.setter
	def id(self, value: NamespaceId):
		self._id = value

	@registration_type.setter
	def registration_type(self, value: NamespaceRegistrationType):
		self._registration_type = value

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		if NamespaceRegistrationType.ROOT == self.registration_type:
			size += self.duration.size
		if NamespaceRegistrationType.CHILD == self.registration_type:
			size += self.parent_id.size
		size += self.id.size
		size += self.registration_type.size
		size += 1
		size += len(self._name)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NamespaceRegistrationTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		# deserialize to temporary buffer for further processing
		duration_temporary = BlockDuration.deserialize(buffer)
		registration_type_condition = buffer[:duration_temporary.size]
		buffer = buffer[duration_temporary.size:]

		id = NamespaceId.deserialize(buffer)
		buffer = buffer[id.size:]
		registration_type = NamespaceRegistrationType.deserialize(buffer)
		buffer = buffer[registration_type.size:]
		duration = None
		if NamespaceRegistrationType.ROOT == registration_type:
			duration = BlockDuration.deserialize(registration_type_condition)
			registration_type_condition = registration_type_condition[duration.size:]
		parent_id = None
		if NamespaceRegistrationType.CHILD == registration_type:
			parent_id = NamespaceId.deserialize(registration_type_condition)
			registration_type_condition = registration_type_condition[parent_id.size:]
		name_size = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]

		instance = NamespaceRegistrationTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._duration = duration
		instance._parent_id = parent_id
		instance._id = id
		instance._registration_type = registration_type
		instance._name = name
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		if NamespaceRegistrationType.ROOT == self.registration_type:
			buffer += self._duration.serialize()
		if NamespaceRegistrationType.CHILD == self.registration_type:
			buffer += self._parent_id.serialize()
		buffer += self._id.serialize()
		buffer += self._registration_type.serialize()
		buffer += len(self._name).to_bytes(1, byteorder='little', signed=False)  # name_size
		buffer += self._name
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		if NamespaceRegistrationType.ROOT == self.registration_type:
			result += f'duration: {self._duration.__str__()}, '
		if NamespaceRegistrationType.CHILD == self.registration_type:
			result += f'parent_id: {self._parent_id.__str__()}, '
		result += f'id: {self._id.__str__()}, '
		result += f'registration_type: {self._registration_type.__str__()}, '
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		result += ')'
		return result


class EmbeddedNamespaceRegistrationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NAMESPACE_REGISTRATION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'duration': 'pod:BlockDuration',
		'parent_id': 'pod:NamespaceId',
		'id': 'pod:NamespaceId',
		'registration_type': 'enum:NamespaceRegistrationType',
		'name': 'bytes_array'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedNamespaceRegistrationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedNamespaceRegistrationTransaction.TRANSACTION_TYPE
		self._duration = BlockDuration()
		self._parent_id = NamespaceId()
		self._id = NamespaceId()
		self._registration_type = NamespaceRegistrationType.ROOT
		self._name = bytes()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def duration(self) -> BlockDuration:
		return self._duration

	@property
	def parent_id(self) -> NamespaceId:
		return self._parent_id

	@property
	def id(self) -> NamespaceId:
		return self._id

	@property
	def registration_type(self) -> NamespaceRegistrationType:
		return self._registration_type

	@property
	def name(self) -> bytes:
		return self._name

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@duration.setter
	def duration(self, value: BlockDuration):
		self._duration = value

	@parent_id.setter
	def parent_id(self, value: NamespaceId):
		self._parent_id = value

	@id.setter
	def id(self, value: NamespaceId):
		self._id = value

	@registration_type.setter
	def registration_type(self, value: NamespaceRegistrationType):
		self._registration_type = value

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		if NamespaceRegistrationType.ROOT == self.registration_type:
			size += self.duration.size
		if NamespaceRegistrationType.CHILD == self.registration_type:
			size += self.parent_id.size
		size += self.id.size
		size += self.registration_type.size
		size += 1
		size += len(self._name)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedNamespaceRegistrationTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		# deserialize to temporary buffer for further processing
		duration_temporary = BlockDuration.deserialize(buffer)
		registration_type_condition = buffer[:duration_temporary.size]
		buffer = buffer[duration_temporary.size:]

		id = NamespaceId.deserialize(buffer)
		buffer = buffer[id.size:]
		registration_type = NamespaceRegistrationType.deserialize(buffer)
		buffer = buffer[registration_type.size:]
		duration = None
		if NamespaceRegistrationType.ROOT == registration_type:
			duration = BlockDuration.deserialize(registration_type_condition)
			registration_type_condition = registration_type_condition[duration.size:]
		parent_id = None
		if NamespaceRegistrationType.CHILD == registration_type:
			parent_id = NamespaceId.deserialize(registration_type_condition)
			registration_type_condition = registration_type_condition[parent_id.size:]
		name_size = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]

		instance = EmbeddedNamespaceRegistrationTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._duration = duration
		instance._parent_id = parent_id
		instance._id = id
		instance._registration_type = registration_type
		instance._name = name
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		if NamespaceRegistrationType.ROOT == self.registration_type:
			buffer += self._duration.serialize()
		if NamespaceRegistrationType.CHILD == self.registration_type:
			buffer += self._parent_id.serialize()
		buffer += self._id.serialize()
		buffer += self._registration_type.serialize()
		buffer += len(self._name).to_bytes(1, byteorder='little', signed=False)  # name_size
		buffer += self._name
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		if NamespaceRegistrationType.ROOT == self.registration_type:
			result += f'duration: {self._duration.__str__()}, '
		if NamespaceRegistrationType.CHILD == self.registration_type:
			result += f'parent_id: {self._parent_id.__str__()}, '
		result += f'id: {self._id.__str__()}, '
		result += f'registration_type: {self._registration_type.__str__()}, '
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		result += ')'
		return result


class AccountRestrictionFlags(Flag):
	ADDRESS = 1
	MOSAIC_ID = 2
	TRANSACTION_TYPE = 4
	OUTGOING = 16384
	BLOCK = 32768

	@property
	def size(self) -> int:
		return 2

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountRestrictionFlags:
		buffer = memoryview(payload)
		return AccountRestrictionFlags(int.from_bytes(buffer[:2], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(2, byteorder='little', signed=False)
		return buffer


class AccountAddressRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_ADDRESS_RESTRICTION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'restriction_flags': 'enum:AccountRestrictionFlags',
		'restriction_additions': 'array[UnresolvedAddress]',
		'restriction_deletions': 'array[UnresolvedAddress]'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AccountAddressRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AccountAddressRestrictionTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._restriction_flags = AccountRestrictionFlags.ADDRESS
		self._restriction_additions = []
		self._restriction_deletions = []
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._account_restriction_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def restriction_flags(self) -> AccountRestrictionFlags:
		return self._restriction_flags

	@property
	def restriction_additions(self) -> List[UnresolvedAddress]:
		return self._restriction_additions

	@property
	def restriction_deletions(self) -> List[UnresolvedAddress]:
		return self._restriction_deletions

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@restriction_flags.setter
	def restriction_flags(self, value: AccountRestrictionFlags):
		self._restriction_flags = value

	@restriction_additions.setter
	def restriction_additions(self, value: List[UnresolvedAddress]):
		self._restriction_additions = value

	@restriction_deletions.setter
	def restriction_deletions(self, value: List[UnresolvedAddress]):
		self._restriction_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.restriction_flags.size
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.restriction_additions)
		size += ArrayHelpers.size(self.restriction_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountAddressRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		restriction_flags = AccountRestrictionFlags.deserialize(buffer)
		buffer = buffer[restriction_flags.size:]
		restriction_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		restriction_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		account_restriction_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert account_restriction_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({account_restriction_transaction_body_reserved_1})'
		restriction_additions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, restriction_additions_count)
		buffer = buffer[ArrayHelpers.size(restriction_additions):]
		restriction_deletions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, restriction_deletions_count)
		buffer = buffer[ArrayHelpers.size(restriction_deletions):]

		instance = AccountAddressRestrictionTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._restriction_flags = restriction_flags
		instance._restriction_additions = restriction_additions
		instance._restriction_deletions = restriction_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._restriction_flags.serialize()
		buffer += len(self._restriction_additions).to_bytes(1, byteorder='little', signed=False)  # restriction_additions_count
		buffer += len(self._restriction_deletions).to_bytes(1, byteorder='little', signed=False)  # restriction_deletions_count
		buffer += self._account_restriction_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._restriction_additions)
		buffer += ArrayHelpers.write_array(self._restriction_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'restriction_flags: {self._restriction_flags.__str__()}, '
		result += f'restriction_additions: {list(map(str, self._restriction_additions))}, '
		result += f'restriction_deletions: {list(map(str, self._restriction_deletions))}, '
		result += ')'
		return result


class EmbeddedAccountAddressRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_ADDRESS_RESTRICTION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'restriction_flags': 'enum:AccountRestrictionFlags',
		'restriction_additions': 'array[UnresolvedAddress]',
		'restriction_deletions': 'array[UnresolvedAddress]'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedAccountAddressRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedAccountAddressRestrictionTransaction.TRANSACTION_TYPE
		self._restriction_flags = AccountRestrictionFlags.ADDRESS
		self._restriction_additions = []
		self._restriction_deletions = []
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._account_restriction_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def restriction_flags(self) -> AccountRestrictionFlags:
		return self._restriction_flags

	@property
	def restriction_additions(self) -> List[UnresolvedAddress]:
		return self._restriction_additions

	@property
	def restriction_deletions(self) -> List[UnresolvedAddress]:
		return self._restriction_deletions

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@restriction_flags.setter
	def restriction_flags(self, value: AccountRestrictionFlags):
		self._restriction_flags = value

	@restriction_additions.setter
	def restriction_additions(self, value: List[UnresolvedAddress]):
		self._restriction_additions = value

	@restriction_deletions.setter
	def restriction_deletions(self, value: List[UnresolvedAddress]):
		self._restriction_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.restriction_flags.size
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.restriction_additions)
		size += ArrayHelpers.size(self.restriction_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedAccountAddressRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		restriction_flags = AccountRestrictionFlags.deserialize(buffer)
		buffer = buffer[restriction_flags.size:]
		restriction_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		restriction_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		account_restriction_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert account_restriction_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({account_restriction_transaction_body_reserved_1})'
		restriction_additions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, restriction_additions_count)
		buffer = buffer[ArrayHelpers.size(restriction_additions):]
		restriction_deletions = ArrayHelpers.read_array_count(buffer, UnresolvedAddress, restriction_deletions_count)
		buffer = buffer[ArrayHelpers.size(restriction_deletions):]

		instance = EmbeddedAccountAddressRestrictionTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._restriction_flags = restriction_flags
		instance._restriction_additions = restriction_additions
		instance._restriction_deletions = restriction_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._restriction_flags.serialize()
		buffer += len(self._restriction_additions).to_bytes(1, byteorder='little', signed=False)  # restriction_additions_count
		buffer += len(self._restriction_deletions).to_bytes(1, byteorder='little', signed=False)  # restriction_deletions_count
		buffer += self._account_restriction_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._restriction_additions)
		buffer += ArrayHelpers.write_array(self._restriction_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'restriction_flags: {self._restriction_flags.__str__()}, '
		result += f'restriction_additions: {list(map(str, self._restriction_additions))}, '
		result += f'restriction_deletions: {list(map(str, self._restriction_deletions))}, '
		result += ')'
		return result


class AccountMosaicRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_MOSAIC_RESTRICTION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'restriction_flags': 'enum:AccountRestrictionFlags',
		'restriction_additions': 'array[UnresolvedMosaicId]',
		'restriction_deletions': 'array[UnresolvedMosaicId]'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AccountMosaicRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AccountMosaicRestrictionTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._restriction_flags = AccountRestrictionFlags.ADDRESS
		self._restriction_additions = []
		self._restriction_deletions = []
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._account_restriction_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def restriction_flags(self) -> AccountRestrictionFlags:
		return self._restriction_flags

	@property
	def restriction_additions(self) -> List[UnresolvedMosaicId]:
		return self._restriction_additions

	@property
	def restriction_deletions(self) -> List[UnresolvedMosaicId]:
		return self._restriction_deletions

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@restriction_flags.setter
	def restriction_flags(self, value: AccountRestrictionFlags):
		self._restriction_flags = value

	@restriction_additions.setter
	def restriction_additions(self, value: List[UnresolvedMosaicId]):
		self._restriction_additions = value

	@restriction_deletions.setter
	def restriction_deletions(self, value: List[UnresolvedMosaicId]):
		self._restriction_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.restriction_flags.size
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.restriction_additions)
		size += ArrayHelpers.size(self.restriction_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountMosaicRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		restriction_flags = AccountRestrictionFlags.deserialize(buffer)
		buffer = buffer[restriction_flags.size:]
		restriction_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		restriction_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		account_restriction_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert account_restriction_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({account_restriction_transaction_body_reserved_1})'
		restriction_additions = ArrayHelpers.read_array_count(buffer, UnresolvedMosaicId, restriction_additions_count)
		buffer = buffer[ArrayHelpers.size(restriction_additions):]
		restriction_deletions = ArrayHelpers.read_array_count(buffer, UnresolvedMosaicId, restriction_deletions_count)
		buffer = buffer[ArrayHelpers.size(restriction_deletions):]

		instance = AccountMosaicRestrictionTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._restriction_flags = restriction_flags
		instance._restriction_additions = restriction_additions
		instance._restriction_deletions = restriction_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._restriction_flags.serialize()
		buffer += len(self._restriction_additions).to_bytes(1, byteorder='little', signed=False)  # restriction_additions_count
		buffer += len(self._restriction_deletions).to_bytes(1, byteorder='little', signed=False)  # restriction_deletions_count
		buffer += self._account_restriction_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._restriction_additions)
		buffer += ArrayHelpers.write_array(self._restriction_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'restriction_flags: {self._restriction_flags.__str__()}, '
		result += f'restriction_additions: {list(map(str, self._restriction_additions))}, '
		result += f'restriction_deletions: {list(map(str, self._restriction_deletions))}, '
		result += ')'
		return result


class EmbeddedAccountMosaicRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_MOSAIC_RESTRICTION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'restriction_flags': 'enum:AccountRestrictionFlags',
		'restriction_additions': 'array[UnresolvedMosaicId]',
		'restriction_deletions': 'array[UnresolvedMosaicId]'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedAccountMosaicRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedAccountMosaicRestrictionTransaction.TRANSACTION_TYPE
		self._restriction_flags = AccountRestrictionFlags.ADDRESS
		self._restriction_additions = []
		self._restriction_deletions = []
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._account_restriction_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def restriction_flags(self) -> AccountRestrictionFlags:
		return self._restriction_flags

	@property
	def restriction_additions(self) -> List[UnresolvedMosaicId]:
		return self._restriction_additions

	@property
	def restriction_deletions(self) -> List[UnresolvedMosaicId]:
		return self._restriction_deletions

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@restriction_flags.setter
	def restriction_flags(self, value: AccountRestrictionFlags):
		self._restriction_flags = value

	@restriction_additions.setter
	def restriction_additions(self, value: List[UnresolvedMosaicId]):
		self._restriction_additions = value

	@restriction_deletions.setter
	def restriction_deletions(self, value: List[UnresolvedMosaicId]):
		self._restriction_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.restriction_flags.size
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.restriction_additions)
		size += ArrayHelpers.size(self.restriction_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedAccountMosaicRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		restriction_flags = AccountRestrictionFlags.deserialize(buffer)
		buffer = buffer[restriction_flags.size:]
		restriction_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		restriction_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		account_restriction_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert account_restriction_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({account_restriction_transaction_body_reserved_1})'
		restriction_additions = ArrayHelpers.read_array_count(buffer, UnresolvedMosaicId, restriction_additions_count)
		buffer = buffer[ArrayHelpers.size(restriction_additions):]
		restriction_deletions = ArrayHelpers.read_array_count(buffer, UnresolvedMosaicId, restriction_deletions_count)
		buffer = buffer[ArrayHelpers.size(restriction_deletions):]

		instance = EmbeddedAccountMosaicRestrictionTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._restriction_flags = restriction_flags
		instance._restriction_additions = restriction_additions
		instance._restriction_deletions = restriction_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._restriction_flags.serialize()
		buffer += len(self._restriction_additions).to_bytes(1, byteorder='little', signed=False)  # restriction_additions_count
		buffer += len(self._restriction_deletions).to_bytes(1, byteorder='little', signed=False)  # restriction_deletions_count
		buffer += self._account_restriction_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._restriction_additions)
		buffer += ArrayHelpers.write_array(self._restriction_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'restriction_flags: {self._restriction_flags.__str__()}, '
		result += f'restriction_additions: {list(map(str, self._restriction_additions))}, '
		result += f'restriction_deletions: {list(map(str, self._restriction_deletions))}, '
		result += ')'
		return result


class AccountOperationRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_OPERATION_RESTRICTION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'restriction_flags': 'enum:AccountRestrictionFlags',
		'restriction_additions': 'array[TransactionType]',
		'restriction_deletions': 'array[TransactionType]'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = AccountOperationRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = AccountOperationRestrictionTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._restriction_flags = AccountRestrictionFlags.ADDRESS
		self._restriction_additions = []
		self._restriction_deletions = []
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._account_restriction_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def restriction_flags(self) -> AccountRestrictionFlags:
		return self._restriction_flags

	@property
	def restriction_additions(self) -> List[TransactionType]:
		return self._restriction_additions

	@property
	def restriction_deletions(self) -> List[TransactionType]:
		return self._restriction_deletions

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@restriction_flags.setter
	def restriction_flags(self, value: AccountRestrictionFlags):
		self._restriction_flags = value

	@restriction_additions.setter
	def restriction_additions(self, value: List[TransactionType]):
		self._restriction_additions = value

	@restriction_deletions.setter
	def restriction_deletions(self, value: List[TransactionType]):
		self._restriction_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.restriction_flags.size
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.restriction_additions)
		size += ArrayHelpers.size(self.restriction_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountOperationRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		restriction_flags = AccountRestrictionFlags.deserialize(buffer)
		buffer = buffer[restriction_flags.size:]
		restriction_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		restriction_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		account_restriction_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert account_restriction_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({account_restriction_transaction_body_reserved_1})'
		restriction_additions = ArrayHelpers.read_array_count(buffer, TransactionType, restriction_additions_count)
		buffer = buffer[ArrayHelpers.size(restriction_additions):]
		restriction_deletions = ArrayHelpers.read_array_count(buffer, TransactionType, restriction_deletions_count)
		buffer = buffer[ArrayHelpers.size(restriction_deletions):]

		instance = AccountOperationRestrictionTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._restriction_flags = restriction_flags
		instance._restriction_additions = restriction_additions
		instance._restriction_deletions = restriction_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._restriction_flags.serialize()
		buffer += len(self._restriction_additions).to_bytes(1, byteorder='little', signed=False)  # restriction_additions_count
		buffer += len(self._restriction_deletions).to_bytes(1, byteorder='little', signed=False)  # restriction_deletions_count
		buffer += self._account_restriction_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._restriction_additions)
		buffer += ArrayHelpers.write_array(self._restriction_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'restriction_flags: {self._restriction_flags.__str__()}, '
		result += f'restriction_additions: {list(map(str, self._restriction_additions))}, '
		result += f'restriction_deletions: {list(map(str, self._restriction_deletions))}, '
		result += ')'
		return result


class EmbeddedAccountOperationRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_OPERATION_RESTRICTION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'restriction_flags': 'enum:AccountRestrictionFlags',
		'restriction_additions': 'array[TransactionType]',
		'restriction_deletions': 'array[TransactionType]'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedAccountOperationRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedAccountOperationRestrictionTransaction.TRANSACTION_TYPE
		self._restriction_flags = AccountRestrictionFlags.ADDRESS
		self._restriction_additions = []
		self._restriction_deletions = []
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._account_restriction_transaction_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def restriction_flags(self) -> AccountRestrictionFlags:
		return self._restriction_flags

	@property
	def restriction_additions(self) -> List[TransactionType]:
		return self._restriction_additions

	@property
	def restriction_deletions(self) -> List[TransactionType]:
		return self._restriction_deletions

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@restriction_flags.setter
	def restriction_flags(self, value: AccountRestrictionFlags):
		self._restriction_flags = value

	@restriction_additions.setter
	def restriction_additions(self, value: List[TransactionType]):
		self._restriction_additions = value

	@restriction_deletions.setter
	def restriction_deletions(self, value: List[TransactionType]):
		self._restriction_deletions = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.restriction_flags.size
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.restriction_additions)
		size += ArrayHelpers.size(self.restriction_deletions)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedAccountOperationRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		restriction_flags = AccountRestrictionFlags.deserialize(buffer)
		buffer = buffer[restriction_flags.size:]
		restriction_additions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		restriction_deletions_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		account_restriction_transaction_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert account_restriction_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({account_restriction_transaction_body_reserved_1})'
		restriction_additions = ArrayHelpers.read_array_count(buffer, TransactionType, restriction_additions_count)
		buffer = buffer[ArrayHelpers.size(restriction_additions):]
		restriction_deletions = ArrayHelpers.read_array_count(buffer, TransactionType, restriction_deletions_count)
		buffer = buffer[ArrayHelpers.size(restriction_deletions):]

		instance = EmbeddedAccountOperationRestrictionTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._restriction_flags = restriction_flags
		instance._restriction_additions = restriction_additions
		instance._restriction_deletions = restriction_deletions
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._restriction_flags.serialize()
		buffer += len(self._restriction_additions).to_bytes(1, byteorder='little', signed=False)  # restriction_additions_count
		buffer += len(self._restriction_deletions).to_bytes(1, byteorder='little', signed=False)  # restriction_deletions_count
		buffer += self._account_restriction_transaction_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._restriction_additions)
		buffer += ArrayHelpers.write_array(self._restriction_deletions)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'restriction_flags: {self._restriction_flags.__str__()}, '
		result += f'restriction_additions: {list(map(str, self._restriction_additions))}, '
		result += f'restriction_deletions: {list(map(str, self._restriction_deletions))}, '
		result += ')'
		return result


class MosaicAddressRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_ADDRESS_RESTRICTION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'target_address': 'pod:UnresolvedAddress'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicAddressRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicAddressRestrictionTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_id = UnresolvedMosaicId()
		self._restriction_key = 0
		self._previous_restriction_value = 0
		self._new_restriction_value = 0
		self._target_address = UnresolvedAddress()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def restriction_key(self) -> int:
		return self._restriction_key

	@property
	def previous_restriction_value(self) -> int:
		return self._previous_restriction_value

	@property
	def new_restriction_value(self) -> int:
		return self._new_restriction_value

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@restriction_key.setter
	def restriction_key(self, value: int):
		self._restriction_key = value

	@previous_restriction_value.setter
	def previous_restriction_value(self, value: int):
		self._previous_restriction_value = value

	@new_restriction_value.setter
	def new_restriction_value(self, value: int):
		self._new_restriction_value = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.mosaic_id.size
		size += 8
		size += 8
		size += 8
		size += self.target_address.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicAddressRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		restriction_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		previous_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		new_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]

		instance = MosaicAddressRestrictionTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_id = mosaic_id
		instance._restriction_key = restriction_key
		instance._previous_restriction_value = previous_restriction_value
		instance._new_restriction_value = new_restriction_value
		instance._target_address = target_address
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._restriction_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._previous_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._new_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._target_address.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'restriction_key: 0x{self._restriction_key:X}, '
		result += f'previous_restriction_value: 0x{self._previous_restriction_value:X}, '
		result += f'new_restriction_value: 0x{self._new_restriction_value:X}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += ')'
		return result


class EmbeddedMosaicAddressRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_ADDRESS_RESTRICTION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'target_address': 'pod:UnresolvedAddress'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicAddressRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicAddressRestrictionTransaction.TRANSACTION_TYPE
		self._mosaic_id = UnresolvedMosaicId()
		self._restriction_key = 0
		self._previous_restriction_value = 0
		self._new_restriction_value = 0
		self._target_address = UnresolvedAddress()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def restriction_key(self) -> int:
		return self._restriction_key

	@property
	def previous_restriction_value(self) -> int:
		return self._previous_restriction_value

	@property
	def new_restriction_value(self) -> int:
		return self._new_restriction_value

	@property
	def target_address(self) -> UnresolvedAddress:
		return self._target_address

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@restriction_key.setter
	def restriction_key(self, value: int):
		self._restriction_key = value

	@previous_restriction_value.setter
	def previous_restriction_value(self, value: int):
		self._previous_restriction_value = value

	@new_restriction_value.setter
	def new_restriction_value(self, value: int):
		self._new_restriction_value = value

	@target_address.setter
	def target_address(self, value: UnresolvedAddress):
		self._target_address = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.mosaic_id.size
		size += 8
		size += 8
		size += 8
		size += self.target_address.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicAddressRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		restriction_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		previous_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		new_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		target_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[target_address.size:]

		instance = EmbeddedMosaicAddressRestrictionTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._mosaic_id = mosaic_id
		instance._restriction_key = restriction_key
		instance._previous_restriction_value = previous_restriction_value
		instance._new_restriction_value = new_restriction_value
		instance._target_address = target_address
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._restriction_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._previous_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._new_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._target_address.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'restriction_key: 0x{self._restriction_key:X}, '
		result += f'previous_restriction_value: 0x{self._previous_restriction_value:X}, '
		result += f'new_restriction_value: 0x{self._new_restriction_value:X}, '
		result += f'target_address: {self._target_address.__str__()}, '
		result += ')'
		return result


class MosaicRestrictionKey(BaseValue):
	SIZE = 8

	def __init__(self, mosaic_restriction_key: int = 0):
		super().__init__(self.SIZE, mosaic_restriction_key, MosaicRestrictionKey)

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicRestrictionKey:
		buffer = memoryview(payload)
		return MosaicRestrictionKey(int.from_bytes(buffer[:8], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(8, byteorder='little', signed=False)


class MosaicRestrictionType(Enum):
	NONE = 0
	EQ = 1
	NE = 2
	LT = 3
	LE = 4
	GT = 5
	GE = 6

	@property
	def size(self) -> int:
		return 1

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicRestrictionType:
		buffer = memoryview(payload)
		return MosaicRestrictionType(int.from_bytes(buffer[:1], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(1, byteorder='little', signed=False)
		return buffer


class MosaicGlobalRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_GLOBAL_RESTRICTION
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'reference_mosaic_id': 'pod:UnresolvedMosaicId',
		'previous_restriction_type': 'enum:MosaicRestrictionType',
		'new_restriction_type': 'enum:MosaicRestrictionType'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = MosaicGlobalRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = MosaicGlobalRestrictionTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_id = UnresolvedMosaicId()
		self._reference_mosaic_id = UnresolvedMosaicId()
		self._restriction_key = 0
		self._previous_restriction_value = 0
		self._new_restriction_value = 0
		self._previous_restriction_type = MosaicRestrictionType.NONE
		self._new_restriction_type = MosaicRestrictionType.NONE
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def reference_mosaic_id(self) -> UnresolvedMosaicId:
		return self._reference_mosaic_id

	@property
	def restriction_key(self) -> int:
		return self._restriction_key

	@property
	def previous_restriction_value(self) -> int:
		return self._previous_restriction_value

	@property
	def new_restriction_value(self) -> int:
		return self._new_restriction_value

	@property
	def previous_restriction_type(self) -> MosaicRestrictionType:
		return self._previous_restriction_type

	@property
	def new_restriction_type(self) -> MosaicRestrictionType:
		return self._new_restriction_type

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@reference_mosaic_id.setter
	def reference_mosaic_id(self, value: UnresolvedMosaicId):
		self._reference_mosaic_id = value

	@restriction_key.setter
	def restriction_key(self, value: int):
		self._restriction_key = value

	@previous_restriction_value.setter
	def previous_restriction_value(self, value: int):
		self._previous_restriction_value = value

	@new_restriction_value.setter
	def new_restriction_value(self, value: int):
		self._new_restriction_value = value

	@previous_restriction_type.setter
	def previous_restriction_type(self, value: MosaicRestrictionType):
		self._previous_restriction_type = value

	@new_restriction_type.setter
	def new_restriction_type(self, value: MosaicRestrictionType):
		self._new_restriction_type = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.mosaic_id.size
		size += self.reference_mosaic_id.size
		size += 8
		size += 8
		size += 8
		size += self.previous_restriction_type.size
		size += self.new_restriction_type.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicGlobalRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		reference_mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[reference_mosaic_id.size:]
		restriction_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		previous_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		new_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		previous_restriction_type = MosaicRestrictionType.deserialize(buffer)
		buffer = buffer[previous_restriction_type.size:]
		new_restriction_type = MosaicRestrictionType.deserialize(buffer)
		buffer = buffer[new_restriction_type.size:]

		instance = MosaicGlobalRestrictionTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_id = mosaic_id
		instance._reference_mosaic_id = reference_mosaic_id
		instance._restriction_key = restriction_key
		instance._previous_restriction_value = previous_restriction_value
		instance._new_restriction_value = new_restriction_value
		instance._previous_restriction_type = previous_restriction_type
		instance._new_restriction_type = new_restriction_type
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._reference_mosaic_id.serialize()
		buffer += self._restriction_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._previous_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._new_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._previous_restriction_type.serialize()
		buffer += self._new_restriction_type.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'reference_mosaic_id: {self._reference_mosaic_id.__str__()}, '
		result += f'restriction_key: 0x{self._restriction_key:X}, '
		result += f'previous_restriction_value: 0x{self._previous_restriction_value:X}, '
		result += f'new_restriction_value: 0x{self._new_restriction_value:X}, '
		result += f'previous_restriction_type: {self._previous_restriction_type.__str__()}, '
		result += f'new_restriction_type: {self._new_restriction_type.__str__()}, '
		result += ')'
		return result


class EmbeddedMosaicGlobalRestrictionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_GLOBAL_RESTRICTION
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'mosaic_id': 'pod:UnresolvedMosaicId',
		'reference_mosaic_id': 'pod:UnresolvedMosaicId',
		'previous_restriction_type': 'enum:MosaicRestrictionType',
		'new_restriction_type': 'enum:MosaicRestrictionType'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedMosaicGlobalRestrictionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedMosaicGlobalRestrictionTransaction.TRANSACTION_TYPE
		self._mosaic_id = UnresolvedMosaicId()
		self._reference_mosaic_id = UnresolvedMosaicId()
		self._restriction_key = 0
		self._previous_restriction_value = 0
		self._new_restriction_value = 0
		self._previous_restriction_type = MosaicRestrictionType.NONE
		self._new_restriction_type = MosaicRestrictionType.NONE
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def mosaic_id(self) -> UnresolvedMosaicId:
		return self._mosaic_id

	@property
	def reference_mosaic_id(self) -> UnresolvedMosaicId:
		return self._reference_mosaic_id

	@property
	def restriction_key(self) -> int:
		return self._restriction_key

	@property
	def previous_restriction_value(self) -> int:
		return self._previous_restriction_value

	@property
	def new_restriction_value(self) -> int:
		return self._new_restriction_value

	@property
	def previous_restriction_type(self) -> MosaicRestrictionType:
		return self._previous_restriction_type

	@property
	def new_restriction_type(self) -> MosaicRestrictionType:
		return self._new_restriction_type

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@mosaic_id.setter
	def mosaic_id(self, value: UnresolvedMosaicId):
		self._mosaic_id = value

	@reference_mosaic_id.setter
	def reference_mosaic_id(self, value: UnresolvedMosaicId):
		self._reference_mosaic_id = value

	@restriction_key.setter
	def restriction_key(self, value: int):
		self._restriction_key = value

	@previous_restriction_value.setter
	def previous_restriction_value(self, value: int):
		self._previous_restriction_value = value

	@new_restriction_value.setter
	def new_restriction_value(self, value: int):
		self._new_restriction_value = value

	@previous_restriction_type.setter
	def previous_restriction_type(self, value: MosaicRestrictionType):
		self._previous_restriction_type = value

	@new_restriction_type.setter
	def new_restriction_type(self, value: MosaicRestrictionType):
		self._new_restriction_type = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.mosaic_id.size
		size += self.reference_mosaic_id.size
		size += 8
		size += 8
		size += 8
		size += self.previous_restriction_type.size
		size += self.new_restriction_type.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedMosaicGlobalRestrictionTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[mosaic_id.size:]
		reference_mosaic_id = UnresolvedMosaicId.deserialize(buffer)
		buffer = buffer[reference_mosaic_id.size:]
		restriction_key = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		previous_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		new_restriction_value = int.from_bytes(buffer[:8], byteorder='little', signed=False)
		buffer = buffer[8:]
		previous_restriction_type = MosaicRestrictionType.deserialize(buffer)
		buffer = buffer[previous_restriction_type.size:]
		new_restriction_type = MosaicRestrictionType.deserialize(buffer)
		buffer = buffer[new_restriction_type.size:]

		instance = EmbeddedMosaicGlobalRestrictionTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._mosaic_id = mosaic_id
		instance._reference_mosaic_id = reference_mosaic_id
		instance._restriction_key = restriction_key
		instance._previous_restriction_value = previous_restriction_value
		instance._new_restriction_value = new_restriction_value
		instance._previous_restriction_type = previous_restriction_type
		instance._new_restriction_type = new_restriction_type
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._mosaic_id.serialize()
		buffer += self._reference_mosaic_id.serialize()
		buffer += self._restriction_key.to_bytes(8, byteorder='little', signed=False)
		buffer += self._previous_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._new_restriction_value.to_bytes(8, byteorder='little', signed=False)
		buffer += self._previous_restriction_type.serialize()
		buffer += self._new_restriction_type.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'reference_mosaic_id: {self._reference_mosaic_id.__str__()}, '
		result += f'restriction_key: 0x{self._restriction_key:X}, '
		result += f'previous_restriction_value: 0x{self._previous_restriction_value:X}, '
		result += f'new_restriction_value: 0x{self._new_restriction_value:X}, '
		result += f'previous_restriction_type: {self._previous_restriction_type.__str__()}, '
		result += f'new_restriction_type: {self._new_restriction_type.__str__()}, '
		result += ')'
		return result


class TransferTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.TRANSFER
	TYPE_HINTS = {
		'signature': 'pod:Signature',
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:UnresolvedAddress',
		'mosaics': 'array[UnresolvedMosaic]',
		'message': 'bytes_array'
	}

	def __init__(self):
		self._signature = Signature()
		self._signer_public_key = PublicKey()
		self._version = TransferTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = TransferTransaction.TRANSACTION_TYPE
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = UnresolvedAddress()
		self._mosaics = []
		self._message = bytes()
		self._verifiable_entity_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._transfer_transaction_body_reserved_1 = 0  # reserved field
		self._transfer_transaction_body_reserved_2 = 0  # reserved field

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> UnresolvedAddress:
		return self._recipient_address

	@property
	def mosaics(self) -> List[UnresolvedMosaic]:
		return self._mosaics

	@property
	def message(self) -> bytes:
		return self._message

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: UnresolvedAddress):
		self._recipient_address = value

	@mosaics.setter
	def mosaics(self, value: List[UnresolvedMosaic]):
		self._mosaics = value

	@message.setter
	def message(self, value: bytes):
		self._message = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signature.size
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.fee.size
		size += self.deadline.size
		size += self.recipient_address.size
		size += 2
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.mosaics)
		size += len(self._message)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> TransferTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		verifiable_entity_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert verifiable_entity_header_reserved_1 == 0, f'Invalid value of reserved field ({verifiable_entity_header_reserved_1})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		message_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		mosaics_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		transfer_transaction_body_reserved_1 = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		assert transfer_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({transfer_transaction_body_reserved_1})'
		transfer_transaction_body_reserved_2 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert transfer_transaction_body_reserved_2 == 0, f'Invalid value of reserved field ({transfer_transaction_body_reserved_2})'
		mosaics = ArrayHelpers.read_array_count(buffer, UnresolvedMosaic, mosaics_count, lambda e: e.mosaic_id)
		buffer = buffer[ArrayHelpers.size(mosaics):]
		message = ArrayHelpers.get_bytes(buffer, message_size)
		buffer = buffer[message_size:]

		instance = TransferTransaction()
		instance._signature = signature
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._mosaics = mosaics
		instance._message = message
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._verifiable_entity_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address.serialize()
		buffer += len(self._message).to_bytes(2, byteorder='little', signed=False)  # message_size
		buffer += len(self._mosaics).to_bytes(1, byteorder='little', signed=False)  # mosaics_count
		buffer += self._transfer_transaction_body_reserved_1.to_bytes(1, byteorder='little', signed=False)
		buffer += self._transfer_transaction_body_reserved_2.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._mosaics, lambda e: e.mosaic_id)
		buffer += self._message
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signature: {self._signature.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'mosaics: {list(map(str, self._mosaics))}, '
		result += f'message: {hexlify(self._message).decode("utf8")}, '
		result += ')'
		return result


class EmbeddedTransferTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.TRANSFER
	TYPE_HINTS = {
		'signer_public_key': 'pod:PublicKey',
		'network': 'enum:NetworkType',
		'type_': 'enum:TransactionType',
		'recipient_address': 'pod:UnresolvedAddress',
		'mosaics': 'array[UnresolvedMosaic]',
		'message': 'bytes_array'
	}

	def __init__(self):
		self._signer_public_key = PublicKey()
		self._version = EmbeddedTransferTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._type_ = EmbeddedTransferTransaction.TRANSACTION_TYPE
		self._recipient_address = UnresolvedAddress()
		self._mosaics = []
		self._message = bytes()
		self._embedded_transaction_header_reserved_1 = 0  # reserved field
		self._entity_body_reserved_1 = 0  # reserved field
		self._transfer_transaction_body_reserved_1 = 0  # reserved field
		self._transfer_transaction_body_reserved_2 = 0  # reserved field

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def recipient_address(self) -> UnresolvedAddress:
		return self._recipient_address

	@property
	def mosaics(self) -> List[UnresolvedMosaic]:
		return self._mosaics

	@property
	def message(self) -> bytes:
		return self._message

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@recipient_address.setter
	def recipient_address(self, value: UnresolvedAddress):
		self._recipient_address = value

	@mosaics.setter
	def mosaics(self, value: List[UnresolvedMosaic]):
		self._mosaics = value

	@message.setter
	def message(self, value: bytes):
		self._message = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += 1
		size += self.network.size
		size += self.type_.size
		size += self.recipient_address.size
		size += 2
		size += 1
		size += 1
		size += 4
		size += ArrayHelpers.size(self.mosaics)
		size += len(self._message)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> EmbeddedTransferTransaction:
		buffer = memoryview(payload)
		size_ = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		buffer = buffer[:size_ - 4]
		del size_
		embedded_transaction_header_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert embedded_transaction_header_reserved_1 == 0, f'Invalid value of reserved field ({embedded_transaction_header_reserved_1})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		entity_body_reserved_1 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		recipient_address = UnresolvedAddress.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		message_size = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		mosaics_count = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		transfer_transaction_body_reserved_1 = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		assert transfer_transaction_body_reserved_1 == 0, f'Invalid value of reserved field ({transfer_transaction_body_reserved_1})'
		transfer_transaction_body_reserved_2 = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert transfer_transaction_body_reserved_2 == 0, f'Invalid value of reserved field ({transfer_transaction_body_reserved_2})'
		mosaics = ArrayHelpers.read_array_count(buffer, UnresolvedMosaic, mosaics_count, lambda e: e.mosaic_id)
		buffer = buffer[ArrayHelpers.size(mosaics):]
		message = ArrayHelpers.get_bytes(buffer, message_size)
		buffer = buffer[message_size:]

		instance = EmbeddedTransferTransaction()
		instance._signer_public_key = signer_public_key
		instance._version = version
		instance._network = network
		instance._type_ = type_
		instance._recipient_address = recipient_address
		instance._mosaics = mosaics
		instance._message = message
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._embedded_transaction_header_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._entity_body_reserved_1.to_bytes(4, byteorder='little', signed=False)
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._type_.serialize()
		buffer += self._recipient_address.serialize()
		buffer += len(self._message).to_bytes(2, byteorder='little', signed=False)  # message_size
		buffer += len(self._mosaics).to_bytes(1, byteorder='little', signed=False)  # mosaics_count
		buffer += self._transfer_transaction_body_reserved_1.to_bytes(1, byteorder='little', signed=False)
		buffer += self._transfer_transaction_body_reserved_2.to_bytes(4, byteorder='little', signed=False)
		buffer += ArrayHelpers.write_array(self._mosaics, lambda e: e.mosaic_id)
		buffer += self._message
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'type_: {self._type_.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'mosaics: {list(map(str, self._mosaics))}, '
		result += f'message: {hexlify(self._message).decode("utf8")}, '
		result += ')'
		return result


class TransactionFactory:
	@classmethod
	def deserialize(cls, payload: bytes) -> Transaction:
		buffer = bytes(payload)
		parent = Transaction.deserialize(buffer)
		mapping = {
			(AccountKeyLinkTransaction.TRANSACTION_TYPE): AccountKeyLinkTransaction,
			(NodeKeyLinkTransaction.TRANSACTION_TYPE): NodeKeyLinkTransaction,
			(AggregateCompleteTransaction.TRANSACTION_TYPE): AggregateCompleteTransaction,
			(AggregateBondedTransaction.TRANSACTION_TYPE): AggregateBondedTransaction,
			(VotingKeyLinkTransaction.TRANSACTION_TYPE): VotingKeyLinkTransaction,
			(VrfKeyLinkTransaction.TRANSACTION_TYPE): VrfKeyLinkTransaction,
			(HashLockTransaction.TRANSACTION_TYPE): HashLockTransaction,
			(SecretLockTransaction.TRANSACTION_TYPE): SecretLockTransaction,
			(SecretProofTransaction.TRANSACTION_TYPE): SecretProofTransaction,
			(AccountMetadataTransaction.TRANSACTION_TYPE): AccountMetadataTransaction,
			(MosaicMetadataTransaction.TRANSACTION_TYPE): MosaicMetadataTransaction,
			(NamespaceMetadataTransaction.TRANSACTION_TYPE): NamespaceMetadataTransaction,
			(MosaicDefinitionTransaction.TRANSACTION_TYPE): MosaicDefinitionTransaction,
			(MosaicSupplyChangeTransaction.TRANSACTION_TYPE): MosaicSupplyChangeTransaction,
			(MosaicSupplyRevocationTransaction.TRANSACTION_TYPE): MosaicSupplyRevocationTransaction,
			(MultisigAccountModificationTransaction.TRANSACTION_TYPE): MultisigAccountModificationTransaction,
			(AddressAliasTransaction.TRANSACTION_TYPE): AddressAliasTransaction,
			(MosaicAliasTransaction.TRANSACTION_TYPE): MosaicAliasTransaction,
			(NamespaceRegistrationTransaction.TRANSACTION_TYPE): NamespaceRegistrationTransaction,
			(AccountAddressRestrictionTransaction.TRANSACTION_TYPE): AccountAddressRestrictionTransaction,
			(AccountMosaicRestrictionTransaction.TRANSACTION_TYPE): AccountMosaicRestrictionTransaction,
			(AccountOperationRestrictionTransaction.TRANSACTION_TYPE): AccountOperationRestrictionTransaction,
			(MosaicAddressRestrictionTransaction.TRANSACTION_TYPE): MosaicAddressRestrictionTransaction,
			(MosaicGlobalRestrictionTransaction.TRANSACTION_TYPE): MosaicGlobalRestrictionTransaction,
			(TransferTransaction.TRANSACTION_TYPE): TransferTransaction
		}
		discriminator = (parent.type_)
		factory_class = mapping[discriminator]
		return factory_class.deserialize(buffer)

	@classmethod
	def create_by_name(cls, entity_name: str) -> Transaction:
		mapping = {
			'account_key_link_transaction': AccountKeyLinkTransaction,
			'node_key_link_transaction': NodeKeyLinkTransaction,
			'aggregate_complete_transaction': AggregateCompleteTransaction,
			'aggregate_bonded_transaction': AggregateBondedTransaction,
			'voting_key_link_transaction': VotingKeyLinkTransaction,
			'vrf_key_link_transaction': VrfKeyLinkTransaction,
			'hash_lock_transaction': HashLockTransaction,
			'secret_lock_transaction': SecretLockTransaction,
			'secret_proof_transaction': SecretProofTransaction,
			'account_metadata_transaction': AccountMetadataTransaction,
			'mosaic_metadata_transaction': MosaicMetadataTransaction,
			'namespace_metadata_transaction': NamespaceMetadataTransaction,
			'mosaic_definition_transaction': MosaicDefinitionTransaction,
			'mosaic_supply_change_transaction': MosaicSupplyChangeTransaction,
			'mosaic_supply_revocation_transaction': MosaicSupplyRevocationTransaction,
			'multisig_account_modification_transaction': MultisigAccountModificationTransaction,
			'address_alias_transaction': AddressAliasTransaction,
			'mosaic_alias_transaction': MosaicAliasTransaction,
			'namespace_registration_transaction': NamespaceRegistrationTransaction,
			'account_address_restriction_transaction': AccountAddressRestrictionTransaction,
			'account_mosaic_restriction_transaction': AccountMosaicRestrictionTransaction,
			'account_operation_restriction_transaction': AccountOperationRestrictionTransaction,
			'mosaic_address_restriction_transaction': MosaicAddressRestrictionTransaction,
			'mosaic_global_restriction_transaction': MosaicGlobalRestrictionTransaction,
			'transfer_transaction': TransferTransaction
		}

		if entity_name not in mapping:
			raise ValueError('unknown Transaction type')

		return mapping[entity_name]()


class EmbeddedTransactionFactory:
	@classmethod
	def deserialize(cls, payload: bytes) -> EmbeddedTransaction:
		buffer = bytes(payload)
		parent = EmbeddedTransaction.deserialize(buffer)
		mapping = {
			(EmbeddedAccountKeyLinkTransaction.TRANSACTION_TYPE): EmbeddedAccountKeyLinkTransaction,
			(EmbeddedNodeKeyLinkTransaction.TRANSACTION_TYPE): EmbeddedNodeKeyLinkTransaction,
			(EmbeddedVotingKeyLinkTransaction.TRANSACTION_TYPE): EmbeddedVotingKeyLinkTransaction,
			(EmbeddedVrfKeyLinkTransaction.TRANSACTION_TYPE): EmbeddedVrfKeyLinkTransaction,
			(EmbeddedHashLockTransaction.TRANSACTION_TYPE): EmbeddedHashLockTransaction,
			(EmbeddedSecretLockTransaction.TRANSACTION_TYPE): EmbeddedSecretLockTransaction,
			(EmbeddedSecretProofTransaction.TRANSACTION_TYPE): EmbeddedSecretProofTransaction,
			(EmbeddedAccountMetadataTransaction.TRANSACTION_TYPE): EmbeddedAccountMetadataTransaction,
			(EmbeddedMosaicMetadataTransaction.TRANSACTION_TYPE): EmbeddedMosaicMetadataTransaction,
			(EmbeddedNamespaceMetadataTransaction.TRANSACTION_TYPE): EmbeddedNamespaceMetadataTransaction,
			(EmbeddedMosaicDefinitionTransaction.TRANSACTION_TYPE): EmbeddedMosaicDefinitionTransaction,
			(EmbeddedMosaicSupplyChangeTransaction.TRANSACTION_TYPE): EmbeddedMosaicSupplyChangeTransaction,
			(EmbeddedMosaicSupplyRevocationTransaction.TRANSACTION_TYPE): EmbeddedMosaicSupplyRevocationTransaction,
			(EmbeddedMultisigAccountModificationTransaction.TRANSACTION_TYPE): EmbeddedMultisigAccountModificationTransaction,
			(EmbeddedAddressAliasTransaction.TRANSACTION_TYPE): EmbeddedAddressAliasTransaction,
			(EmbeddedMosaicAliasTransaction.TRANSACTION_TYPE): EmbeddedMosaicAliasTransaction,
			(EmbeddedNamespaceRegistrationTransaction.TRANSACTION_TYPE): EmbeddedNamespaceRegistrationTransaction,
			(EmbeddedAccountAddressRestrictionTransaction.TRANSACTION_TYPE): EmbeddedAccountAddressRestrictionTransaction,
			(EmbeddedAccountMosaicRestrictionTransaction.TRANSACTION_TYPE): EmbeddedAccountMosaicRestrictionTransaction,
			(EmbeddedAccountOperationRestrictionTransaction.TRANSACTION_TYPE): EmbeddedAccountOperationRestrictionTransaction,
			(EmbeddedMosaicAddressRestrictionTransaction.TRANSACTION_TYPE): EmbeddedMosaicAddressRestrictionTransaction,
			(EmbeddedMosaicGlobalRestrictionTransaction.TRANSACTION_TYPE): EmbeddedMosaicGlobalRestrictionTransaction,
			(EmbeddedTransferTransaction.TRANSACTION_TYPE): EmbeddedTransferTransaction
		}
		discriminator = (parent.type_)
		factory_class = mapping[discriminator]
		return factory_class.deserialize(buffer)

	@classmethod
	def create_by_name(cls, entity_name: str) -> EmbeddedTransaction:
		mapping = {
			'account_key_link_transaction': EmbeddedAccountKeyLinkTransaction,
			'node_key_link_transaction': EmbeddedNodeKeyLinkTransaction,
			'voting_key_link_transaction': EmbeddedVotingKeyLinkTransaction,
			'vrf_key_link_transaction': EmbeddedVrfKeyLinkTransaction,
			'hash_lock_transaction': EmbeddedHashLockTransaction,
			'secret_lock_transaction': EmbeddedSecretLockTransaction,
			'secret_proof_transaction': EmbeddedSecretProofTransaction,
			'account_metadata_transaction': EmbeddedAccountMetadataTransaction,
			'mosaic_metadata_transaction': EmbeddedMosaicMetadataTransaction,
			'namespace_metadata_transaction': EmbeddedNamespaceMetadataTransaction,
			'mosaic_definition_transaction': EmbeddedMosaicDefinitionTransaction,
			'mosaic_supply_change_transaction': EmbeddedMosaicSupplyChangeTransaction,
			'mosaic_supply_revocation_transaction': EmbeddedMosaicSupplyRevocationTransaction,
			'multisig_account_modification_transaction': EmbeddedMultisigAccountModificationTransaction,
			'address_alias_transaction': EmbeddedAddressAliasTransaction,
			'mosaic_alias_transaction': EmbeddedMosaicAliasTransaction,
			'namespace_registration_transaction': EmbeddedNamespaceRegistrationTransaction,
			'account_address_restriction_transaction': EmbeddedAccountAddressRestrictionTransaction,
			'account_mosaic_restriction_transaction': EmbeddedAccountMosaicRestrictionTransaction,
			'account_operation_restriction_transaction': EmbeddedAccountOperationRestrictionTransaction,
			'mosaic_address_restriction_transaction': EmbeddedMosaicAddressRestrictionTransaction,
			'mosaic_global_restriction_transaction': EmbeddedMosaicGlobalRestrictionTransaction,
			'transfer_transaction': EmbeddedTransferTransaction
		}

		if entity_name not in mapping:
			raise ValueError('unknown EmbeddedTransaction type')

		return mapping[entity_name]()
