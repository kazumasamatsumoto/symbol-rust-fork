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


class Timestamp(BaseValue):
	SIZE = 4

	def __init__(self, timestamp: int = 0):
		super().__init__(self.SIZE, timestamp, Timestamp)

	@classmethod
	def deserialize(cls, payload: ByteString) -> Timestamp:
		buffer = memoryview(payload)
		return Timestamp(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		return self.value.to_bytes(4, byteorder='little', signed=False)


class Address(ByteArray):
	SIZE = 40

	def __init__(self, address: StrBytes = bytes(40)):
		super().__init__(self.SIZE, address, Address)

	@property
	def size(self) -> int:
		return 40

	@classmethod
	def deserialize(cls, payload: ByteString) -> Address:
		buffer = memoryview(payload)
		return Address(ArrayHelpers.get_bytes(buffer, 40))

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
	TRANSFER = 257
	ACCOUNT_KEY_LINK = 2049
	MULTISIG_ACCOUNT_MODIFICATION = 4097
	MULTISIG_COSIGNATURE = 4098
	MULTISIG_TRANSACTION = 4100
	NAMESPACE_REGISTRATION = 8193
	MOSAIC_DEFINITION = 16385
	MOSAIC_SUPPLY_CHANGE = 16386

	@property
	def size(self) -> int:
		return 4

	@classmethod
	def deserialize(cls, payload: ByteString) -> TransactionType:
		buffer = memoryview(payload)
		return TransactionType(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(4, byteorder='little', signed=False)
		return buffer


class Transaction:
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp'
	}

	def __init__(self):
		self._type_ = TransactionType.TRANSFER
		self._version = 0
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Transaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]

		instance = Transaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += ')'
		return result


class NonVerifiableTransaction:
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp'
	}

	def __init__(self):
		self._type_ = TransactionType.TRANSFER
		self._version = 0
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]

		instance = NonVerifiableTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += ')'
		return result


class LinkAction(Enum):
	LINK = 1
	UNLINK = 2

	@property
	def size(self) -> int:
		return 4

	@classmethod
	def deserialize(cls, payload: ByteString) -> LinkAction:
		buffer = memoryview(payload)
		return LinkAction(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(4, byteorder='little', signed=False)
		return buffer


class AccountKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_KEY_LINK
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'link_action': 'enum:LinkAction',
		'remote_public_key': 'pod:PublicKey'
	}

	def __init__(self):
		self._type_ = AccountKeyLinkTransaction.TRANSACTION_TYPE
		self._version = AccountKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._link_action = LinkAction.LINK
		self._remote_public_key = PublicKey()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._remote_public_key_size = 32  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@property
	def remote_public_key(self) -> PublicKey:
		return self._remote_public_key

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@remote_public_key.setter
	def remote_public_key(self, value: PublicKey):
		self._remote_public_key = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += self.link_action.size
		size += 4
		size += self.remote_public_key.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> AccountKeyLinkTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]
		remote_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert remote_public_key_size == 32, f'Invalid value of reserved field ({remote_public_key_size})'
		remote_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[remote_public_key.size:]

		instance = AccountKeyLinkTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._link_action = link_action
		instance._remote_public_key = remote_public_key
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._link_action.serialize()
		buffer += self._remote_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._remote_public_key.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += f'remote_public_key: {self._remote_public_key.__str__()}, '
		result += ')'
		return result


class NonVerifiableAccountKeyLinkTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.ACCOUNT_KEY_LINK
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'link_action': 'enum:LinkAction',
		'remote_public_key': 'pod:PublicKey'
	}

	def __init__(self):
		self._type_ = NonVerifiableAccountKeyLinkTransaction.TRANSACTION_TYPE
		self._version = NonVerifiableAccountKeyLinkTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._link_action = LinkAction.LINK
		self._remote_public_key = PublicKey()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._remote_public_key_size = 32  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def link_action(self) -> LinkAction:
		return self._link_action

	@property
	def remote_public_key(self) -> PublicKey:
		return self._remote_public_key

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@link_action.setter
	def link_action(self, value: LinkAction):
		self._link_action = value

	@remote_public_key.setter
	def remote_public_key(self, value: PublicKey):
		self._remote_public_key = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += self.link_action.size
		size += 4
		size += self.remote_public_key.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableAccountKeyLinkTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		link_action = LinkAction.deserialize(buffer)
		buffer = buffer[link_action.size:]
		remote_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert remote_public_key_size == 32, f'Invalid value of reserved field ({remote_public_key_size})'
		remote_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[remote_public_key.size:]

		instance = NonVerifiableAccountKeyLinkTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._link_action = link_action
		instance._remote_public_key = remote_public_key
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._link_action.serialize()
		buffer += self._remote_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._remote_public_key.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'link_action: {self._link_action.__str__()}, '
		result += f'remote_public_key: {self._remote_public_key.__str__()}, '
		result += ')'
		return result


class NamespaceId:
	TYPE_HINTS = {
		'name': 'bytes_array'
	}

	def __init__(self):
		self._name = bytes()

	@property
	def name(self) -> bytes:
		return self._name

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += len(self._name)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NamespaceId:
		buffer = memoryview(payload)
		name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]

		instance = NamespaceId()
		instance._name = name
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += len(self._name).to_bytes(4, byteorder='little', signed=False)  # name_size
		buffer += self._name
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		result += ')'
		return result


class MosaicId:
	TYPE_HINTS = {
		'namespace_id': 'struct:NamespaceId',
		'name': 'bytes_array'
	}

	def __init__(self):
		self._namespace_id = NamespaceId()
		self._name = bytes()

	@property
	def namespace_id(self) -> NamespaceId:
		return self._namespace_id

	@property
	def name(self) -> bytes:
		return self._name

	@namespace_id.setter
	def namespace_id(self, value: NamespaceId):
		self._namespace_id = value

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@property
	def size(self) -> int:
		size = 0
		size += self.namespace_id.size
		size += 4
		size += len(self._name)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicId:
		buffer = memoryview(payload)
		namespace_id = NamespaceId.deserialize(buffer)
		buffer = buffer[namespace_id.size:]
		name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]

		instance = MosaicId()
		instance._namespace_id = namespace_id
		instance._name = name
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._namespace_id.serialize()
		buffer += len(self._name).to_bytes(4, byteorder='little', signed=False)  # name_size
		buffer += self._name
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'namespace_id: {self._namespace_id.__str__()}, '
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		result += ')'
		return result


