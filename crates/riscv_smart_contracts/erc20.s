# This is still a work in progress (I have been very lazy :(
# SPDX-License-Identifier: MIT
# Basic ERC20 Token in RISC-V RV32IM Assembly (Illustrative)

# --- Ecall Definitions (Hypothetical) ---
.equ ECALL_SSTORE, 1          # Args: a0=slot_hi, a1=slot_lo, a2=val_hi, a3=val_lo
.equ ECALL_SLOAD, 2           # Args: a0=slot_hi, a1=slot_lo. Returns: a0=val_hi, a1=val_lo
.equ ECALL_CALLER, 3          # Returns: a0=addr_hi, a1=addr_lo
.equ ECALL_CALLDATASIZE, 4    # Returns: a0=size
.equ ECALL_CALLDATALOAD, 5    # Args: a0=offset. Returns: a0=word_hi, a1=word_lo (reads 32 bytes)
.equ ECALL_CODECOPY, 6        # Args: a0=mem_offset, a1=code_offset, a2=length
.equ ECALL_LOG3, 7            # Args: a0=mem_offset, a1=length, a2=topic1_hi, a3=topic1_lo, a4=topic2_hi, a5=topic2_lo, a6=topic3_hi, t0=topic3_lo
.equ ECALL_RETURN, 8          # Args: a0=mem_offset, a1=length
.equ ECALL_REVERT, 9          # Args: a0=mem_offset, a1=length
.equ ECALL_KECCAK256, 10      # Args: a0=mem_offset, a1=length. Returns: a0=hash_hi, a1=hash_lo (Simplified!)

# --- Storage Slot Definitions ---
.equ SLOT_NAME, 0
.equ SLOT_SYMBOL, 1
.equ SLOT_DECIMALS, 2
.equ SLOT_TOTALSUPPLY, 3
.equ SLOT_BALANCES_BASE, 4
.equ SLOT_ALLOWANCES_BASE, 5

# --- Constants ---
.equ ADDR_SIZE, 20           # Bytes in an address (Simplified - EVM uses 20)
.equ WORD_SIZE, 32           # Bytes in a storage word/calldata word
.equ ZERO_ADDR_HI, 0
.equ ZERO_ADDR_LO, 0

.section .text
.global _start             # Entry point for deployment (initcode)

