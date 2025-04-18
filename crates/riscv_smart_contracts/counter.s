# SPDX-License-Identifier: MIT
# Simple Counter Contract in RISC-V RV32IM Assembly

# --- Ecall Definitions (Using the same as provided) ---
.equ ECALL_KECCAK256, 0x20    # [offset, size] -> hash
.equ ECALL_ADDRESS, 0x30      # Address of the current executing contract |-> address
.equ ECALL_BALANCE, 0x31      # Native balance of the current caller [address] -> balance
.equ ECALL_ORIGIN, 0x32       # Address of the transaction origin |-> address
.equ ECALL_CALLER, 0x33       # Address of the current calling address |-> address
.equ ECALL_CALLVALUE, 0x34    # Deposit value for this Tx |-> value
.equ ECALL_CALLDATALOAD, 0x35 # Load a Word(256bits) from the calldata [i] -> data[i]
.equ ECALL_CALLDATASIZE, 0x36 # Returns the size of the calldata |-> usize
.equ ECALL_CALLDATACOPY, 0x37 # Copy calldata from input to memory [destOffset, offset, size]
.equ ECALL_CODESIZE, 0x38     # Returns the size of the code |-> usize
.equ ECALL_CODECOPY, 0x39     # Copy code from input to memory [destOffset, offset, size]
.equ ECALL_GASPRICE, 0x3A     # Gas price now
.equ ECALL_EXTCODESIZE, 0x3B  # Get the size of an External account's code [address] -> usize
.equ ECALL_EXTCODECOPY, 0x3C  # Get the code of an External account [address, destOffset, offset, size]
.equ ECALL_RETURNDATASIZE, 0x3D # Get size of output data from the previous call
.equ ECALL_RETURNDATACOPY, 0x3E # Copy output data from the previous call [destOffset, offset, size]
.equ ECALL_EXTCODEHASH, 0x3F  # Get hash of an account's code [address] -> hash
.equ ECALL_BLOCKHASH, 0x40    # Get hash of recent block [blockNumber] -> hash
.equ ECALL_COINBASE, 0x41     # Get the block's beneficiary address |-> address
.equ ECALL_TIMESTAMP, 0x42    # Get the block's timestamp |-> timestamp
.equ ECALL_NUMBER, 0x43       # Get the block's number |-> blockNumber
.equ ECALL_PREVRANDAO, 0x44   # Get the block's difficulty |-> difficulty
.equ ECALL_GASLIMIT, 0x45     # Get the block's gas limit |-> gasLimit
.equ ECALL_CHAINID, 0x46      # Get the chain ID |-> chainId
.equ ECALL_SELFBALANCE, 0x47  # Get balance of currently executing account |-> balance
.equ ECALL_BASEFEE, 0x48      # Get the base fee |-> baseFee
.equ ECALL_BLOBHASH, 0x49     # Get versioned hashes [index] -> blobVersionedHashesAtIndex
.equ ECALL_BLOBBASEFEE, 0x4A  # Returns the blob base-fee of the current block |-> blobBaseFee
.equ ECALL_GAS, 0x5A          # Amount of available gas
.equ ECALL_LOG0, 0xA0         # Append log record with no topics [offset, size]
.equ ECALL_LOG1, 0xA1         # Append log record with one topic [offset, size, topic]
.equ ECALL_LOG2, 0xA2         # Append log record with two topics [offset, size, topic1, topic2]
.equ ECALL_LOG3, 0xA3         # Append log record with three topics [offset, size, topic1, topic2, topic3]
.equ ECALL_LOG4, 0xA4         # Append log record with four topics [offset, size, topic1-4]
.equ ECALL_CREATE, 0xF0       # Create a new account with code [value, offset, size] -> address
.equ ECALL_CALL, 0xF1         # Call into an account [gas, address, value, argsOffset, argsSize, retOffset, retSize]
.equ ECALL_CALLCODE, 0xF2     # Call with alternative code [gas, address, value, argsOffset, argsSize, retOffset, retSize]
.equ ECALL_RETURN, 0xF3       # Halt execution returning output data [offset, size]
.equ ECALL_DELEGATECALL, 0xF4 # Call with alternative code, persisting sender and value [gas, address, argsOffset, argsSize, retOffset, retSize]
.equ ECALL_CREATE2, 0xF5      # Create account with predictable address [value, offset, size, salt] -> address
.equ ECALL_STATICCALL, 0xFA   # Static call into an account [gas, address, argsOffset, argsSize, retOffset, retSize]
.equ ECALL_REVERT, 0xFD       # Halt execution reverting state changes [offset, size]
.equ ECALL_SLOAD, 0x54        # Loads a word (32-bytes) from storage
.equ ECALL_SSTORE, 0x55       # Stores a word (32-bytes) to storage