class Mosaic:
	TYPE_HINTS = {
		'mosaic_id': 'struct:MosaicId',
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
		size += 4
		size += self.mosaic_id.size
		size += self.amount.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Mosaic:
		buffer = memoryview(payload)
		mosaic_id_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic_id = MosaicId.deserialize(buffer[:mosaic_id_size])
		buffer = buffer[mosaic_id.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]

		instance = Mosaic()
		instance._mosaic_id = mosaic_id
		instance._amount = amount
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.mosaic_id.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_id_size
		buffer += self._mosaic_id.serialize()
		buffer += self._amount.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += ')'
		return result


class SizePrefixedMosaic:
	TYPE_HINTS = {
		'mosaic': 'struct:Mosaic'
	}

	def __init__(self):
		self._mosaic = Mosaic()

	@property
	def mosaic(self) -> Mosaic:
		return self._mosaic

	@mosaic.setter
	def mosaic(self, value: Mosaic):
		self._mosaic = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += self.mosaic.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> SizePrefixedMosaic:
		buffer = memoryview(payload)
		mosaic_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic = Mosaic.deserialize(buffer[:mosaic_size])
		buffer = buffer[mosaic.size:]

		instance = SizePrefixedMosaic()
		instance._mosaic = mosaic
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.mosaic.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_size
		buffer += self._mosaic.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'mosaic: {self._mosaic.__str__()}, '
		result += ')'
		return result


class MosaicTransferFeeType(Enum):
	ABSOLUTE = 1
	PERCENTILE = 2

	@property
	def size(self) -> int:
		return 4

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicTransferFeeType:
		buffer = memoryview(payload)
		return MosaicTransferFeeType(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(4, byteorder='little', signed=False)
		return buffer


class MosaicLevy:
	TYPE_HINTS = {
		'transfer_fee_type': 'enum:MosaicTransferFeeType',
		'recipient_address': 'pod:Address',
		'mosaic_id': 'struct:MosaicId',
		'fee': 'pod:Amount'
	}

	def __init__(self):
		self._transfer_fee_type = MosaicTransferFeeType.ABSOLUTE
		self._recipient_address = Address()
		self._mosaic_id = MosaicId()
		self._fee = Amount()
		self._recipient_address_size = 40  # reserved field

	@property
	def transfer_fee_type(self) -> MosaicTransferFeeType:
		return self._transfer_fee_type

	@property
	def recipient_address(self) -> Address:
		return self._recipient_address

	@property
	def mosaic_id(self) -> MosaicId:
		return self._mosaic_id

	@property
	def fee(self) -> Amount:
		return self._fee

	@transfer_fee_type.setter
	def transfer_fee_type(self, value: MosaicTransferFeeType):
		self._transfer_fee_type = value

	@recipient_address.setter
	def recipient_address(self, value: Address):
		self._recipient_address = value

	@mosaic_id.setter
	def mosaic_id(self, value: MosaicId):
		self._mosaic_id = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@property
	def size(self) -> int:
		size = 0
		size += self.transfer_fee_type.size
		size += 4
		size += self.recipient_address.size
		size += 4
		size += self.mosaic_id.size
		size += self.fee.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicLevy:
		buffer = memoryview(payload)
		transfer_fee_type = MosaicTransferFeeType.deserialize(buffer)
		buffer = buffer[transfer_fee_type.size:]
		recipient_address_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert recipient_address_size == 40, f'Invalid value of reserved field ({recipient_address_size})'
		recipient_address = Address.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		mosaic_id_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic_id = MosaicId.deserialize(buffer[:mosaic_id_size])
		buffer = buffer[mosaic_id.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]

		instance = MosaicLevy()
		instance._transfer_fee_type = transfer_fee_type
		instance._recipient_address = recipient_address
		instance._mosaic_id = mosaic_id
		instance._fee = fee
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._transfer_fee_type.serialize()
		buffer += self._recipient_address_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._recipient_address.serialize()
		buffer += self.mosaic_id.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_id_size
		buffer += self._mosaic_id.serialize()
		buffer += self._fee.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'transfer_fee_type: {self._transfer_fee_type.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += ')'
		return result


class MosaicProperty:
	TYPE_HINTS = {
		'name': 'bytes_array',
		'value': 'bytes_array'
	}

	def __init__(self):
		self._name = bytes()
		self._value = bytes()

	@property
	def name(self) -> bytes:
		return self._name

	@property
	def value(self) -> bytes:
		return self._value

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@value.setter
	def value(self, value: bytes):
		self._value = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += len(self._name)
		size += 4
		size += len(self._value)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicProperty:
		buffer = memoryview(payload)
		name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]
		value_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		value = ArrayHelpers.get_bytes(buffer, value_size)
		buffer = buffer[value_size:]

		instance = MosaicProperty()
		instance._name = name
		instance._value = value
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += len(self._name).to_bytes(4, byteorder='little', signed=False)  # name_size
		buffer += self._name
		buffer += len(self._value).to_bytes(4, byteorder='little', signed=False)  # value_size
		buffer += self._value
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		result += f'value: {hexlify(self._value).decode("utf8")}, '
		result += ')'
		return result


class SizePrefixedMosaicProperty:
	TYPE_HINTS = {
		'property_': 'struct:MosaicProperty'
	}

	def __init__(self):
		self._property_ = MosaicProperty()

	@property
	def property_(self) -> MosaicProperty:
		return self._property_

	@property_.setter
	def property_(self, value: MosaicProperty):
		self._property_ = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += self.property_.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> SizePrefixedMosaicProperty:
		buffer = memoryview(payload)
		property_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		property_ = MosaicProperty.deserialize(buffer[:property_size])
		buffer = buffer[property_.size:]

		instance = SizePrefixedMosaicProperty()
		instance._property_ = property_
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.property_.size.to_bytes(4, byteorder='little', signed=False)  # property_size
		buffer += self._property_.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'property_: {self._property_.__str__()}, '
		result += ')'
		return result


class MosaicDefinition:
	TYPE_HINTS = {
		'owner_public_key': 'pod:PublicKey',
		'id': 'struct:MosaicId',
		'description': 'bytes_array',
		'properties': 'array[SizePrefixedMosaicProperty]',
		'levy': 'struct:MosaicLevy'
	}

	def __init__(self):
		self._owner_public_key = PublicKey()
		self._id = MosaicId()
		self._description = bytes()
		self._properties = []
		self._levy_size = 0
		self._levy = MosaicLevy()
		self._owner_public_key_size = 32  # reserved field

	@property
	def owner_public_key(self) -> PublicKey:
		return self._owner_public_key

	@property
	def id(self) -> MosaicId:
		return self._id

	@property
	def description(self) -> bytes:
		return self._description

	@property
	def properties(self) -> List[SizePrefixedMosaicProperty]:
		return self._properties

	@property
	def levy_size(self) -> int:
		return self._levy_size

	@property
	def levy(self) -> MosaicLevy:
		return self._levy

	@owner_public_key.setter
	def owner_public_key(self, value: PublicKey):
		self._owner_public_key = value

	@id.setter
	def id(self, value: MosaicId):
		self._id = value

	@description.setter
	def description(self, value: bytes):
		self._description = value

	@properties.setter
	def properties(self, value: List[SizePrefixedMosaicProperty]):
		self._properties = value

	@levy_size.setter
	def levy_size(self, value: int):
		self._levy_size = value

	@levy.setter
	def levy(self, value: MosaicLevy):
		self._levy = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += self.owner_public_key.size
		size += 4
		size += self.id.size
		size += 4
		size += len(self._description)
		size += 4
		size += ArrayHelpers.size(self.properties)
		size += 4
		if 0 != self.levy_size:
			size += self.levy.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicDefinition:
		buffer = memoryview(payload)
		owner_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert owner_public_key_size == 32, f'Invalid value of reserved field ({owner_public_key_size})'
		owner_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[owner_public_key.size:]
		id_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		id = MosaicId.deserialize(buffer[:id_size])
		buffer = buffer[id.size:]
		description_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		description = ArrayHelpers.get_bytes(buffer, description_size)
		buffer = buffer[description_size:]
		properties_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		properties = ArrayHelpers.read_array_count(buffer, SizePrefixedMosaicProperty, properties_count)
		buffer = buffer[ArrayHelpers.size(properties):]
		levy_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		levy = None
		if 0 != levy_size:
			levy = MosaicLevy.deserialize(buffer)
			buffer = buffer[levy.size:]

		instance = MosaicDefinition()
		instance._owner_public_key = owner_public_key
		instance._id = id
		instance._description = description
		instance._properties = properties
		instance._levy_size = levy_size
		instance._levy = levy
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._owner_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._owner_public_key.serialize()
		buffer += self.id.size.to_bytes(4, byteorder='little', signed=False)  # id_size
		buffer += self._id.serialize()
		buffer += len(self._description).to_bytes(4, byteorder='little', signed=False)  # description_size
		buffer += self._description
		buffer += len(self._properties).to_bytes(4, byteorder='little', signed=False)  # properties_count
		buffer += ArrayHelpers.write_array(self._properties)
		buffer += self._levy_size.to_bytes(4, byteorder='little', signed=False)
		if 0 != self.levy_size:
			buffer += self._levy.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'owner_public_key: {self._owner_public_key.__str__()}, '
		result += f'id: {self._id.__str__()}, '
		result += f'description: {hexlify(self._description).decode("utf8")}, '
		result += f'properties: {list(map(str, self._properties))}, '
		result += f'levy_size: 0x{self._levy_size:X}, '
		if 0 != self.levy_size:
			result += f'levy: {self._levy.__str__()}, '
		result += ')'
		return result


class MosaicDefinitionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_DEFINITION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_definition': 'struct:MosaicDefinition',
		'rental_fee_sink': 'pod:Address',
		'rental_fee': 'pod:Amount'
	}

	def __init__(self):
		self._type_ = MosaicDefinitionTransaction.TRANSACTION_TYPE
		self._version = MosaicDefinitionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_definition = MosaicDefinition()
		self._rental_fee_sink = Address()
		self._rental_fee = Amount()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._rental_fee_sink_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_definition(self) -> MosaicDefinition:
		return self._mosaic_definition

	@property
	def rental_fee_sink(self) -> Address:
		return self._rental_fee_sink

	@property
	def rental_fee(self) -> Amount:
		return self._rental_fee

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_definition.setter
	def mosaic_definition(self, value: MosaicDefinition):
		self._mosaic_definition = value

	@rental_fee_sink.setter
	def rental_fee_sink(self, value: Address):
		self._rental_fee_sink = value

	@rental_fee.setter
	def rental_fee(self, value: Amount):
		self._rental_fee = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.mosaic_definition.size
		size += 4
		size += self.rental_fee_sink.size
		size += self.rental_fee.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicDefinitionTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_definition_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic_definition = MosaicDefinition.deserialize(buffer[:mosaic_definition_size])
		buffer = buffer[mosaic_definition.size:]
		rental_fee_sink_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert rental_fee_sink_size == 40, f'Invalid value of reserved field ({rental_fee_sink_size})'
		rental_fee_sink = Address.deserialize(buffer)
		buffer = buffer[rental_fee_sink.size:]
		rental_fee = Amount.deserialize(buffer)
		buffer = buffer[rental_fee.size:]

		instance = MosaicDefinitionTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_definition = mosaic_definition
		instance._rental_fee_sink = rental_fee_sink
		instance._rental_fee = rental_fee
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self.mosaic_definition.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_definition_size
		buffer += self._mosaic_definition.serialize()
		buffer += self._rental_fee_sink_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._rental_fee_sink.serialize()
		buffer += self._rental_fee.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_definition: {self._mosaic_definition.__str__()}, '
		result += f'rental_fee_sink: {self._rental_fee_sink.__str__()}, '
		result += f'rental_fee: {self._rental_fee.__str__()}, '
		result += ')'
		return result


class NonVerifiableMosaicDefinitionTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_DEFINITION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_definition': 'struct:MosaicDefinition',
		'rental_fee_sink': 'pod:Address',
		'rental_fee': 'pod:Amount'
	}

	def __init__(self):
		self._type_ = NonVerifiableMosaicDefinitionTransaction.TRANSACTION_TYPE
		self._version = NonVerifiableMosaicDefinitionTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_definition = MosaicDefinition()
		self._rental_fee_sink = Address()
		self._rental_fee = Amount()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._rental_fee_sink_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_definition(self) -> MosaicDefinition:
		return self._mosaic_definition

	@property
	def rental_fee_sink(self) -> Address:
		return self._rental_fee_sink

	@property
	def rental_fee(self) -> Amount:
		return self._rental_fee

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_definition.setter
	def mosaic_definition(self, value: MosaicDefinition):
		self._mosaic_definition = value

	@rental_fee_sink.setter
	def rental_fee_sink(self, value: Address):
		self._rental_fee_sink = value

	@rental_fee.setter
	def rental_fee(self, value: Amount):
		self._rental_fee = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.mosaic_definition.size
		size += 4
		size += self.rental_fee_sink.size
		size += self.rental_fee.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableMosaicDefinitionTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_definition_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic_definition = MosaicDefinition.deserialize(buffer[:mosaic_definition_size])
		buffer = buffer[mosaic_definition.size:]
		rental_fee_sink_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert rental_fee_sink_size == 40, f'Invalid value of reserved field ({rental_fee_sink_size})'
		rental_fee_sink = Address.deserialize(buffer)
		buffer = buffer[rental_fee_sink.size:]
		rental_fee = Amount.deserialize(buffer)
		buffer = buffer[rental_fee.size:]

		instance = NonVerifiableMosaicDefinitionTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_definition = mosaic_definition
		instance._rental_fee_sink = rental_fee_sink
		instance._rental_fee = rental_fee
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self.mosaic_definition.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_definition_size
		buffer += self._mosaic_definition.serialize()
		buffer += self._rental_fee_sink_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._rental_fee_sink.serialize()
		buffer += self._rental_fee.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_definition: {self._mosaic_definition.__str__()}, '
		result += f'rental_fee_sink: {self._rental_fee_sink.__str__()}, '
		result += f'rental_fee: {self._rental_fee.__str__()}, '
		result += ')'
		return result


class MosaicSupplyChangeAction(Enum):
	INCREASE = 1
	DECREASE = 2

	@property
	def size(self) -> int:
		return 4

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicSupplyChangeAction:
		buffer = memoryview(payload)
		return MosaicSupplyChangeAction(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(4, byteorder='little', signed=False)
		return buffer


class MosaicSupplyChangeTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_SUPPLY_CHANGE
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_id': 'struct:MosaicId',
		'action': 'enum:MosaicSupplyChangeAction',
		'delta': 'pod:Amount'
	}

	def __init__(self):
		self._type_ = MosaicSupplyChangeTransaction.TRANSACTION_TYPE
		self._version = MosaicSupplyChangeTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_id = MosaicId()
		self._action = MosaicSupplyChangeAction.INCREASE
		self._delta = Amount()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_id(self) -> MosaicId:
		return self._mosaic_id

	@property
	def action(self) -> MosaicSupplyChangeAction:
		return self._action

	@property
	def delta(self) -> Amount:
		return self._delta

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_id.setter
	def mosaic_id(self, value: MosaicId):
		self._mosaic_id = value

	@action.setter
	def action(self, value: MosaicSupplyChangeAction):
		self._action = value

	@delta.setter
	def delta(self, value: Amount):
		self._delta = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.mosaic_id.size
		size += self.action.size
		size += self.delta.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MosaicSupplyChangeTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_id_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic_id = MosaicId.deserialize(buffer[:mosaic_id_size])
		buffer = buffer[mosaic_id.size:]
		action = MosaicSupplyChangeAction.deserialize(buffer)
		buffer = buffer[action.size:]
		delta = Amount.deserialize(buffer)
		buffer = buffer[delta.size:]

		instance = MosaicSupplyChangeTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_id = mosaic_id
		instance._action = action
		instance._delta = delta
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self.mosaic_id.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_id_size
		buffer += self._mosaic_id.serialize()
		buffer += self._action.serialize()
		buffer += self._delta.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'action: {self._action.__str__()}, '
		result += f'delta: {self._delta.__str__()}, '
		result += ')'
		return result


class NonVerifiableMosaicSupplyChangeTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MOSAIC_SUPPLY_CHANGE
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'mosaic_id': 'struct:MosaicId',
		'action': 'enum:MosaicSupplyChangeAction',
		'delta': 'pod:Amount'
	}

	def __init__(self):
		self._type_ = NonVerifiableMosaicSupplyChangeTransaction.TRANSACTION_TYPE
		self._version = NonVerifiableMosaicSupplyChangeTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._mosaic_id = MosaicId()
		self._action = MosaicSupplyChangeAction.INCREASE
		self._delta = Amount()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def mosaic_id(self) -> MosaicId:
		return self._mosaic_id

	@property
	def action(self) -> MosaicSupplyChangeAction:
		return self._action

	@property
	def delta(self) -> Amount:
		return self._delta

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@mosaic_id.setter
	def mosaic_id(self, value: MosaicId):
		self._mosaic_id = value

	@action.setter
	def action(self, value: MosaicSupplyChangeAction):
		self._action = value

	@delta.setter
	def delta(self, value: Amount):
		self._delta = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.mosaic_id.size
		size += self.action.size
		size += self.delta.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableMosaicSupplyChangeTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		mosaic_id_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		mosaic_id = MosaicId.deserialize(buffer[:mosaic_id_size])
		buffer = buffer[mosaic_id.size:]
		action = MosaicSupplyChangeAction.deserialize(buffer)
		buffer = buffer[action.size:]
		delta = Amount.deserialize(buffer)
		buffer = buffer[delta.size:]

		instance = NonVerifiableMosaicSupplyChangeTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._mosaic_id = mosaic_id
		instance._action = action
		instance._delta = delta
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self.mosaic_id.size.to_bytes(4, byteorder='little', signed=False)  # mosaic_id_size
		buffer += self._mosaic_id.serialize()
		buffer += self._action.serialize()
		buffer += self._delta.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'mosaic_id: {self._mosaic_id.__str__()}, '
		result += f'action: {self._action.__str__()}, '
		result += f'delta: {self._delta.__str__()}, '
		result += ')'
		return result