# ==============================================================
# INITCODE SECTION
# Runs only once during deployment.
# Sets up initial state and returns the runtime code.
# ==============================================================
_start:
    # --- Load Constructor Arguments ---
    # Assume calldata layout: name_len, name_data, symbol_len, symbol_data, decimals, initialSupply_hi, initialSupply_lo
    # NOTE: This part is complex in reality (ABI decoding). Simplified here.
    # Let's assume args are pre-loaded into registers for simplicity or fetched via CALLDATALOAD
    # Example: Pretend args are in memory locations referenced by s0, s1, s2, s3
    # s0 = pointer to name string
    # s1 = pointer to symbol string
    # s2 = decimals (uint8)
    # s3 = initialSupply (uint256 - assume in s3/s4 pair for hi/lo)

    # In a real scenario, use ECALL_CALLDATALOAD repeatedly based on expected layout.
    # For this example, we'll skip complex ABI decoding and assume values are somehow available.
    # We'll store placeholders or simplified values.

    # --- Store Name & Symbol (Simplified: storing pointers/references) ---
    # This usually involves storing the actual string data elsewhere or using a different mechanism.
    li t0, SLOT_NAME          # Storage slot for name
    li t1, 0                  # Assuming name data reference is 0 (placeholder)
    li t2, 0
    mv a0, t1                 # slot_hi = 0
    mv a1, t0                 # slot_lo = SLOT_NAME
    mv a2, t2                 # val_hi = 0
    mv a3, t1                 # val_lo = 0 (placeholder reference)
    li a7, ECALL_SSTORE
    ecall

    li t0, SLOT_SYMBOL        # Storage slot for symbol
    li t1, 1                  # Assuming symbol data reference is 1 (placeholder)
    li t2, 0
    mv a0, t2                 # slot_hi = 0
    mv a1, t0                 # slot_lo = SLOT_SYMBOL
    mv a2, t2                 # val_hi = 0
    mv a3, t1                 # val_lo = 1 (placeholder reference)
    li a7, ECALL_SSTORE
    ecall

    # --- Store Decimals ---
    # Assume decimals value is in register s2 (e.g., 18)
    li t0, SLOT_DECIMALS
    li t1, 0                  # Assuming s2 holds decimals
    li t2, 0
    lw t1, 0(s2)              # Load decimals value from assumed memory location
    mv a0, t2                 # slot_hi = 0
    mv a1, t0                 # slot_lo = SLOT_DECIMALS
    mv a2, t2                 # val_hi = 0
    mv a3, t1                 # val_lo = decimals
    li a7, ECALL_SSTORE
    ecall

    # --- Store Initial Supply & Set Balance ---
    # Assume initialSupply in s3 (hi), s4 (lo)
    # Calculate totalSupply = initialSupply * (10**decimals) - SKIPPED FOR SIMPLICITY
    # We'll just store the provided initialSupply as totalSupply

    # Store totalSupply
    li t0, SLOT_TOTALSUPPLY
    li t1, 0
    lw t2, 0(s3)              # Load initialSupply_hi
    lw t3, 4(s3)              # Load initialSupply_lo
    mv a0, t1                 # slot_hi = 0
    mv a1, t0                 # slot_lo = SLOT_TOTALSUPPLY
    mv a2, t2                 # val_hi = initialSupply_hi
    mv a3, t3                 # val_lo = initialSupply_lo
    li a7, ECALL_SSTORE
    ecall

    # Get msg.sender (deployer)
    li a7, ECALL_CALLER
    ecall                     # Returns caller address in a0 (hi), a1 (lo)
    mv s5, a0                 # Store caller_hi
    mv s6, a1                 # Store caller_lo

    # Calculate balance slot for deployer: hash(SLOT_BALANCES_BASE, msg.sender)
    # Simplified Hash: Just use sender address parts + base slot (INSECURE!)
    li t0, SLOT_BALANCES_BASE
    add t1, s5, t0            # Simplified slot_hi
    add t2, s6, t0            # Simplified slot_lo

    # Store initial balance for deployer
    mv a0, t1                 # slot_hi
    mv a1, t2                 # slot_lo
    mv a2, t2                 # val_hi = initialSupply_hi (previously loaded)
    mv a3, t3                 # val_lo = initialSupply_lo (previously loaded)
    li a7, ECALL_SSTORE
    ecall

    # --- Emit Transfer Event (from address(0) to deployer) ---
    # Topic 1: Event Signature Hash (e.g., keccak256("Transfer(address,address,uint256)")) - Precomputed
    # Topic 2: from (address(0))
    # Topic 3: to (msg.sender)
    # Data: value (totalSupply)

    # Prepare data buffer for log (value) - Simplified: Assume stack space
    addi sp, sp, -64          # Allocate space for topics and data
    sw t2, 0(sp)              # Store value_hi (totalSupply_hi)
    sw t3, 4(sp)              # Store value_lo (totalSupply_lo)

    # Arguments for ECALL_LOG3
    mv a0, sp                 # mem_offset (points to value data)
    li a1, 8                  # length (for uint256 value)
    # Topic 1 (Event Hash - use placeholder 0x123...)
    li a2, 0
    li a3, 0x12345678
    # Topic 2 (from = address(0))
    li a4, ZERO_ADDR_HI
    li a5, ZERO_ADDR_LO
    # Topic 3 (to = msg.sender)
    mv a6, s5                 # caller_hi
    mv t0, s6                 # caller_lo (ecall uses t0 for 7th arg)

    li a7, ECALL_LOG3
    ecall

    addi sp, sp, 64           # Deallocate stack space

    # --- Return Runtime Code ---
    # Calculate the size and offset of the runtime code section.
    # runtime_code_start and runtime_code_end are labels defined below.
    la t0, runtime_code_start # Get address of runtime code start
    la t1, runtime_code_end   # Get address of runtime code end
    sub t2, t1, t0            # t2 = length of runtime code

    # We need to copy the runtime code from our *code space* to *memory*
    # Assume memory starting at address 0 is usable for return data.
    li t3, 0                  # Destination memory offset

    # Use ECALL_CODECOPY (or similar mechanism provided by the environment)
    # This ecall might not exist; alternatively, load instructions could fetch code bytes.
    # Assuming CODECOPY exists:
    mv a0, t3                 # mem_offset = 0
    mv a1, t0                 # code_offset = start address of runtime code
    mv a2, t2                 # length
    li a7, ECALL_CODECOPY
    ecall                     # Copies runtime code to memory @ 0

    # Return the copied code
    mv a0, t3                 # mem_offset = 0
    mv a1, t2                 # length
    li a7, ECALL_RETURN
    ecall

    # End of initcode. Should not be reached after ECALL_RETURN.
    j _halt


