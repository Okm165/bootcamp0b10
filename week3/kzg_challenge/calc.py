from sympy import mod_inverse, isprime, primitive_root

# The prime modulus r
r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001

# Check if r is prime (it should be)
assert isprime(r)

# Find a primitive root of the field
g = primitive_root(r)

# Calculate the 64nd root of unity
omega = pow(g, (r - 1) // 64, r)

print(f"Primitive root g: {g}")
print(f"64nd root of unity omega: {hex(omega)}")

# Verify that omega^64 == 1 mod r
assert pow(omega, 64, r) == 1
print("Verification passed: omega^64 == 1 mod r")

for i in range(0, 65):
    print(hex(pow(omega, i, r)))