class MultisigAccountModificationType(Enum):
	ADD_COSIGNATORY = 1
	DELETE_COSIGNATORY = 2

	@property
	def size(self) -> int:
		return 4

	@classmethod
	def deserialize(cls, payload: ByteString) -> MultisigAccountModificationType:
		buffer = memoryview(payload)
		return MultisigAccountModificationType(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(4, byteorder='little', signed=False)
		return buffer


class MultisigAccountModification:
	TYPE_HINTS = {
		'modification_type': 'enum:MultisigAccountModificationType',
		'cosignatory_public_key': 'pod:PublicKey'
	}

	def __init__(self):
		self._modification_type = MultisigAccountModificationType.ADD_COSIGNATORY
		self._cosignatory_public_key = PublicKey()
		self._cosignatory_public_key_size = 32  # reserved field

	@property
	def modification_type(self) -> MultisigAccountModificationType:
		return self._modification_type

	@property
	def cosignatory_public_key(self) -> PublicKey:
		return self._cosignatory_public_key

	@modification_type.setter
	def modification_type(self, value: MultisigAccountModificationType):
		self._modification_type = value

	@cosignatory_public_key.setter
	def cosignatory_public_key(self, value: PublicKey):
		self._cosignatory_public_key = value

	@property
	def size(self) -> int:
		size = 0
		size += self.modification_type.size
		size += 4
		size += self.cosignatory_public_key.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MultisigAccountModification:
		buffer = memoryview(payload)
		modification_type = MultisigAccountModificationType.deserialize(buffer)
		buffer = buffer[modification_type.size:]
		cosignatory_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert cosignatory_public_key_size == 32, f'Invalid value of reserved field ({cosignatory_public_key_size})'
		cosignatory_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[cosignatory_public_key.size:]

		instance = MultisigAccountModification()
		instance._modification_type = modification_type
		instance._cosignatory_public_key = cosignatory_public_key
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._modification_type.serialize()
		buffer += self._cosignatory_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._cosignatory_public_key.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'modification_type: {self._modification_type.__str__()}, '
		result += f'cosignatory_public_key: {self._cosignatory_public_key.__str__()}, '
		result += ')'
		return result


class SizePrefixedMultisigAccountModification:
	TYPE_HINTS = {
		'modification': 'struct:MultisigAccountModification'
	}

	def __init__(self):
		self._modification = MultisigAccountModification()

	@property
	def modification(self) -> MultisigAccountModification:
		return self._modification

	@modification.setter
	def modification(self, value: MultisigAccountModification):
		self._modification = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += self.modification.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> SizePrefixedMultisigAccountModification:
		buffer = memoryview(payload)
		modification_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		modification = MultisigAccountModification.deserialize(buffer[:modification_size])
		buffer = buffer[modification.size:]

		instance = SizePrefixedMultisigAccountModification()
		instance._modification = modification
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.modification.size.to_bytes(4, byteorder='little', signed=False)  # modification_size
		buffer += self._modification.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'modification: {self._modification.__str__()}, '
		result += ')'
		return result


class MultisigAccountModificationTransactionV1:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_ACCOUNT_MODIFICATION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'modifications': 'array[SizePrefixedMultisigAccountModification]'
	}

	def __init__(self):
		self._type_ = MultisigAccountModificationTransactionV1.TRANSACTION_TYPE
		self._version = MultisigAccountModificationTransactionV1.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._modifications = []
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def modifications(self) -> List[SizePrefixedMultisigAccountModification]:
		return self._modifications

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@modifications.setter
	def modifications(self, value: List[SizePrefixedMultisigAccountModification]):
		self._modifications = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += ArrayHelpers.size(self.modifications)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MultisigAccountModificationTransactionV1:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		modifications_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		modifications = ArrayHelpers.read_array_count(buffer, SizePrefixedMultisigAccountModification, modifications_count)
		buffer = buffer[ArrayHelpers.size(modifications):]

		instance = MultisigAccountModificationTransactionV1()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._modifications = modifications
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += len(self._modifications).to_bytes(4, byteorder='little', signed=False)  # modifications_count
		buffer += ArrayHelpers.write_array(self._modifications)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'modifications: {list(map(str, self._modifications))}, '
		result += ')'
		return result


class NonVerifiableMultisigAccountModificationTransactionV1:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_ACCOUNT_MODIFICATION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'modifications': 'array[SizePrefixedMultisigAccountModification]'
	}

	def __init__(self):
		self._type_ = NonVerifiableMultisigAccountModificationTransactionV1.TRANSACTION_TYPE
		self._version = NonVerifiableMultisigAccountModificationTransactionV1.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._modifications = []
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def modifications(self) -> List[SizePrefixedMultisigAccountModification]:
		return self._modifications

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@modifications.setter
	def modifications(self, value: List[SizePrefixedMultisigAccountModification]):
		self._modifications = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += ArrayHelpers.size(self.modifications)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableMultisigAccountModificationTransactionV1:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		modifications_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		modifications = ArrayHelpers.read_array_count(buffer, SizePrefixedMultisigAccountModification, modifications_count)
		buffer = buffer[ArrayHelpers.size(modifications):]

		instance = NonVerifiableMultisigAccountModificationTransactionV1()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._modifications = modifications
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += len(self._modifications).to_bytes(4, byteorder='little', signed=False)  # modifications_count
		buffer += ArrayHelpers.write_array(self._modifications)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'modifications: {list(map(str, self._modifications))}, '
		result += ')'
		return result


class MultisigAccountModificationTransaction:
	TRANSACTION_VERSION: int = 2
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_ACCOUNT_MODIFICATION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'modifications': 'array[SizePrefixedMultisigAccountModification]'
	}

	def __init__(self):
		self._type_ = MultisigAccountModificationTransaction.TRANSACTION_TYPE
		self._version = MultisigAccountModificationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._modifications = []
		self._min_approval_delta = 0
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._min_approval_delta_size = 4  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def modifications(self) -> List[SizePrefixedMultisigAccountModification]:
		return self._modifications

	@property
	def min_approval_delta(self) -> int:
		return self._min_approval_delta

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@modifications.setter
	def modifications(self, value: List[SizePrefixedMultisigAccountModification]):
		self._modifications = value

	@min_approval_delta.setter
	def min_approval_delta(self, value: int):
		self._min_approval_delta = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += ArrayHelpers.size(self.modifications)
		size += 4
		size += 4
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MultisigAccountModificationTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		modifications_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		modifications = ArrayHelpers.read_array_count(buffer, SizePrefixedMultisigAccountModification, modifications_count)
		buffer = buffer[ArrayHelpers.size(modifications):]
		min_approval_delta_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert min_approval_delta_size == 4, f'Invalid value of reserved field ({min_approval_delta_size})'
		min_approval_delta = int.from_bytes(buffer[:4], byteorder='little', signed=True)
		buffer = buffer[4:]

		instance = MultisigAccountModificationTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._modifications = modifications
		instance._min_approval_delta = min_approval_delta
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += len(self._modifications).to_bytes(4, byteorder='little', signed=False)  # modifications_count
		buffer += ArrayHelpers.write_array(self._modifications)
		buffer += self._min_approval_delta_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._min_approval_delta.to_bytes(4, byteorder='little', signed=True)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'modifications: {list(map(str, self._modifications))}, '
		result += f'min_approval_delta: 0x{self._min_approval_delta:X}, '
		result += ')'
		return result


class NonVerifiableMultisigAccountModificationTransaction:
	TRANSACTION_VERSION: int = 2
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_ACCOUNT_MODIFICATION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'modifications': 'array[SizePrefixedMultisigAccountModification]'
	}

	def __init__(self):
		self._type_ = NonVerifiableMultisigAccountModificationTransaction.TRANSACTION_TYPE
		self._version = NonVerifiableMultisigAccountModificationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._modifications = []
		self._min_approval_delta = 0
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._min_approval_delta_size = 4  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def modifications(self) -> List[SizePrefixedMultisigAccountModification]:
		return self._modifications

	@property
	def min_approval_delta(self) -> int:
		return self._min_approval_delta

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@modifications.setter
	def modifications(self, value: List[SizePrefixedMultisigAccountModification]):
		self._modifications = value

	@min_approval_delta.setter
	def min_approval_delta(self, value: int):
		self._min_approval_delta = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += ArrayHelpers.size(self.modifications)
		size += 4
		size += 4
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableMultisigAccountModificationTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		modifications_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		modifications = ArrayHelpers.read_array_count(buffer, SizePrefixedMultisigAccountModification, modifications_count)
		buffer = buffer[ArrayHelpers.size(modifications):]
		min_approval_delta_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert min_approval_delta_size == 4, f'Invalid value of reserved field ({min_approval_delta_size})'
		min_approval_delta = int.from_bytes(buffer[:4], byteorder='little', signed=True)
		buffer = buffer[4:]

		instance = NonVerifiableMultisigAccountModificationTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._modifications = modifications
		instance._min_approval_delta = min_approval_delta
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += len(self._modifications).to_bytes(4, byteorder='little', signed=False)  # modifications_count
		buffer += ArrayHelpers.write_array(self._modifications)
		buffer += self._min_approval_delta_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._min_approval_delta.to_bytes(4, byteorder='little', signed=True)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'modifications: {list(map(str, self._modifications))}, '
		result += f'min_approval_delta: 0x{self._min_approval_delta:X}, '
		result += ')'
		return result


