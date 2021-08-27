import "entity.cats"
import "transaction_type.cats"

# binary layout for a transaction
struct Transaction
	inline SizePrefixedEntity
	inline VerifiableEntity
	inline EntityBody

	# transaction type
	type = TransactionType

	# transaction fee
	fee = Amount

	# transaction deadline
	deadline = Timestamp

# binary layout for an embedded transaction header
struct EmbeddedTransactionHeader
	inline SizePrefixedEntity

	# reserved padding to align end of EmbeddedTransactionHeader on 8-byte boundary
	embeddedTransactionHeader_Reserved1 = uint32

# binary layout for an embedded transaction
struct EmbeddedTransaction
	inline EmbeddedTransactionHeader
	inline EntityBody

	# transaction type
	type = TransactionType