# ==============================================================
# RUNTIME CODE SECTION
# This code is stored on the blockchain after successful deployment.
# It handles subsequent calls to the contract.
# ==============================================================
runtime_code_start:
    # --- Runtime Entry Point & Dispatcher ---
    # Read function selector (first 4 bytes of calldata)
    li a0, 0                  # offset = 0
    li a7, ECALL_CALLDATALOAD
    ecall                     # Returns first 32 bytes in a0 (hi), a1 (lo)
                              # We only need the first 4 bytes from a1 (assuming little-endian).
    andi t0, a1, 0xFFFFFFFF   # t0 = function selector (lower 32 bits sufficient if selectors fit)

    # Simple dispatcher using comparisons (replace with actual selectors)
    li t1, 0xa9059cbb         # Selector for transfer(address,uint256)
    beq t0, t1, _transfer

    li t1, 0x095ea7b3         # Selector for approve(address,uint256)
    beq t0, t1, _approve

    li t1, 0x23b872dd         # Selector for transferFrom(address,address,uint256)
    beq t0, t1, _transferFrom

    li t1, 0x70a08231         # Selector for balanceOf(address)
    beq t0, t1, _balanceOf

    li t1, 0x18160ddd         # Selector for totalSupply()
    beq t0, t1, _totalSupply

    li t1, 0x06fdde03         # Selector for name()
    beq t0, t1, _name

    li t1, 0x95d89b41         # Selector for symbol()
    beq t0, t1, _symbol

    li t1, 0x313ce567         # Selector for decimals()
    beq t0, t1, _decimals

    # Fallback: If no function matches, revert.
    j _revert_default

# --- Function Implementations ---

_transfer:
    # Args: address _to (offset 4), uint256 _value (offset 36)
    # 1. Load arguments
    # 2. Get caller (msg.sender)
    # 3. Check non-zero _to address
    # 4. Check sender balance >= _value
    # 5. Update balances (SLOAD, subtract, SSTORE; SLOAD, add, SSTORE)
    # 6. Emit Transfer event
    # 7. Return true

    # Simplified implementation - Detailed steps omitted for brevity
    # ... load args via ECALL_CALLDATALOAD ...
    # ... get caller via ECALL_CALLER ...
    # ... calculate sender balance slot (hash(SLOT_BALANCES_BASE, caller)) ...
    # ... calculate receiver balance slot (hash(SLOT_BALANCES_BASE, to)) ...
    # ... ECALL_SLOAD sender balance ...
    # ... Check balance >= value (requires 256-bit comparison) -> jump to _revert on failure ...
    # ... Check _to != address(0) -> jump to _revert on failure ...
    # ... Subtract value from sender balance (256-bit arithmetic) ...
    # ... ECALL_SSTORE updated sender balance ...
    # ... ECALL_SLOAD receiver balance ...
    # ... Add value to receiver balance (256-bit arithmetic) ...
    # ... ECALL_SSTORE updated receiver balance ...
    # ... Emit Transfer event via ECALL_LOG3 ...
    # ... Prepare return value (true = 1) in memory ...
    # ... ECALL_RETURN ...
    j _return_true # Placeholder jump

_approve:
    # Args: address _spender (offset 4), uint256 _value (offset 36)
    # 1. Load arguments
    # 2. Get caller (msg.sender)
    # 3. Check non-zero _spender address
    # 4. Calculate allowance slot (hash(SLOT_ALLOWANCES_BASE, caller, spender))
    # 5. Store _value in allowance slot (SSTORE)
    # 6. Emit Approval event
    # 7. Return true

    # Simplified implementation - Detailed steps omitted for brevity
    # ... load args ...
    # ... get caller ...
    # ... check spender != 0 -> _revert ...
    # ... calculate allowance slot (requires 2-level mapping logic/hashing) ...
    # ... ECALL_SSTORE allowance value ...
    # ... Emit Approval event via ECALL_LOG3 ...
    # ... prepare return true ...
    # ... ECALL_RETURN ...
    j _return_true # Placeholder jump