class Cosignature:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_COSIGNATURE
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'multisig_transaction_hash': 'pod:Hash256',
		'multisig_account_address': 'pod:Address'
	}

	def __init__(self):
		self._type_ = Cosignature.TRANSACTION_TYPE
		self._version = Cosignature.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._multisig_transaction_hash = Hash256()
		self._multisig_account_address = Address()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._multisig_transaction_hash_outer_size = 36  # reserved field
		self._multisig_transaction_hash_size = 32  # reserved field
		self._multisig_account_address_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def multisig_transaction_hash(self) -> Hash256:
		return self._multisig_transaction_hash

	@property
	def multisig_account_address(self) -> Address:
		return self._multisig_account_address

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@multisig_transaction_hash.setter
	def multisig_transaction_hash(self, value: Hash256):
		self._multisig_transaction_hash = value

	@multisig_account_address.setter
	def multisig_account_address(self, value: Address):
		self._multisig_account_address = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += 4
		size += self.multisig_transaction_hash.size
		size += 4
		size += self.multisig_account_address.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Cosignature:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		multisig_transaction_hash_outer_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert multisig_transaction_hash_outer_size == 36, f'Invalid value of reserved field ({multisig_transaction_hash_outer_size})'
		multisig_transaction_hash_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert multisig_transaction_hash_size == 32, f'Invalid value of reserved field ({multisig_transaction_hash_size})'
		multisig_transaction_hash = Hash256.deserialize(buffer)
		buffer = buffer[multisig_transaction_hash.size:]
		multisig_account_address_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert multisig_account_address_size == 40, f'Invalid value of reserved field ({multisig_account_address_size})'
		multisig_account_address = Address.deserialize(buffer)
		buffer = buffer[multisig_account_address.size:]

		instance = Cosignature()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._multisig_transaction_hash = multisig_transaction_hash
		instance._multisig_account_address = multisig_account_address
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._multisig_transaction_hash_outer_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._multisig_transaction_hash_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._multisig_transaction_hash.serialize()
		buffer += self._multisig_account_address_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._multisig_account_address.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'multisig_transaction_hash: {self._multisig_transaction_hash.__str__()}, '
		result += f'multisig_account_address: {self._multisig_account_address.__str__()}, '
		result += ')'
		return result


class SizePrefixedCosignature:
	TYPE_HINTS = {
		'cosignature': 'struct:Cosignature'
	}

	def __init__(self):
		self._cosignature = Cosignature()

	@property
	def cosignature(self) -> Cosignature:
		return self._cosignature

	@cosignature.setter
	def cosignature(self, value: Cosignature):
		self._cosignature = value

	@property
	def size(self) -> int:
		size = 0
		size += 4
		size += self.cosignature.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> SizePrefixedCosignature:
		buffer = memoryview(payload)
		cosignature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		cosignature = Cosignature.deserialize(buffer[:cosignature_size])
		buffer = buffer[cosignature.size:]

		instance = SizePrefixedCosignature()
		instance._cosignature = cosignature
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.cosignature.size.to_bytes(4, byteorder='little', signed=False)  # cosignature_size
		buffer += self._cosignature.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'cosignature: {self._cosignature.__str__()}, '
		result += ')'
		return result


class MultisigTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.MULTISIG_TRANSACTION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'inner_transaction': 'struct:NonVerifiableTransaction',
		'cosignatures': 'array[SizePrefixedCosignature]'
	}

	def __init__(self):
		self._type_ = MultisigTransaction.TRANSACTION_TYPE
		self._version = MultisigTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._inner_transaction = NonVerifiableTransaction()
		self._cosignatures = []
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def inner_transaction(self) -> NonVerifiableTransaction:
		return self._inner_transaction

	@property
	def cosignatures(self) -> List[SizePrefixedCosignature]:
		return self._cosignatures

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@inner_transaction.setter
	def inner_transaction(self, value: NonVerifiableTransaction):
		self._inner_transaction = value

	@cosignatures.setter
	def cosignatures(self, value: List[SizePrefixedCosignature]):
		self._cosignatures = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.inner_transaction.size
		size += 4
		size += ArrayHelpers.size(self.cosignatures)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> MultisigTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		inner_transaction_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		# marking sizeof field
		inner_transaction = NonVerifiableTransactionFactory.deserialize(buffer[:inner_transaction_size])
		buffer = buffer[inner_transaction.size:]
		cosignatures_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		cosignatures = ArrayHelpers.read_array_count(buffer, SizePrefixedCosignature, cosignatures_count)
		buffer = buffer[ArrayHelpers.size(cosignatures):]

		instance = MultisigTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._inner_transaction = inner_transaction
		instance._cosignatures = cosignatures
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self.inner_transaction.size.to_bytes(4, byteorder='little', signed=False)  # inner_transaction_size
		buffer += self._inner_transaction.serialize()
		buffer += len(self._cosignatures).to_bytes(4, byteorder='little', signed=False)  # cosignatures_count
		buffer += ArrayHelpers.write_array(self._cosignatures)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'inner_transaction: {self._inner_transaction.__str__()}, '
		result += f'cosignatures: {list(map(str, self._cosignatures))}, '
		result += ')'
		return result


class NamespaceRegistrationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NAMESPACE_REGISTRATION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'rental_fee_sink': 'pod:Address',
		'rental_fee': 'pod:Amount',
		'name': 'bytes_array',
		'parent_name': 'bytes_array'
	}

	def __init__(self):
		self._type_ = NamespaceRegistrationTransaction.TRANSACTION_TYPE
		self._version = NamespaceRegistrationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._rental_fee_sink = Address()
		self._rental_fee = Amount()
		self._name = bytes()
		self._parent_name = bytes()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._rental_fee_sink_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def rental_fee_sink(self) -> Address:
		return self._rental_fee_sink

	@property
	def rental_fee(self) -> Amount:
		return self._rental_fee

	@property
	def name(self) -> bytes:
		return self._name

	@property
	def parent_name(self) -> bytes:
		return self._parent_name

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@rental_fee_sink.setter
	def rental_fee_sink(self, value: Address):
		self._rental_fee_sink = value

	@rental_fee.setter
	def rental_fee(self, value: Amount):
		self._rental_fee = value

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@parent_name.setter
	def parent_name(self, value: bytes):
		self._parent_name = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.rental_fee_sink.size
		size += self.rental_fee.size
		size += 4
		size += len(self._name)
		size += 4
		if self.parent_name:
			size += len(self._parent_name)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NamespaceRegistrationTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		rental_fee_sink_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert rental_fee_sink_size == 40, f'Invalid value of reserved field ({rental_fee_sink_size})'
		rental_fee_sink = Address.deserialize(buffer)
		buffer = buffer[rental_fee_sink.size:]
		rental_fee = Amount.deserialize(buffer)
		buffer = buffer[rental_fee.size:]
		name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]
		parent_name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		parent_name = None
		if 4294967295 != parent_name_size:
			parent_name = ArrayHelpers.get_bytes(buffer, parent_name_size)
			buffer = buffer[parent_name_size:]

		instance = NamespaceRegistrationTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._rental_fee_sink = rental_fee_sink
		instance._rental_fee = rental_fee
		instance._name = name
		instance._parent_name = parent_name
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._rental_fee_sink_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._rental_fee_sink.serialize()
		buffer += self._rental_fee.serialize()
		buffer += len(self._name).to_bytes(4, byteorder='little', signed=False)  # name_size
		buffer += self._name
		buffer += (len(self._parent_name) if self._parent_name is not None else 4294967295).to_bytes(4, byteorder='little', signed=False)  # parent_name_size
		if self.parent_name:
			buffer += self._parent_name
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'rental_fee_sink: {self._rental_fee_sink.__str__()}, '
		result += f'rental_fee: {self._rental_fee.__str__()}, '
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		if self.parent_name:
			result += f'parent_name: {hexlify(self._parent_name).decode("utf8")}, '
		result += ')'
		return result