# --- Storage Slot Definitions ---
.equ SLOT_COUNTER_1, 0
.equ SLOT_COUNTER_2, 0
.equ SLOT_COUNTER_3, 0
.equ SLOT_COUNTER_4, 0
.equ SLOT_COUNTER_5, 0
.equ SLOT_COUNTER_6, 0
.equ SLOT_COUNTER_7, 0
.equ SLOT_COUNTER_8, 0


.text           # Entry point for deployment (initcode)

# ==============================================================
# INITCODE SECTION
# Runs only once during deployment.
# Sets up initial state and returns the runtime code.
# ==============================================================
_start:
    # --- Initialize Counter Value to 0 ---    
    addi x1, zero, SLOT_COUNTER_1 
    addi x2, zero, SLOT_COUNTER_2 
    addi x3, zero, SLOT_COUNTER_3 
    addi x4, zero, SLOT_COUNTER_4 
    addi x5, zero, SLOT_COUNTER_5 
    addi x6, zero, SLOT_COUNTER_6 
    addi x7, zero, SLOT_COUNTER_7 
    addi x8, zero, SLOT_COUNTER_8 
    
    addi x9, zero, 0 
    addi x10, zero, 0 
    addi x11, zero, 0 
    addi x12, zero, 0 
    addi x13, zero, 0 
    addi x14, zero, 0 
    addi x15, zero, 0 
    addi x16, zero, 1 
    
    # Store initial counter value to storage
    addi x31, zero, ECALL_SSTORE
    ecall
    
    # --- Return Runtime Code ---
    # Calculate the size and offset of the runtime code section
    addi x5, zero, runtime_code_start # Get address of runtime code start
    addi x6, zero, runtime_code_end
    
    sub x3, x6, x5            # x3 = length of runtime code

    # Return the copied code
    add x1, zero, x5                # mem_offset = x5
    add x2, zero, x3                # length of runtime code
    addi x31, zero, ECALL_RETURN
    ecall

    # End of initcode. Should not be reached after ECALL_RETURN.


# ==============================================================
# RUNTIME CODE SECTION
# This code is stored on the blockchain after deployment.
# It handles subsequent calls to the contract.
# ==============================================================
runtime_code_start:
    # --- Runtime Entry Point & Function Dispatcher ---
    # Read function selector (first 4 bytes of calldata)
    addi x1, zero, 0                # offset = 0
    addi x31, zero, ECALL_CALLDATALOAD  # setup for the ECALL
    ecall                     # Returns first 32 bytes: Note in this case x2 would hold the first 4 bytes
    

    # Function dispatcher - compare with known selectors
    addi x3, zero, 0x00000037   # Selector for increment()      
    beq x2, x3, _increment

    addi x3, zero, 0x00000020   # Selector for getValue()      
    beq x2, x3, _getValue

    addi x3, zero, 0x00000055   # Selector for setValue(uint256)      
    beq x2, x3, _setValue

    # Fallback: If no function matches, revert
    jal x0, _revert_default

# --- Function Implementations ---