_transferFrom:
    # Args: address _from (offset 4), address _to (offset 36), uint256 _value (offset 68)
    # 1. Load arguments
    # 2. Get caller (msg.sender)
    # 3. Check non-zero _from, _to addresses
    # 4. Check _from balance >= _value
    # 5. Check allowance[from][msg.sender] >= _value
    # 6. Decrease allowance
    # 7. Update balances (_from -= value, _to += value)
    # 8. Emit Transfer event
    # 9. Return true

    # Simplified implementation - Detailed steps omitted for brevity
    # ... load args ...
    # ... get caller ...
    # ... check from/to != 0 -> _revert ...
    # ... calculate from balance slot ...
    # ... ECALL_SLOAD from_balance ...
    # ... check from_balance >= value -> _revert ...
    # ... calculate allowance slot (hash(SLOT_ALLOWANCES_BASE, from, caller)) ...
    # ... ECALL_SLOAD current_allowance ...
    # ... check current_allowance >= value -> _revert ...
    # ... Subtract value from allowance (256-bit) ...
    # ... ECALL_SSTORE updated allowance ...
    # ... Subtract value from from_balance (256-bit) ...
    # ... ECALL_SSTORE updated from_balance ...
    # ... calculate to balance slot ...
    # ... ECALL_SLOAD to_balance ...
    # ... Add value to to_balance (256-bit) ...
    # ... ECALL_SSTORE updated to_balance ...
    # ... Emit Transfer event ...
    # ... prepare return true ...
    # ... ECALL_RETURN ...
    j _return_true # Placeholder jump

_balanceOf:
    # Args: address _owner (offset 4)
    # 1. Load argument (_owner)
    # 2. Calculate balance slot (hash(SLOT_BALANCES_BASE, owner))
    # 3. Load balance (SLOAD)
    # 4. Prepare return value (balance) in memory
    # 5. Return balance

    # Simplified implementation
    li a0, 4 # offset for _owner arg
    li a7, ECALL_CALLDATALOAD
    ecall    # a0=owner_hi, a1=owner_lo (only need lower 20 bytes usually)

    # Calculate slot (Simplified: base + owner_lo)
    li t0, SLOT_BALANCES_BASE
    add t1, a1, t0 # slot_lo
    li t2, 0       # slot_hi

    # Load balance
    mv a0, t2
    mv a1, t1
    li a7, ECALL_SLOAD
    ecall    # Returns balance in a0 (hi), a1 (lo)

    # Prepare return buffer (e.g., on stack)
    addi sp, sp, -8
    sw a0, 0(sp) # store balance_hi
    sw a1, 4(sp) # store balance_lo

    # Return
    mv a0, sp  # mem_offset
    li a1, 8   # length (uint256)
    li a7, ECALL_RETURN
    ecall

    addi sp, sp, 8 # Clean up stack
    j _halt        # End execution for this call

_totalSupply:
    # 1. Load totalSupply from SLOT_TOTALSUPPLY
    # 2. Prepare return value
    # 3. Return

    li t0, SLOT_TOTALSUPPLY
    li t1, 0 # slot_hi
    mv a0, t1
    mv a1, t0
    li a7, ECALL_SLOAD
    ecall    # Returns totalSupply in a0 (hi), a1 (lo)

    addi sp, sp, -8
    sw a0, 0(sp) # store hi
    sw a1, 4(sp) # store lo

    mv a0, sp
    li a1, 8
    li a7, ECALL_RETURN
    ecall

    addi sp, sp, 8
    j _halt

_name:
_symbol:
_decimals:
    # These would load the corresponding value from storage (SLOT_NAME, SLOT_SYMBOL, SLOT_DECIMALS)
    # For name/symbol, returning dynamic strings requires careful ABI handling (pointer + length).
    # For decimals, load the uint8 value.
    # Simplified: Assume they return fixed values or revert.
    j _revert_default # Placeholder

# --- Helper Routines ---

_return_true:
    # Prepare return value 'true' (uint256(1))
    addi sp, sp, -8
    li t0, 0
    li t1, 1
    sw t0, 0(sp) # hi = 0
    sw t1, 4(sp) # lo = 1

    mv a0, sp
    li a1, 8
    li a7, ECALL_RETURN
    ecall

    addi sp, sp, 8
    j _halt

_revert_default:
    # Revert without error data
    li a0, 0
    li a1, 0
    li a7, ECALL_REVERT
    ecall
    j _halt # Should not be reached

# Generic halt/end sequence for successful calls that don't explicitly return
_halt:
    # In some environments, execution might just end.
    # In others, an explicit signal might be needed.
    # For simulation, just loop infinitely or use a break instruction if available.
    ebreak # Or use an environment specific halt


runtime_code_end: # Label to mark the end of the runtime code section

# --- Data Section (Optional, if needed for constants) ---
# .section .data
# my_string: .string "Hello"