class NonVerifiableNamespaceRegistrationTransaction:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.NAMESPACE_REGISTRATION
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'rental_fee_sink': 'pod:Address',
		'rental_fee': 'pod:Amount',
		'name': 'bytes_array',
		'parent_name': 'bytes_array'
	}

	def __init__(self):
		self._type_ = NonVerifiableNamespaceRegistrationTransaction.TRANSACTION_TYPE
		self._version = NonVerifiableNamespaceRegistrationTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._rental_fee_sink = Address()
		self._rental_fee = Amount()
		self._name = bytes()
		self._parent_name = bytes()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._rental_fee_sink_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def rental_fee_sink(self) -> Address:
		return self._rental_fee_sink

	@property
	def rental_fee(self) -> Amount:
		return self._rental_fee

	@property
	def name(self) -> bytes:
		return self._name

	@property
	def parent_name(self) -> bytes:
		return self._parent_name

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@rental_fee_sink.setter
	def rental_fee_sink(self, value: Address):
		self._rental_fee_sink = value

	@rental_fee.setter
	def rental_fee(self, value: Amount):
		self._rental_fee = value

	@name.setter
	def name(self, value: bytes):
		self._name = value

	@parent_name.setter
	def parent_name(self, value: bytes):
		self._parent_name = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.rental_fee_sink.size
		size += self.rental_fee.size
		size += 4
		size += len(self._name)
		size += 4
		if self.parent_name:
			size += len(self._parent_name)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableNamespaceRegistrationTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		rental_fee_sink_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert rental_fee_sink_size == 40, f'Invalid value of reserved field ({rental_fee_sink_size})'
		rental_fee_sink = Address.deserialize(buffer)
		buffer = buffer[rental_fee_sink.size:]
		rental_fee = Amount.deserialize(buffer)
		buffer = buffer[rental_fee.size:]
		name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		name = ArrayHelpers.get_bytes(buffer, name_size)
		buffer = buffer[name_size:]
		parent_name_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		parent_name = None
		if 4294967295 != parent_name_size:
			parent_name = ArrayHelpers.get_bytes(buffer, parent_name_size)
			buffer = buffer[parent_name_size:]

		instance = NonVerifiableNamespaceRegistrationTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._rental_fee_sink = rental_fee_sink
		instance._rental_fee = rental_fee
		instance._name = name
		instance._parent_name = parent_name
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._rental_fee_sink_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._rental_fee_sink.serialize()
		buffer += self._rental_fee.serialize()
		buffer += len(self._name).to_bytes(4, byteorder='little', signed=False)  # name_size
		buffer += self._name
		buffer += (len(self._parent_name) if self._parent_name is not None else 4294967295).to_bytes(4, byteorder='little', signed=False)  # parent_name_size
		if self.parent_name:
			buffer += self._parent_name
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'rental_fee_sink: {self._rental_fee_sink.__str__()}, '
		result += f'rental_fee: {self._rental_fee.__str__()}, '
		result += f'name: {hexlify(self._name).decode("utf8")}, '
		if self.parent_name:
			result += f'parent_name: {hexlify(self._parent_name).decode("utf8")}, '
		result += ')'
		return result


class MessageType(Enum):
	PLAIN = 1
	ENCRYPTED = 2

	@property
	def size(self) -> int:
		return 4

	@classmethod
	def deserialize(cls, payload: ByteString) -> MessageType:
		buffer = memoryview(payload)
		return MessageType(int.from_bytes(buffer[:4], byteorder='little', signed=False))

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self.value.to_bytes(4, byteorder='little', signed=False)
		return buffer


class Message:
	TYPE_HINTS = {
		'message_type': 'enum:MessageType',
		'message': 'bytes_array'
	}

	def __init__(self):
		self._message_type = MessageType.PLAIN
		self._message = bytes()

	@property
	def message_type(self) -> MessageType:
		return self._message_type

	@property
	def message(self) -> bytes:
		return self._message

	@message_type.setter
	def message_type(self, value: MessageType):
		self._message_type = value

	@message.setter
	def message(self, value: bytes):
		self._message = value

	@property
	def size(self) -> int:
		size = 0
		size += self.message_type.size
		size += 4
		size += len(self._message)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> Message:
		buffer = memoryview(payload)
		message_type = MessageType.deserialize(buffer)
		buffer = buffer[message_type.size:]
		message_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		message = ArrayHelpers.get_bytes(buffer, message_size)
		buffer = buffer[message_size:]

		instance = Message()
		instance._message_type = message_type
		instance._message = message
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._message_type.serialize()
		buffer += len(self._message).to_bytes(4, byteorder='little', signed=False)  # message_size
		buffer += self._message
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'message_type: {self._message_type.__str__()}, '
		result += f'message: {hexlify(self._message).decode("utf8")}, '
		result += ')'
		return result


class TransferTransactionV1:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.TRANSFER
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:Address',
		'amount': 'pod:Amount',
		'message': 'struct:Message'
	}

	def __init__(self):
		self._type_ = TransferTransactionV1.TRANSACTION_TYPE
		self._version = TransferTransactionV1.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = Address()
		self._amount = Amount()
		self._message_envelope_size = 0
		self._message = Message()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._recipient_address_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> Address:
		return self._recipient_address

	@property
	def amount(self) -> Amount:
		return self._amount

	@property
	def message_envelope_size(self) -> int:
		return self._message_envelope_size

	@property
	def message(self) -> Message:
		return self._message

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: Address):
		self._recipient_address = value

	@amount.setter
	def amount(self, value: Amount):
		self._amount = value

	@message_envelope_size.setter
	def message_envelope_size(self, value: int):
		self._message_envelope_size = value

	@message.setter
	def message(self, value: Message):
		self._message = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.recipient_address.size
		size += self.amount.size
		size += 4
		if 0 != self.message_envelope_size:
			size += self.message.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> TransferTransactionV1:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert recipient_address_size == 40, f'Invalid value of reserved field ({recipient_address_size})'
		recipient_address = Address.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]
		message_envelope_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		message = None
		if 0 != message_envelope_size:
			message = Message.deserialize(buffer)
			buffer = buffer[message.size:]

		instance = TransferTransactionV1()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._amount = amount
		instance._message_envelope_size = message_envelope_size
		instance._message = message
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._recipient_address.serialize()
		buffer += self._amount.serialize()
		buffer += self._message_envelope_size.to_bytes(4, byteorder='little', signed=False)
		if 0 != self.message_envelope_size:
			buffer += self._message.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += f'message_envelope_size: 0x{self._message_envelope_size:X}, '
		if 0 != self.message_envelope_size:
			result += f'message: {self._message.__str__()}, '
		result += ')'
		return result


class NonVerifiableTransferTransactionV1:
	TRANSACTION_VERSION: int = 1
	TRANSACTION_TYPE: TransactionType = TransactionType.TRANSFER
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:Address',
		'amount': 'pod:Amount',
		'message': 'struct:Message'
	}

	def __init__(self):
		self._type_ = NonVerifiableTransferTransactionV1.TRANSACTION_TYPE
		self._version = NonVerifiableTransferTransactionV1.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = Address()
		self._amount = Amount()
		self._message_envelope_size = 0
		self._message = Message()
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._recipient_address_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> Address:
		return self._recipient_address

	@property
	def amount(self) -> Amount:
		return self._amount

	@property
	def message_envelope_size(self) -> int:
		return self._message_envelope_size

	@property
	def message(self) -> Message:
		return self._message

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: Address):
		self._recipient_address = value

	@amount.setter
	def amount(self, value: Amount):
		self._amount = value

	@message_envelope_size.setter
	def message_envelope_size(self, value: int):
		self._message_envelope_size = value

	@message.setter
	def message(self, value: Message):
		self._message = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.recipient_address.size
		size += self.amount.size
		size += 4
		if 0 != self.message_envelope_size:
			size += self.message.size
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableTransferTransactionV1:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert recipient_address_size == 40, f'Invalid value of reserved field ({recipient_address_size})'
		recipient_address = Address.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]
		message_envelope_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		message = None
		if 0 != message_envelope_size:
			message = Message.deserialize(buffer)
			buffer = buffer[message.size:]

		instance = NonVerifiableTransferTransactionV1()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._amount = amount
		instance._message_envelope_size = message_envelope_size
		instance._message = message
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._recipient_address.serialize()
		buffer += self._amount.serialize()
		buffer += self._message_envelope_size.to_bytes(4, byteorder='little', signed=False)
		if 0 != self.message_envelope_size:
			buffer += self._message.serialize()
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += f'message_envelope_size: 0x{self._message_envelope_size:X}, '
		if 0 != self.message_envelope_size:
			result += f'message: {self._message.__str__()}, '
		result += ')'
		return result