_increment:
    # Increment the counter value by 1
    # 1. Load current counter value
    # 2. Add 1
    # 3. Store new counter value
    # 4. Return success (true)
    
    # Load current counter value
    addi x1, zero, SLOT_COUNTER_1 
    addi x2, zero, SLOT_COUNTER_2 
    addi x3, zero, SLOT_COUNTER_3 
    addi x4, zero, SLOT_COUNTER_4 
    addi x5, zero, SLOT_COUNTER_5 
    addi x6, zero, SLOT_COUNTER_6 
    addi x7, zero, SLOT_COUNTER_7 
    addi x8, zero, SLOT_COUNTER_8 
    
    addi x31, zero, ECALL_SLOAD  # setup for the ECALL
    ecall                     # Returns counter value in [x9 - x16]

    
    # Increment counter (need to handle potential overflow from any of the limbs)
    addi x16, x16, 1            # counter_last_limb += 1, next is to check for overflow and propagate if need be
    beq x16, zero, _inc_overflow      # If counter_last_limb wrapped to 0, increment counter_last_limb - 1
    jal x0, _inc_store

_inc_overflow:
    addi x15, x15, 1            # counter_last_limb-1 += 1
    bne x15, zero, _inc_store      # If counter_last_limb-1 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    
    # Increment counter_last_limb - 2
    addi x14, x14, 1            # counter_last_limb-2 += 1
    bne x14, zero, _inc_store      # If counter_last_limb-2 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    
    # Increment counter_last_limb - 3
    addi x13, x13, 1            # counter_last_limb-3 += 1
    bne x13, zero, _inc_store      # If counter_last_limb-3 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    
    # Increment counter_last_limb - 4
    addi x12, x12, 1            # counter_last_limb-4 += 1
    bne x12, zero, _inc_store      # If counter_last_limb-4 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    
    # Increment counter_last_limb - 5
    addi x11, x11, 1            # counter_last_limb-5 += 1
    bne x11, zero, _inc_store      # If counter_last_limb-5 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    
    # Increment counter_last_limb - 6
    addi x10, x10, 1            # counter_last_limb-6 += 1
    bne x10, zero, _inc_store      # If counter_last_limb-6 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    
    # Increment counter_last_limb - 7
    addi x9, x9, 1            # counter_last_limb-7 += 1
    bne x9, zero, _inc_store      # If counter_last_limb-7 does not wrapped to 0, continue to store this to storage becuase there was no overflow
    

_inc_store:
    # Store updated counter value
    # At this point this new state of the counter is stored on register [x9, x16]
    addi x1, zero, SLOT_COUNTER_1   # Setting up counter slot
    addi x2, zero, SLOT_COUNTER_2   # Setting up counter slot
    addi x3, zero, SLOT_COUNTER_3   # Setting up counter slot
    addi x4, zero, SLOT_COUNTER_4   # Setting up counter slot
    addi x5, zero, SLOT_COUNTER_5   # Setting up counter slot
    addi x6, zero, SLOT_COUNTER_6   # Setting up counter slot
    addi x7, zero, SLOT_COUNTER_7   # Setting up counter slot
    addi x8, zero, SLOT_COUNTER_8   # Setting up counter slot
    
    # The value is already stored in registers [x9, x16]
    addi x31, zero, ECALL_SSTORE  # setup for the ECALL
    ecall
    
    # Return success (true = 1)
    jal x0, _return_true
    