class TransferTransaction:
	TRANSACTION_VERSION: int = 2
	TRANSACTION_TYPE: TransactionType = TransactionType.TRANSFER
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'signature': 'pod:Signature',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:Address',
		'amount': 'pod:Amount',
		'message': 'struct:Message',
		'mosaics': 'array[SizePrefixedMosaic]'
	}

	def __init__(self):
		self._type_ = TransferTransaction.TRANSACTION_TYPE
		self._version = TransferTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._signature = Signature()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = Address()
		self._amount = Amount()
		self._message_envelope_size = 0
		self._message = Message()
		self._mosaics = []
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._signature_size = 64  # reserved field
		self._recipient_address_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def signature(self) -> Signature:
		return self._signature

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> Address:
		return self._recipient_address

	@property
	def amount(self) -> Amount:
		return self._amount

	@property
	def message_envelope_size(self) -> int:
		return self._message_envelope_size

	@property
	def message(self) -> Message:
		return self._message

	@property
	def mosaics(self) -> List[SizePrefixedMosaic]:
		return self._mosaics

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@signature.setter
	def signature(self, value: Signature):
		self._signature = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: Address):
		self._recipient_address = value

	@amount.setter
	def amount(self, value: Amount):
		self._amount = value

	@message_envelope_size.setter
	def message_envelope_size(self, value: int):
		self._message_envelope_size = value

	@message.setter
	def message(self, value: Message):
		self._message = value

	@mosaics.setter
	def mosaics(self, value: List[SizePrefixedMosaic]):
		self._mosaics = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += 4
		size += self.signature.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.recipient_address.size
		size += self.amount.size
		size += 4
		if 0 != self.message_envelope_size:
			size += self.message.size
		size += 4
		size += ArrayHelpers.size(self.mosaics)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> TransferTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		signature_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signature_size == 64, f'Invalid value of reserved field ({signature_size})'
		signature = Signature.deserialize(buffer)
		buffer = buffer[signature.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert recipient_address_size == 40, f'Invalid value of reserved field ({recipient_address_size})'
		recipient_address = Address.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]
		message_envelope_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		message = None
		if 0 != message_envelope_size:
			message = Message.deserialize(buffer)
			buffer = buffer[message.size:]
		mosaics_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		mosaics = ArrayHelpers.read_array_count(buffer, SizePrefixedMosaic, mosaics_count)
		buffer = buffer[ArrayHelpers.size(mosaics):]

		instance = TransferTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._signature = signature
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._amount = amount
		instance._message_envelope_size = message_envelope_size
		instance._message = message
		instance._mosaics = mosaics
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._signature_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signature.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._recipient_address.serialize()
		buffer += self._amount.serialize()
		buffer += self._message_envelope_size.to_bytes(4, byteorder='little', signed=False)
		if 0 != self.message_envelope_size:
			buffer += self._message.serialize()
		buffer += len(self._mosaics).to_bytes(4, byteorder='little', signed=False)  # mosaics_count
		buffer += ArrayHelpers.write_array(self._mosaics)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'signature: {self._signature.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += f'message_envelope_size: 0x{self._message_envelope_size:X}, '
		if 0 != self.message_envelope_size:
			result += f'message: {self._message.__str__()}, '
		result += f'mosaics: {list(map(str, self._mosaics))}, '
		result += ')'
		return result


class NonVerifiableTransferTransaction:
	TRANSACTION_VERSION: int = 2
	TRANSACTION_TYPE: TransactionType = TransactionType.TRANSFER
	TYPE_HINTS = {
		'type_': 'enum:TransactionType',
		'network': 'enum:NetworkType',
		'timestamp': 'pod:Timestamp',
		'signer_public_key': 'pod:PublicKey',
		'fee': 'pod:Amount',
		'deadline': 'pod:Timestamp',
		'recipient_address': 'pod:Address',
		'amount': 'pod:Amount',
		'message': 'struct:Message',
		'mosaics': 'array[SizePrefixedMosaic]'
	}

	def __init__(self):
		self._type_ = NonVerifiableTransferTransaction.TRANSACTION_TYPE
		self._version = NonVerifiableTransferTransaction.TRANSACTION_VERSION
		self._network = NetworkType.MAINNET
		self._timestamp = Timestamp()
		self._signer_public_key = PublicKey()
		self._fee = Amount()
		self._deadline = Timestamp()
		self._recipient_address = Address()
		self._amount = Amount()
		self._message_envelope_size = 0
		self._message = Message()
		self._mosaics = []
		self._entity_body_reserved_1 = 0  # reserved field
		self._signer_public_key_size = 32  # reserved field
		self._recipient_address_size = 40  # reserved field

	@property
	def type_(self) -> TransactionType:
		return self._type_

	@property
	def version(self) -> int:
		return self._version

	@property
	def network(self) -> NetworkType:
		return self._network

	@property
	def timestamp(self) -> Timestamp:
		return self._timestamp

	@property
	def signer_public_key(self) -> PublicKey:
		return self._signer_public_key

	@property
	def fee(self) -> Amount:
		return self._fee

	@property
	def deadline(self) -> Timestamp:
		return self._deadline

	@property
	def recipient_address(self) -> Address:
		return self._recipient_address

	@property
	def amount(self) -> Amount:
		return self._amount

	@property
	def message_envelope_size(self) -> int:
		return self._message_envelope_size

	@property
	def message(self) -> Message:
		return self._message

	@property
	def mosaics(self) -> List[SizePrefixedMosaic]:
		return self._mosaics

	@type_.setter
	def type_(self, value: TransactionType):
		self._type_ = value

	@version.setter
	def version(self, value: int):
		self._version = value

	@network.setter
	def network(self, value: NetworkType):
		self._network = value

	@timestamp.setter
	def timestamp(self, value: Timestamp):
		self._timestamp = value

	@signer_public_key.setter
	def signer_public_key(self, value: PublicKey):
		self._signer_public_key = value

	@fee.setter
	def fee(self, value: Amount):
		self._fee = value

	@deadline.setter
	def deadline(self, value: Timestamp):
		self._deadline = value

	@recipient_address.setter
	def recipient_address(self, value: Address):
		self._recipient_address = value

	@amount.setter
	def amount(self, value: Amount):
		self._amount = value

	@message_envelope_size.setter
	def message_envelope_size(self, value: int):
		self._message_envelope_size = value

	@message.setter
	def message(self, value: Message):
		self._message = value

	@mosaics.setter
	def mosaics(self, value: List[SizePrefixedMosaic]):
		self._mosaics = value

	@property
	def size(self) -> int:
		size = 0
		size += self.type_.size
		size += 1
		size += 2
		size += self.network.size
		size += self.timestamp.size
		size += 4
		size += self.signer_public_key.size
		size += self.fee.size
		size += self.deadline.size
		size += 4
		size += self.recipient_address.size
		size += self.amount.size
		size += 4
		if 0 != self.message_envelope_size:
			size += self.message.size
		size += 4
		size += ArrayHelpers.size(self.mosaics)
		return size

	@classmethod
	def deserialize(cls, payload: ByteString) -> NonVerifiableTransferTransaction:
		buffer = memoryview(payload)
		type_ = TransactionType.deserialize(buffer)
		buffer = buffer[type_.size:]
		version = int.from_bytes(buffer[:1], byteorder='little', signed=False)
		buffer = buffer[1:]
		entity_body_reserved_1 = int.from_bytes(buffer[:2], byteorder='little', signed=False)
		buffer = buffer[2:]
		assert entity_body_reserved_1 == 0, f'Invalid value of reserved field ({entity_body_reserved_1})'
		network = NetworkType.deserialize(buffer)
		buffer = buffer[network.size:]
		timestamp = Timestamp.deserialize(buffer)
		buffer = buffer[timestamp.size:]
		signer_public_key_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert signer_public_key_size == 32, f'Invalid value of reserved field ({signer_public_key_size})'
		signer_public_key = PublicKey.deserialize(buffer)
		buffer = buffer[signer_public_key.size:]
		fee = Amount.deserialize(buffer)
		buffer = buffer[fee.size:]
		deadline = Timestamp.deserialize(buffer)
		buffer = buffer[deadline.size:]
		recipient_address_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		assert recipient_address_size == 40, f'Invalid value of reserved field ({recipient_address_size})'
		recipient_address = Address.deserialize(buffer)
		buffer = buffer[recipient_address.size:]
		amount = Amount.deserialize(buffer)
		buffer = buffer[amount.size:]
		message_envelope_size = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		message = None
		if 0 != message_envelope_size:
			message = Message.deserialize(buffer)
			buffer = buffer[message.size:]
		mosaics_count = int.from_bytes(buffer[:4], byteorder='little', signed=False)
		buffer = buffer[4:]
		mosaics = ArrayHelpers.read_array_count(buffer, SizePrefixedMosaic, mosaics_count)
		buffer = buffer[ArrayHelpers.size(mosaics):]

		instance = NonVerifiableTransferTransaction()
		instance._type_ = type_
		instance._version = version
		instance._network = network
		instance._timestamp = timestamp
		instance._signer_public_key = signer_public_key
		instance._fee = fee
		instance._deadline = deadline
		instance._recipient_address = recipient_address
		instance._amount = amount
		instance._message_envelope_size = message_envelope_size
		instance._message = message
		instance._mosaics = mosaics
		return instance

	def serialize(self) -> bytes:
		buffer = bytes()
		buffer += self._type_.serialize()
		buffer += self._version.to_bytes(1, byteorder='little', signed=False)
		buffer += self._entity_body_reserved_1.to_bytes(2, byteorder='little', signed=False)
		buffer += self._network.serialize()
		buffer += self._timestamp.serialize()
		buffer += self._signer_public_key_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._signer_public_key.serialize()
		buffer += self._fee.serialize()
		buffer += self._deadline.serialize()
		buffer += self._recipient_address_size.to_bytes(4, byteorder='little', signed=False)
		buffer += self._recipient_address.serialize()
		buffer += self._amount.serialize()
		buffer += self._message_envelope_size.to_bytes(4, byteorder='little', signed=False)
		if 0 != self.message_envelope_size:
			buffer += self._message.serialize()
		buffer += len(self._mosaics).to_bytes(4, byteorder='little', signed=False)  # mosaics_count
		buffer += ArrayHelpers.write_array(self._mosaics)
		return buffer

	def __str__(self) -> str:
		result = '('
		result += f'type_: {self._type_.__str__()}, '
		result += f'version: 0x{self._version:X}, '
		result += f'network: {self._network.__str__()}, '
		result += f'timestamp: {self._timestamp.__str__()}, '
		result += f'signer_public_key: {self._signer_public_key.__str__()}, '
		result += f'fee: {self._fee.__str__()}, '
		result += f'deadline: {self._deadline.__str__()}, '
		result += f'recipient_address: {self._recipient_address.__str__()}, '
		result += f'amount: {self._amount.__str__()}, '
		result += f'message_envelope_size: 0x{self._message_envelope_size:X}, '
		if 0 != self.message_envelope_size:
			result += f'message: {self._message.__str__()}, '
		result += f'mosaics: {list(map(str, self._mosaics))}, '
		result += ')'
		return result


class TransactionFactory:
	@classmethod
	def deserialize(cls, payload: bytes) -> Transaction:
		buffer = bytes(payload)
		parent = Transaction.deserialize(buffer)
		mapping = {
			(AccountKeyLinkTransaction.TRANSACTION_TYPE, AccountKeyLinkTransaction.TRANSACTION_VERSION): AccountKeyLinkTransaction,
			(MosaicDefinitionTransaction.TRANSACTION_TYPE, MosaicDefinitionTransaction.TRANSACTION_VERSION): MosaicDefinitionTransaction,
			(MosaicSupplyChangeTransaction.TRANSACTION_TYPE, MosaicSupplyChangeTransaction.TRANSACTION_VERSION): MosaicSupplyChangeTransaction,
			(MultisigAccountModificationTransactionV1.TRANSACTION_TYPE, MultisigAccountModificationTransactionV1.TRANSACTION_VERSION): MultisigAccountModificationTransactionV1,
			(MultisigAccountModificationTransaction.TRANSACTION_TYPE, MultisigAccountModificationTransaction.TRANSACTION_VERSION): MultisigAccountModificationTransaction,
			(Cosignature.TRANSACTION_TYPE, Cosignature.TRANSACTION_VERSION): Cosignature,
			(MultisigTransaction.TRANSACTION_TYPE, MultisigTransaction.TRANSACTION_VERSION): MultisigTransaction,
			(NamespaceRegistrationTransaction.TRANSACTION_TYPE, NamespaceRegistrationTransaction.TRANSACTION_VERSION): NamespaceRegistrationTransaction,
			(TransferTransactionV1.TRANSACTION_TYPE, TransferTransactionV1.TRANSACTION_VERSION): TransferTransactionV1,
			(TransferTransaction.TRANSACTION_TYPE, TransferTransaction.TRANSACTION_VERSION): TransferTransaction
		}
		discriminator = (parent.type_, parent.version)
		factory_class = mapping[discriminator]
		return factory_class.deserialize(buffer)

	@classmethod
	def create_by_name(cls, entity_name: str) -> Transaction:
		mapping = {
			'account_key_link_transaction': AccountKeyLinkTransaction,
			'mosaic_definition_transaction': MosaicDefinitionTransaction,
			'mosaic_supply_change_transaction': MosaicSupplyChangeTransaction,
			'multisig_account_modification_transaction_v1': MultisigAccountModificationTransactionV1,
			'multisig_account_modification_transaction': MultisigAccountModificationTransaction,
			'cosignature': Cosignature,
			'multisig_transaction': MultisigTransaction,
			'namespace_registration_transaction': NamespaceRegistrationTransaction,
			'transfer_transaction_v1': TransferTransactionV1,
			'transfer_transaction': TransferTransaction
		}

		if entity_name not in mapping:
			raise ValueError('unknown Transaction type')

		return mapping[entity_name]()


class NonVerifiableTransactionFactory:
	@classmethod
	def deserialize(cls, payload: bytes) -> NonVerifiableTransaction:
		buffer = bytes(payload)
		parent = NonVerifiableTransaction.deserialize(buffer)
		mapping = {
			(NonVerifiableAccountKeyLinkTransaction.TRANSACTION_TYPE, NonVerifiableAccountKeyLinkTransaction.TRANSACTION_VERSION): NonVerifiableAccountKeyLinkTransaction,
			(NonVerifiableMosaicDefinitionTransaction.TRANSACTION_TYPE, NonVerifiableMosaicDefinitionTransaction.TRANSACTION_VERSION): NonVerifiableMosaicDefinitionTransaction,
			(NonVerifiableMosaicSupplyChangeTransaction.TRANSACTION_TYPE, NonVerifiableMosaicSupplyChangeTransaction.TRANSACTION_VERSION): NonVerifiableMosaicSupplyChangeTransaction,
			(NonVerifiableMultisigAccountModificationTransactionV1.TRANSACTION_TYPE, NonVerifiableMultisigAccountModificationTransactionV1.TRANSACTION_VERSION): NonVerifiableMultisigAccountModificationTransactionV1,
			(NonVerifiableMultisigAccountModificationTransaction.TRANSACTION_TYPE, NonVerifiableMultisigAccountModificationTransaction.TRANSACTION_VERSION): NonVerifiableMultisigAccountModificationTransaction,
			(NonVerifiableNamespaceRegistrationTransaction.TRANSACTION_TYPE, NonVerifiableNamespaceRegistrationTransaction.TRANSACTION_VERSION): NonVerifiableNamespaceRegistrationTransaction,
			(NonVerifiableTransferTransactionV1.TRANSACTION_TYPE, NonVerifiableTransferTransactionV1.TRANSACTION_VERSION): NonVerifiableTransferTransactionV1,
			(NonVerifiableTransferTransaction.TRANSACTION_TYPE, NonVerifiableTransferTransaction.TRANSACTION_VERSION): NonVerifiableTransferTransaction
		}
		discriminator = (parent.type_, parent.version)
		factory_class = mapping[discriminator]
		return factory_class.deserialize(buffer)

	@classmethod
	def create_by_name(cls, entity_name: str) -> NonVerifiableTransaction:
		mapping = {
			'non_verifiable_account_key_link_transaction': NonVerifiableAccountKeyLinkTransaction,
			'non_verifiable_mosaic_definition_transaction': NonVerifiableMosaicDefinitionTransaction,
			'non_verifiable_mosaic_supply_change_transaction': NonVerifiableMosaicSupplyChangeTransaction,
			'non_verifiable_multisig_account_modification_transaction_v1': NonVerifiableMultisigAccountModificationTransactionV1,
			'non_verifiable_multisig_account_modification_transaction': NonVerifiableMultisigAccountModificationTransaction,
			'non_verifiable_namespace_registration_transaction': NonVerifiableNamespaceRegistrationTransaction,
			'non_verifiable_transfer_transaction_v1': NonVerifiableTransferTransactionV1,
			'non_verifiable_transfer_transaction': NonVerifiableTransferTransaction
		}

		if entity_name not in mapping:
			raise ValueError('unknown NonVerifiableTransaction type')

		return mapping[entity_name]()