_getValue:
    # Return the current counter value
    # 1. Load counter value
    # 2. Return it
    
    # Load counter value
    addi x1, zero, SLOT_COUNTER_1 
    addi x2, zero, SLOT_COUNTER_2 
    addi x3, zero, SLOT_COUNTER_3 
    addi x4, zero, SLOT_COUNTER_4 
    addi x5, zero, SLOT_COUNTER_5 
    addi x6, zero, SLOT_COUNTER_6 
    addi x7, zero, SLOT_COUNTER_7 
    addi x8, zero, SLOT_COUNTER_8
    
    addi x31, zero, ECALL_SLOAD
    ecall                           # Load counter value storing at [x9 - x16]
    
    # Writing this value to storage
    addi x2, x2, -8           # Allocate stack space for uint256
    sw x9, 0(x2)             # Store counter_limb_1 at sp
    sw x10, 4(x2)             # Store counter_limb_2 at sp+4
    sw x11, 8(x2)             # Store counter_limb_3 at sp+8
    sw x12, 12(x2)            # Store counter_limb_4 at sp+12
    sw x13, 16(x2)            # Store counter_limb_5 at sp+16
    sw x14, 20(x2)            # Store counter_limb_6 at sp+20
    sw x15, 24(x2)            # Store counter_limb_7 at sp+24
    sw x16, 28(x2)            # Store counter_limb_8 at sp+28
    
    
    # Return the counter value   
    add x1, zero, x2                # mem_offset = 0
    addi x2, zero, 8                # length = 8 bytes (uint256)
    addi x31, zero, ECALL_RETURN
    ecall
    
    # Clean up stack (should not be reached after ECALL_RETURN)
    addi x2, x2, 8

_setValue:
    # Set the counter to a specific value
    # Args: uint256 _value (offset 4)
    # 1. Load the new value from calldata
    # 2. Store it as the new counter value
    # 3. Return success (true)
    
    # Load new value from calldata   
    addi x1, zero, 4   # offset skipping function selector
    addi x31, zero, ECALL_CALLDATALOAD
    ecall # call data is now stored from [x2 - x9]
    
    # Store new counter value   
    
    add x10, zero, x3 
    add x11, zero, x4 
    add x12, zero, x5 
    add x13, zero, x6 
    add x14, zero, x7 
    add x15, zero, x8 
    add x16, zero, x9 
    add x9, zero, x2
    
    addi x1, zero, SLOT_COUNTER_1 
    addi x2, zero, SLOT_COUNTER_2 
    addi x3, zero, SLOT_COUNTER_3 
    addi x4, zero, SLOT_COUNTER_4 
    addi x5, zero, SLOT_COUNTER_5 
    addi x6, zero, SLOT_COUNTER_6 
    addi x7, zero, SLOT_COUNTER_7 
    addi x8, zero, SLOT_COUNTER_8 
    
    # Store initial counter value to storage
    addi x31, zero, ECALL_SSTORE
    ecall
    
    # Return success (true = 1)
    jal x0, _return_true

# --- Helper Routines ---

_return_true:
    # Prepare return value 'true' (uint256(1))
    addi x2, zero, -48           # Allocate stack space
    addi x31, zero, ECALL_SSTORE
    addi x3, zero, 0
    addi x4, zero, 1
    
    sw x3, 0(x2)             # Store counter_limb_1 at sp
    sw x3, 4(x2)             # Store counter_limb_2 at sp+4
    sw x3, 8(x2)             # Store counter_limb_3 at sp+8
    sw x3, 12(x2)            # Store counter_limb_4 at sp+12
    sw x3, 16(x2)            # Store counter_limb_5 at sp+16
    sw x3, 20(x2)            # Store counter_limb_6 at sp+20
    sw x3, 24(x2)            # Store counter_limb_7 at sp+24
    sw x4, 28(x2)            # Store counter_limb_8 at sp+28
    
    # Return true u256(1)  
    add x1, zero, x2                # mem_offset = 0
    addi x2, zero, 32                # length = 8 bytes (uint256)
    addi x31, zero, ECALL_RETURN
    ecall
    
    # Clean up stack (should not be reached after ECALL_RETURN)
    addi x2, x2, 8
    jal x0, _return_true

_revert_default:   
    addi x1, zero, 0 
    addi x2, zero, 0                
    addi x31, zero, ECALL_REVERT
    ecall

runtime_code_end:              # Mark the end of the runtime code section